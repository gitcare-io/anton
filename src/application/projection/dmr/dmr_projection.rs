use crate::application::event::PULL_REQUEST_CLOSED;
use crate::infrastructure::models::event_store::event::EventQueryable;
use crate::infrastructure::models::read::dmr_projection::{
    DMRProjectionInsertable, DMRProjectionQueryable,
};
use crate::infrastructure::repository::dmr_projection_repository::DMRProjectionRepository;
use crate::infrastructure::repository::event_repository::EventRepository;
use chrono::NaiveDateTime;
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use serde_json::json;

// DMR - Daily Merge Rate

pub struct DMRProjection<'a, 'b, 'c, ER, DMRR>
where
    ER: EventRepository + 'a,
    DMRR: DMRProjectionRepository + 'b,
{
    pub event_repository: &'a ER,
    pub dmr_projection_repository: &'b DMRR,
    pub event_type: &'static str,
    pub event: &'c EventQueryable,
    pub body: DMRProjectionBody,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DMRProjectionBody {
    pub id: String,
    pub repo_id: u64,
    pub from: NaiveDateTime,
    pub to: NaiveDateTime,
    pub data: DMRProjectionData,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct DMRProjectionData {
    pub target: f32,
    pub team_size: u64,
    pub avg: DMRProjectionAvgValue,
    pub users: Vec<DMRProjectionUserValue>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct DMRProjectionAvgValue {
    pub value: f32,
    pub index: f32,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct DMRProjectionUserValue {
    pub user_id: u64,
    pub value: f32,
    pub index: f32,
}

impl<'a, 'b, 'c, ER, DMRR> DMRProjection<'a, 'b, 'c, ER, DMRR>
where
    ER: EventRepository + 'a,
    DMRR: DMRProjectionRepository + 'b,
{
    pub fn new(
        event_repository: &'a ER,
        dmr_projection_repository: &'b DMRR,
        event: &'c EventQueryable,
        repo_id: u64,
        target: f32,
        team_size: u64,
        from: NaiveDateTime,
        to: NaiveDateTime,
    ) -> Self {
        if target <= 0.0 {
            panic!("DMR projection: Target is invalid: {}", target);
        }

        Self {
            event_repository,
            dmr_projection_repository,
            event_type: PULL_REQUEST_CLOSED,
            event,
            body: DMRProjectionBody {
                id: Self::gen_key(repo_id, from, to),
                repo_id,
                from,
                to,
                data: DMRProjectionData {
                    target,
                    team_size,
                    avg: DMRProjectionAvgValue {
                        value: 0.0,
                        index: 0.0,
                    },
                    users: vec![],
                },
            },
        }
    }

    pub fn generate(mut self) -> Self {
        if self.event.data["pull_request"]["merged"] == false {
            return self;
        }

        let last_dmr_projection = self.dmr_projection_repository.find_one(
            self.body.repo_id as i64,
            self.body.from,
            self.body.to,
        );

        match last_dmr_projection {
            Ok(last_dmr) => {
                self.calculate_dmr_avg(Some(last_dmr.clone()));
                self.calculate_dmr_users(Some(last_dmr.clone()));
            }
            _ => {
                self.calculate_dmr_avg(None);
                self.calculate_dmr_users(None);
            }
        }

        self
    }

    pub fn persist(&self) -> usize {
        self.dmr_projection_repository
            .persist_dmr(DMRProjectionInsertable::new(
                self.body.id.clone(),
                self.body.repo_id as i64,
                self.body.from,
                self.body.to,
                json!(self.body.data),
            ))
            .expect("DMR projection: cannot persist")
    }

    // private

    pub fn calculate_dmr_avg(&mut self, last_dmr: Option<DMRProjectionQueryable>) {
        match last_dmr {
            Some(last_dmr) => {
                let last_dmr_data: DMRProjectionData =
                    serde_json::from_value(last_dmr.data).unwrap();
                self.body.data.avg.value = last_dmr_data.avg.value + 1.0;
                self.body.data.avg.index = (last_dmr_data.avg.value + 1.0)
                    / (self.body.data.target * self.body.data.team_size as f32);
            }
            None => {
                self.body.data.avg.value = 1.0;
                self.body.data.avg.index =
                    1.0 / (self.body.data.target * self.body.data.team_size as f32);
            }
        }
    }

    pub fn calculate_dmr_users(&mut self, last_dmr: Option<DMRProjectionQueryable>) {
        let user_id = self.event.meta["user_id"]
            .as_u64()
            .expect("DMR Projection: cannot convert event.meta.user_id to u64");
        match last_dmr {
            Some(last_dmr) => {
                let last_dmr_data: DMRProjectionData =
                    serde_json::from_value(last_dmr.data).unwrap();
                let is_not_user_included = last_dmr_data
                    .users
                    .iter()
                    .find(|&x| x.user_id == user_id)
                    .is_none();

                if is_not_user_included {
                    self.body.data.users = last_dmr_data.users.clone();
                    self.body.data.users.push(DMRProjectionUserValue {
                        user_id,
                        value: 1.0,
                        index: 1.0 / self.body.data.target,
                    });
                } else {
                    self.body.data.users = last_dmr_data
                        .users
                        .iter()
                        .map(|item| {
                            let mut value = item.value.clone();
                            let mut index = item.index.clone();
                            if item.user_id == user_id {
                                value += 1.0;
                                index = value / self.body.data.target;
                            }
                            DMRProjectionUserValue {
                                user_id: item.user_id,
                                value,
                                index,
                            }
                        })
                        .collect::<Vec<DMRProjectionUserValue>>();
                }
            }
            None => self.body.data.users.push(DMRProjectionUserValue {
                user_id,
                value: 1.0,
                index: 1.0 / self.body.data.target,
            }),
        }
    }

    fn gen_key(repo_id: u64, from: NaiveDateTime, to: NaiveDateTime) -> String {
        let mut hasher = Sha1::new();
        hasher.input_str(&format!("{}_{}_{}", repo_id, from.timestamp(), to.timestamp())[..]);
        hasher.result_str()
    }
}

// TESTS

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::projection::helpers;
    use crate::infrastructure::models::event_store::event::EventInsertable;
    use crate::infrastructure::models::read::dmr_projection::DMRProjectionQueryable;
    use chrono::Utc;
    use diesel::QueryResult;

    #[test]
    fn new() {
        let event_repository = FakeEventRepository::new();
        let dmr_projection_repository = FakeDMRProjectionRepository::new();
        let timezone = timezone_factory();
        let repo_id = 10_u64;
        let event = event_repository.find_by_seq_num(1_i64).unwrap();
        let dmr_projection = DMRProjection::new(
            &event_repository,
            &dmr_projection_repository,
            &event,
            repo_id,
            10.0,
            2_u64,
            helpers::today_midnight(&timezone),
            helpers::tomorrow_midnight(&timezone),
        );
        let mut hasher = Sha1::new();
        hasher.input_str(
            &format!(
                "{}_{}_{}",
                repo_id,
                helpers::today_midnight(&timezone).timestamp(),
                helpers::tomorrow_midnight(&timezone).timestamp()
            )[..],
        );
        assert_eq!(hasher.result_str(), dmr_projection.body.id);
    }

    #[test]
    fn generate() {
        let event_repository = FakeEventRepository::new();
        let dmr_projection_repository = FakeDMRProjectionRepository::new();
        let repo_id = 10_u64;
        let target = 11.0;
        let team_size = 2_u64;
        let event = event_repository.find_by_seq_num(1_i64).unwrap();
        let timezone = timezone_factory();
        let dmr_projection = DMRProjection::new(
            &event_repository,
            &dmr_projection_repository,
            &event,
            repo_id,
            target,
            team_size,
            helpers::today_midnight(&timezone),
            helpers::tomorrow_midnight(&timezone),
        );

        assert_eq!(
            dmr_projection.generate().body.data,
            DMRProjectionData {
                target: 11.0,
                team_size: 2_u64,
                avg: DMRProjectionAvgValue {
                    value: 4.0,
                    index: 4.0 / (team_size as f32 * target)
                },
                users: vec![
                    DMRProjectionUserValue {
                        user_id: 1,
                        value: 2.0,
                        index: 2.0 / target,
                    },
                    DMRProjectionUserValue {
                        user_id: 2,
                        value: 2.0,
                        index: 2.0 / target,
                    }
                ]
            }
        )
    }

    #[test]
    #[should_panic]
    fn generate_with_invalid_target() {
        let event_repository = FakeEventRepository::new();
        let dmr_projection_repository = FakeDMRProjectionRepository::new();
        let repo_id = 10_u64;
        let event = event_repository.find_by_seq_num(1_i64).unwrap();
        let target = 0.0;
        let team_size = 2_u64;
        let timezone = timezone_factory();
        DMRProjection::new(
            &event_repository,
            &dmr_projection_repository,
            &event,
            repo_id,
            target,
            team_size,
            helpers::today_midnight(&timezone),
            helpers::tomorrow_midnight(&timezone),
        );
    }

    fn timezone_factory() -> String { String::from("Europe/Warsaw") }

    fn event_factory(
        agg_id: i64,
        data: &'static str,
        event_type: &'static str,
        meta: &'static str,
    ) -> EventQueryable {
        EventQueryable {
            seq_num: 1_i64,
            aggregate_id: agg_id,
            data: serde_json::from_str(data).unwrap(),
            event_type: String::from(event_type),
            meta: serde_json::from_str(meta).unwrap(),
            log_date: Utc::now().naive_utc(),
        }
    }

    struct FakeEventRepository;
    struct FakeDMRProjectionRepository;

    impl EventRepository for FakeEventRepository {
        fn new() -> Self { FakeEventRepository {} }

        fn persist_event(&self, _event: EventInsertable) -> QueryResult<(i64)> { Ok(1_i64) }

        fn find_by_seq_num(&self, _seq_n: i64) -> QueryResult<EventQueryable> {
            Ok(event_factory(
                10,
                r#"{
                    "pull_request": {
                        "merged": true
                    }
                }"#,
                PULL_REQUEST_CLOSED,
                r#"{ "user_id": 2 }"#,
            ))
        }

        fn find_all(&self, _limit: i64, _offset: i64) -> QueryResult<Vec<EventQueryable>> {
            Ok(vec![])
        }

        fn find_by_repo_and_type(
            &self,
            _repo_id: u64,
            _event_type: &'static str,
            _from: NaiveDateTime,
            _to: NaiveDateTime,
        ) -> QueryResult<Vec<EventQueryable>> {
            Ok(vec![])
        }
    }

    impl DMRProjectionRepository for FakeDMRProjectionRepository {
        fn new() -> Self { FakeDMRProjectionRepository {} }

        fn find(
            &self,
            _repo_id: i64,
            _from: NaiveDateTime,
            _to: NaiveDateTime,
        ) -> QueryResult<Vec<DMRProjectionQueryable>> {
            Ok(vec![])
        }

        fn find_one(
            &self,
            _repo_id: i64,
            _from: NaiveDateTime,
            _to: NaiveDateTime,
        ) -> QueryResult<DMRProjectionQueryable> {
            let projection_data = r#"
                {
                    "target": 11.0,
                    "team_size": 2,
                    "avg": {
                        "value": 3.0,
                        "index": 0.1363636364
                    },
                    "users": [
                        {
                            "user_id": 1,
                            "value": 2.0,
                            "index": 0.1818181818
                        },
                        {
                            "user_id": 2,
                            "value": 1.0,
                            "index": 0.09090909091
                        }
                    ]
                }
            "#;
            Ok(DMRProjectionQueryable {
                id: String::from(""),
                repo_id: 1_i64,
                from: helpers::today_midnight(&timezone_factory()),
                to: helpers::tomorrow_midnight(&timezone_factory()),
                projected_at: Utc::now().naive_utc(),
                data: serde_json::from_str(projection_data).unwrap(),
            })
        }

        fn persist_dmr(&self, _dmr_projection: DMRProjectionInsertable) -> QueryResult<usize> {
            Ok(1_usize)
        }
    }
}
