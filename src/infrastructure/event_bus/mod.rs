use eventbus::EventBus as EB;

lazy_static! {
    pub static ref EVENT_BUS: EB = EB::new();
}
