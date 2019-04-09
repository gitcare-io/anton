use crate::application::event::helpers;
use crate::infrastructure::models::event_store::event::EventQueryable;
use crate::infrastructure::models::read::dmr_projection::DMRProjectionInsertable;
use crate::infrastructure::repository::dmr_projection_repository::DMRProjectionRepository;
use crate::infrastructure::repository::event_repository::EventRepository;
use chrono::NaiveDateTime;
#[cfg(test)] use crate::infrastructure::models::event_store::event::EventInsertable;
#[cfg(test)] use chrono::Utc;
#[cfg(test)] use diesel::QueryResult;
use serde_json::json;

// DMR - Daily Merge Rate

pub struct DMRProjection<'a, 'b, ER, DMRR>
where
    ER: EventRepository + 'a,
    DMRR: DMRProjectionRepository + 'b,
{
    event_repository: &'a ER,
    dmr_projection_repository: &'b DMRR,
    event_type: &'static str,
    body: DMRProjectionBody,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DMRProjectionBody {
    id: String,
    repo_id: u64,
    from: NaiveDateTime,
    to: NaiveDateTime,
    data: DMRProjectionData,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DMRProjectionData {
    pub target: f32,
    pub value: f32,
    pub index: f32,
}

pub struct DMRProjectionIdentity {
    pub repo_id: u64,
}

impl<'a, 'b, ER, DMRR> DMRProjection<'a, 'b, ER, DMRR>
where
    ER: EventRepository + 'a,
    DMRR: DMRProjectionRepository + 'b,
{
    pub fn new(
        event_repository: &'a ER,
        dmr_projection_repository: &'b DMRR,
        identity: DMRProjectionIdentity,
        timezone: String,
        target: f32,
    ) -> Self {
        let from = helpers::today_midnight(&timezone);
        let to = helpers::tomorrow_midnight(&timezone);

        if target <= 0.0 {
            panic!("DMR projection: Target is invalid: {}", target);
        }

        DMRProjection {
            event_repository,
            dmr_projection_repository,
            event_type: "pull_request_closed",
            body: DMRProjectionBody {
                id: format!(
                    "{}_{}_{}",
                    identity.repo_id,
                    from.timestamp(),
                    to.timestamp(),
                ),
                repo_id: identity.repo_id,
                from,
                to,
                data: DMRProjectionData {
                    target,
                    value: 0_f32,
                    index: 0_f32,
                },
            },
        }
    }

    pub fn generate(mut self) -> Self {
        let events = self.get_events();
        self.body.data = events.iter().fold(self.body.data, |mut acc, i| {
            if i.data["pull_request"]["merged"] == true {
                acc.value += 1_f32;
                acc.index = acc.value / acc.target;
            }
            acc
        });
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
            .unwrap()
    }

    // private

    fn get_events(&self) -> Vec<EventQueryable> {
        self.event_repository
            .find_by_repo_and_type(
                self.body.repo_id,
                self.event_type,
                self.body.from,
                self.body.to,
            )
            .expect("DMR projection: cannot find events")
    }
}

// TESTS

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let event_repository = FakeEventRepository::new();
        let dmr_projection_repository = FakeDMRProjectionRepository::new();
        let timezone = timestamp_factory();
        let repo_id = 10_u64;
        let dmr_projection = DMRProjection::new(
            &event_repository,
            &dmr_projection_repository,
            DMRProjectionIdentity { repo_id },
            timezone.clone(),
            10_f32,
        );
        assert_eq!(
            format!(
                "{}_{}_{}",
                repo_id,
                helpers::today_midnight(&timezone).timestamp(),
                helpers::tomorrow_midnight(&timezone).timestamp()
            ),
            dmr_projection.body.id
        );
    }

    #[test]
    fn generate() {
        let event_repository = FakeEventRepository::new();
        let dmr_projection_repository = FakeDMRProjectionRepository::new();
        let repo_id = 10_u64;
        let target = 11_f32;
        let dmr_projection = DMRProjection::new(
            &event_repository,
            &dmr_projection_repository,
            DMRProjectionIdentity { repo_id },
            timestamp_factory(),
            target,
        );

        assert_eq!(
            dmr_projection.generate().body.data,
            DMRProjectionData {
                target: 11.0,
                value: 2.0,
                index: 2_f32 / target
            }
        )
    }

    #[test]
    #[should_panic]
    fn generate_with_invalid_target() {
        let event_repository = FakeEventRepository::new();
        let dmr_projection_repository = FakeDMRProjectionRepository::new();
        let repo_id = 10_u64;
        let target = 0_f32;
        DMRProjection::new(
            &event_repository,
            &dmr_projection_repository,
            DMRProjectionIdentity { repo_id },
            timestamp_factory(),
            target,
        );
    }

    fn timestamp_factory() -> String { String::from("Europe/Warsaw") }

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
            type_: String::from(event_type),
            meta: serde_json::from_str(meta).unwrap(),
            log_date: Utc::now().naive_utc(),
        }
    }

    struct FakeEventRepository;
    struct FakeDMRProjectionRepository;

    impl EventRepository for FakeEventRepository {
        fn new() -> Self { FakeEventRepository {} }

        fn persist_event(&self, _event: EventInsertable) -> QueryResult<usize> { Ok(1_usize) }

        fn find_by_repo_and_type(
            &self,
            _repo_id: u64,
            _event_type: &'static str,
            _from: NaiveDateTime,
            _to: NaiveDateTime,
        ) -> QueryResult<Vec<EventQueryable>> {
            let event1 = event_factory(
                10,
                "{ \"pull_request\": { \"merged\": true } }",
                "pull_request_closed",
                "{}",
            );
            let event2 = event_factory(
                10,
                "{ \"pull_request\": { \"merged\": false } }",
                "pull_request_closed",
                "{}",
            );
            Ok(vec![event1.clone(), event1.clone(), event2.clone()])
        }
    }

    impl DMRProjectionRepository for FakeDMRProjectionRepository {
        fn new() -> Self { FakeDMRProjectionRepository {} }

        fn persist_dmr(&self, _dmr_projection: DMRProjectionInsertable) -> QueryResult<usize> {
            Ok(1_usize)
        }
    }
}
