use std::env::Args;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::os::unix::process::CommandExt;

use log::*;

use crate::error::Error;

#[allow(dead_code)]
pub struct Project {
    topdir: PathBuf,
    builddir: PathBuf,
}

impl Project {
    pub fn discover(cwd: PathBuf) -> Result<Project, Error> {
        for dirp in cwd.ancestors() {
            debug!("looking at {}", dirp.display());
            if is_cmake_builddir(dirp)? {
                debug!("you are inside a cmake build dir {}", dirp.display());
                return Ok(Project {
                    topdir: dirp.parent().expect("parent dir").to_path_buf(),
                    builddir: dirp.to_owned(),
                });
            }

            if is_git_repo_root(dirp)? {
                debug!("found git repo at {}", dirp.display());
                for sub in std::fs::read_dir(dirp)? {
                    let sub = sub?;
                    let sub = sub.path();
                    if sub.is_dir() && is_cmake_builddir(&sub)? {
                        debug!("detected builddir at {}", sub.display());
                        return Ok(Project {
                            topdir: dirp.to_path_buf(),
                            builddir: sub,
                        })
                    }
                }
                return Err(Error::MysteriousProjectType(dirp.to_path_buf()));
            }
        }
        Err(Error::MysteriousProjectRootError(cwd))
    }

    /// Transfer the args to `cmake --build`.
    pub fn do_build(&self, args: Args) -> Result<(), Error> {
        let is_ninja = self.builddir.join("build.ninja").exists();
        let is_makefile = self.builddir.join("Makefile").exists();
        let build_cmd = if is_ninja {
            "ninja"
        } else if is_makefile {
            "make"
        } else {
            "cmake"
        };
        let mut build_cmd = Command::new(build_cmd);
        if is_ninja {
            build_cmd.arg("-C").arg(&self.builddir);
        } else if is_makefile {
            build_cmd.arg("-C").arg(&self.builddir);
        } else {
            build_cmd.arg("--build").arg(&self.builddir);
        }
        let err = build_cmd
            .args(args.skip(1))
            .exec();
        Err(err.into())
    }
}

fn is_cmake_builddir(dirp: &Path) -> Result<bool, Error> {
    let testfile = dirp.join("CMakeCache.txt");
    Ok(testfile.exists())
}

fn is_git_repo_root(dirp: &Path) -> Result<bool, Error> {
    let testdir = dirp.join(".git");
    Ok(testdir.is_dir())
}
