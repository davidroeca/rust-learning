use std::io::{Error, ErrorKind};
use std::env;
use std::path::PathBuf;

///
/// Provides the full path of the project's directory
///
pub fn get_project_dir() -> Result<PathBuf, Error>{
    let exe_path = try!(env::current_exe());
    if let Some(run_dir) = exe_path.parent() {
        if let Some(target_dir) = run_dir.parent() {
            if let Some(project_dir) = target_dir.parent() {
                return Ok(project_dir.join(""));
            }
        }
    }
    Err(Error::new(ErrorKind::Other, "target dir not found"))
}
