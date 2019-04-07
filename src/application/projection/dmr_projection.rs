use crate::infrastructure::models::read::event::Event;
use crate::infrastructure::models::write::event::Event as WriteEvent;
use crate::infrastructure::repository::event_repository::EventRepository;
use chrono::NaiveDateTime;
use chrono::Utc;
use diesel::QueryResult;
#[cfg(test)]

// DMR - Daily Merge Rate

pub struct DMRProjection<'a, ER>
where
    ER: EventRepository + 'a,
{
    _event_repo: &'a ER,
    body: DMRProjectionBody,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DMRProjectionBody {
    id: String,
    aggregate_id: i64,
    aggregate_type: String,
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
    aggregate_id: i64,
    aggregate_type: String,
    from: NaiveDateTime,
    to: NaiveDateTime,
}

impl<'a, ER> DMRProjection<'a, ER>
where
    ER: EventRepository + 'a,
{
    pub fn new(event_repository: &'a ER, identity: DMRProjectionIdentity) -> Self {
        DMRProjection {
            _event_repo: event_repository,
            body: DMRProjectionBody {
                id: format!(
                    "{}_{}_{}_{}",
                    identity.aggregate_id,
                    identity.from.timestamp(),
                    identity.to.timestamp(),
                    identity.aggregate_type,
                ),
                aggregate_id: identity.aggregate_id,
                aggregate_type: identity.aggregate_type,
                from: identity.from,
                to: identity.to,
                data: DMRProjectionData {
                    target: 10_f32, // FIXME: it should be configurable
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

    fn get_events(&self) -> Vec<Event> {
        self._event_repo
            .find_in_range(
                self.body.aggregate_id,
                "pull_request_closed",
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
        let event_repo = FakeEventRepository::new();
        let dmr_projection = DMRProjection::new(
            &event_repo,
            DMRProjectionIdentity {
                aggregate_id: 10,
                aggregate_type: String::from("user"),
                from: NaiveDateTime::from_timestamp(1554076800, 0),
                to: NaiveDateTime::from_timestamp(1556668800, 0),
            },
        );
        assert_eq!(
            String::from("10_1554076800_1556668800_user"),
            dmr_projection.body.id
        );
    }

    #[test]
    fn generate() {
        let event_repo = FakeEventRepository::new();
        let dmr_projection = DMRProjection::new(
            &event_repo,
            DMRProjectionIdentity {
                aggregate_id: 10,
                aggregate_type: String::from("user"),
                from: NaiveDateTime::from_timestamp(1554076800, 0),
                to: NaiveDateTime::from_timestamp(1556668800, 0),
            },
        );
        assert_eq!(
            dmr_projection.generate().body.data,
            DMRProjectionData {
                target: 10.0,
                value: 2.0,
                index: 2_f32 / 10_f32
            }
        )
    }

    fn event_factory(
        agg_id: i64,
        data: &'static str,
        event_type: &'static str,
        meta: &'static str,
    ) -> Event {
        Event {
            seq_num: 1_i64,
            aggregate_id: agg_id,
            data: serde_json::from_str(data).unwrap(),
            type_: String::from(event_type),
            meta: serde_json::from_str(meta).unwrap(),
            log_date: Utc::now().naive_utc(),
        }
    }

    struct FakeEventRepository;

    impl EventRepository for FakeEventRepository {
        fn new() -> Self { FakeEventRepository {} }

        fn add(&self, _event: WriteEvent) -> QueryResult<usize> { Ok(1_usize) }

        fn find_in_range(
            &self,
            _agg_id: i64,
            _event_type: &'static str,
            _from: NaiveDateTime,
            _to: NaiveDateTime,
        ) -> QueryResult<Vec<Event>> {
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
}
