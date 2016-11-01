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

pub fn rotate_y(dir_rad: f64) -> f64 {
    let safe_rad = rad_0to2pi(dir_rad);
    let q1 = safe_rad >= 0.0 && safe_rad <= PI / 2.0;
    let q2 = safe_rad > PI / 2.0 && safe_rad <= PI;
    if q1 || q2 {
        PI - safe_rad
    } else {
        2.0 * PI - safe_rad
    }
}

///
/// Takes the center of a circle and a square, along with side length/radius
/// and comuptes whether or not the two overlap
///
pub fn square_circle_overlap(cx: f64, cy: f64, cr: f64, sx: f64, sy: f64, ss: f64)
-> bool {
    // Takes point and determines if it lies in the circle
    let point_in_circle = |px: f64, py: f64| -> bool {
        let dx = px - cx;
        let dy = py - cy;
        (dx * dx + dy * dy).sqrt() <= cr
    };


    let s_top = sy + ss / 2.0;
    let s_bot = sy - ss / 2.0;
    let s_rig = sx + ss / 2.0;
    let s_lef = sx - ss / 2.0;
    let point_in_square = |px: f64, py: f64| -> bool {
        px <= s_rig && px >= s_lef && py <= s_top && py >= s_bot
    };

    let c_top = cy + cr;
    let c_bot = cy - cr;
    let c_rig = cx + cr;
    let c_lef = cx - cr;
    // Check if circle points are in square
    point_in_square(cx, c_top) ||
    point_in_square(cx, c_bot) ||
    point_in_square(c_lef, cy) ||
    point_in_square(c_rig, cy) ||
    // Check if square corners are in circle
    point_in_circle(s_rig, s_bot) ||
    point_in_circle(s_lef, s_bot) ||
    point_in_circle(s_rig, s_top) ||
    point_in_circle(s_lef, s_top)
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
