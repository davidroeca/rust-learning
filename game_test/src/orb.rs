extern crate rand;
use std::f64;

fn random_f64_less_than(max: f64) -> f64 {
    let rand_num = rand::random::<f64>();
    max * rand_num
}

fn pos_neg() -> i32 {
    if rand::random::<f64>() < 0.5 {
        -1
    } else {
        1
    }
}

fn rad_0to2pi(rad: f64) -> f64 {
    let mut result = rad;
    let two_pi = 2.0 * f64::consts::PI;
    while result > two_pi {
        result -= two_pi;
    }
    while result < 0.0 {
        result += two_pi;
    }
    result
}

fn sign(n: f64) -> f64 {
    if n >= 0.0 {
        1.0
    } else {
        -1.0
    }
}

pub struct Orb {
    pub state: OrbState,
    pub r: f64,
}

pub struct OrbState {
    pub x: f64,
    pub y: f64,
    dir_rad: f64,
    speed: f64,
    max_rad_turn: f64,
}

impl OrbState {
    pub fn new(height: u32, width: u32, speed: f64) -> OrbState {
        OrbState {
            x: pos_neg() as f64 * random_f64_less_than((width/2) as f64),
            y: pos_neg() as f64 * random_f64_less_than((height/2) as f64),
            dir_rad: random_f64_less_than(2.0 * f64::consts::PI),
            speed: speed,
            max_rad_turn: f64::consts::PI / 6.0,
        }
    }

    pub fn handle_time_change(&mut self, p_x: f64, p_y: f64, dt: f64) {

        // Strange; only works when i set the speed to be negative
        let vx = -self.speed * self.dir_rad.cos();
        let vy = -self.speed * self.dir_rad.sin();

        let new_x = self.x + vx * dt;
        let new_y = self.y + vy * dt;

        let dx = p_x - self.x;
        let dy = p_y - self.y;

        let dir_ideal = dy.atan2(dx) + f64::consts::PI;

        let turn_amt = dir_ideal - self.dir_rad;

        let new_dir_rad = if turn_amt.abs() > self.max_rad_turn {
            rad_0to2pi(sign(turn_amt) * self.max_rad_turn + self.dir_rad)
        } else {
            dir_ideal
        };

        self.x = new_x;
        self.y = new_y;
        self.dir_rad = new_dir_rad;
    }
}
