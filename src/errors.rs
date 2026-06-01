use crate::scheduler::CausalityViolation;

#[derive(thiserror::Error, Debug)]
pub enum DesError<M> {
    #[error("event was scheduled in the past")]
    CausalityViolation(#[from] CausalityViolation),

    #[error("event processing failed")]
    EventProcessFailure(M),
}
