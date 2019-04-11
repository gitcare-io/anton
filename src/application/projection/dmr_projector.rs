use crate::application::projection::dmr_projection::{DMRProjection, DMRProjectionIdentity};
use crate::application::projection::helpers;
use crate::application::projection::projector::Projector;
use crate::infrastructure::models::event_store::event::EventQueryable;
use crate::infrastructure::repository::dmr_projection_repository::DMRProjectionRepository;
use crate::infrastructure::repository::event_repository::EventRepository;
use chrono::{Duration, NaiveDateTime};

pub struct DMRProjector<'a, 'b, ER, DMRR>
where
    ER: EventRepository + 'a,
    DMRR: DMRProjectionRepository + 'b,
{
    event_repository: &'a ER,
    dmr_projection_repository: &'b DMRR,
    timezone: String, // TODO: it should be configurable for each repo.
    target: f32,      // TODO: it should be configurable for each repo.
}

impl<'a, 'b, ER, DMRR> Projector<'a, 'b, ER, DMRR> for DMRProjector<'a, 'b, ER, DMRR>
where
    ER: EventRepository + 'a,
    DMRR: DMRProjectionRepository + 'b,
{
    fn new(event_repository: &'a ER, dmr_projection_repository: &'b DMRR) -> Self {
        DMRProjector {
            event_repository,
            dmr_projection_repository,
            timezone: String::from("Europe/Warsaw"),
            target: 10_f32,
        }
    }

    fn project(&self, seq_num: i64) -> Result<(), &'static str> {
        let event = self.get_event(seq_num);
        let repo_id = self.get_repo_id(&event);

        DMRProjection::new(
            self.event_repository,
            self.dmr_projection_repository,
            DMRProjectionIdentity { repo_id },
            self.target,
            self.get_datetime(&event, "from"),
            self.get_datetime(&event, "to"),
        )
        .generate()
        .persist();

        Ok(())
    }
}

impl<'a, 'b, ER, DMRR> DMRProjector<'a, 'b, ER, DMRR>
where
    ER: EventRepository + 'a,
    DMRR: DMRProjectionRepository + 'b,
{
    fn get_event(&self, seq_num: i64) -> EventQueryable {
        self.event_repository
            .find_by_seq_num(seq_num)
            .expect("DMR Projector: cannot find event by seq_num")
    }

    fn get_repo_id(&self, event: &EventQueryable) -> u64 {
        event.meta["repo_id"]
            .as_u64()
            .expect("DMR Projector: cannot access repo_id in event.meta")
    }

    fn get_datetime(&self, event: &EventQueryable, date_type: &'static str) -> NaiveDateTime {
        match date_type {
            "to" => helpers::date_midnight(&event.log_date, &self.timezone) + Duration::hours(24),
            _ => helpers::date_midnight(&event.log_date, &self.timezone),
        }
    }
}
