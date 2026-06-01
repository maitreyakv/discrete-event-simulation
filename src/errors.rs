#[derive(thiserror::Error, Debug)]
pub enum DesError {
    #[error("event was scheduled in the past")]
    CausalityViolation,
}
