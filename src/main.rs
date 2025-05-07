use std::env;

pub mod project;
pub mod error;

use project::*;
use error::*;

fn main() -> Result<(), Error> {
    env_logger::init();
    let project = Project::discover(env::current_dir()?)?;
    project.do_build(env::args())?;
    Ok(())
}
