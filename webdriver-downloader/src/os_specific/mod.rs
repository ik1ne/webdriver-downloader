pub mod chromedriver;
pub mod geckodriver;

#[derive(thiserror::Error, Debug)]
pub enum DefaultPathError {
    #[error("Failed to get home directory")]
    HomeDir,
    #[error("Failed to get Program Files directory")]
    ProgramFiles(#[from] std::env::VarError),
    #[error("Failed to run command")]
    Which(#[from] which::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
