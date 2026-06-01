#[derive(thiserror::Error, Debug)]
pub enum DesError<M> {
    #[error("event processing failed")]
    EventProcessFailure(M),
}
