use crate::Model;

#[derive(thiserror::Error, Debug)]
pub enum DesError<M: Model> {
    #[error("event processing failed")]
    EventProcessFailure(M::Error),

    #[error("missing logical process")]
    MissingLogicalProcess(M::LogicalProcessId),
}
