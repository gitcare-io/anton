pub trait Projector<'a, 'b, ER, SR> {
    fn new(event_repository: &'a ER, self_repository: &'b SR) -> Self;
    fn project(&self, event_id: i64) -> Result<(), &'static str>;
}