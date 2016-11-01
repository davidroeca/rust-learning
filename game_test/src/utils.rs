use std::io::{Error, ErrorKind};
use std::env;
use std::path::PathBuf;
use std::f64::consts::PI;

extern crate rand;

pub fn random_f64_less_than(max: f64) -> f64 {
    let rand_num = rand::random::<f64>();
    max * rand_num
}

pub fn pos_neg() -> i32 {
    if rand::random::<f64>() < 0.5 {
        -1
    } else {
        1
    }
}

pub fn rad_0to2pi(rad: f64) -> f64 {
    let mut result = rad;
    let two_pi = 2.0 * PI;
    while result > two_pi {
        result -= two_pi;
    }
    while result < 0.0 {
        result += two_pi;
    }
    result
}

pub fn sign(n: f64) -> f64 {
    if n >= 0.0 {
        1.0
    } else {
        -1.0
    }
}

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
