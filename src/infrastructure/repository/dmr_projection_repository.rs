use crate::infrastructure::schema::read_schema::dmrprojections::dsl::*;
use crate::infrastructure::{
    models::read::dmr_projection::DMRProjectionInsertable,
    repository::repository::{CommonRepository, Repository, __construct},
};
use chrono::Utc;
#[allow(unused_imports)]
use diesel::Connection;
use diesel::ExpressionMethods;
use diesel::{QueryResult, RunQueryDsl};

pub trait DMRProjectionRepository {
    fn new() -> Self;
    fn persist_dmr(&self, dmr_projection: DMRProjectionInsertable) -> QueryResult<usize>;
}

impl DMRProjectionRepository for Repository {
    fn new() -> Self { __construct("read") }

    fn persist_dmr(&self, dmr_projection: DMRProjectionInsertable) -> QueryResult<usize> {
        diesel::insert_into(dmrprojections)
            .values(&dmr_projection)
            .on_conflict(id)
            .do_update()
            .set((&dmr_projection, projected_at.eq(Utc::now().naive_utc())))
            .execute(self.conn())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::models::read::dmr_projection::DMRProjectionQueryable;
    use crate::load_config;
    use diesel::result::Error;

    #[test]
    fn persist_dmr_projection() {
        load_config();
        let dmr_projection_repository: Repository = DMRProjectionRepository::new();
        dmr_projection_repository
            .conn()
            .test_transaction::<_, Error, _>(|| {
                let dmr1 = DMRProjectionInsertable::new(
                    String::from("test"),
                    1_i64,
                    Utc::now().naive_utc(),
                    Utc::now().naive_utc(),
                    serde_json::from_str("{}").unwrap(),
                );
                let dmr2 = DMRProjectionInsertable::new(
                    String::from("test"),
                    1_i64,
                    Utc::now().naive_utc(),
                    Utc::now().naive_utc(),
                    serde_json::from_str("{ \"test\": 1 }").unwrap(),
                );
                dmr_projection_repository.persist_dmr(dmr1)?;
                let results1 = dmrprojections
                    .load::<DMRProjectionQueryable>(dmr_projection_repository.conn())?;
                dmr_projection_repository.persist_dmr(dmr2)?;
                let results2 = dmrprojections
                    .load::<DMRProjectionQueryable>(dmr_projection_repository.conn())?;
                assert_ne!(
                    results1.first().unwrap().data,
                    results2.first().unwrap().data
                );
                Ok(())
            });
    }
}
