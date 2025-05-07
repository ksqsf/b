use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Input/Output Error: {}", .0)]
    IO(#[from] std::io::Error),

    #[error("Cannot determine project root from {}", .0.display())]
    MysteriousProjectRootError(PathBuf),

    #[error("Cannot determine project type for project at {}", .0.display())]
    MysteriousProjectType(PathBuf),
}
