/// Almanac errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    General(String),
}
