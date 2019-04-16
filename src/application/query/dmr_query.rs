use crate::infrastructure::models::read::dmr_projection::DMRProjectionQueryable;
use crate::infrastructure::repository::dmr_projection_repository::DMRProjectionRepository;
use chrono::NaiveDateTime;

pub struct DMRQuery<'a, DMRR>
where
    DMRR: DMRProjectionRepository + 'a,
{
    dmr_projection_repository: &'a DMRR,
    repo_id: i64,
    from: NaiveDateTime,
    to: NaiveDateTime,
}

impl<'a, DMRR> DMRQuery<'a, DMRR>
where
    DMRR: DMRProjectionRepository + 'a,
{
    pub fn new(
        dmr_projection_repository: &'a DMRR,
        repo_id: i64,
        from: NaiveDateTime,
        to: NaiveDateTime,
    ) -> Self {
        DMRQuery {
            dmr_projection_repository,
            repo_id,
            from,
            to,
        }
    }

    pub fn execute(&self) -> Vec<DMRProjectionQueryable> {
        self.dmr_projection_repository
            .find(self.repo_id, self.from, self.to)
            .expect("DMR Query: cannot find dmr projections")
    }
}
