extern crate rand;
use std::f64;

pub fn random_f64_less_than(max: f64) -> f64 {
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
    pub r: f64,
    pub x: f64,
    pub y: f64,
    pub orb_type: OrbType,
    dir_rad: Option<f64>,
    speed: f64,
    max_rad_turn: f64,
}

pub enum OrbType {
    Homing,
    Roaming,
}

impl Orb {
    pub fn new(
        r: f64, height: u32, width: u32, speed: f64, dir_rad: Option<f64>
    ) -> Orb {
        let orb_type = match dir_rad {
            Some(_) => Homing,
            None => Roaming,
        };

        Orb {
            r: r,
            x: pos_neg() as f64 * random_f64_less_than((width/2) as f64),
            y: pos_neg() as f64 * random_f64_less_than((height/2) as f64),
            orb_type: orb_type,
            dir_rad: dir_rad,
            speed: speed,
            max_rad_turn: f64::consts::PI / random_f64_less_than(10.0),
        }
    }

    fn roaming_time_change(&mut self, dt: f64) {
    }

    fn homing_time_change(&mut self, p_x: f64, p_y: f64, dt: f64) {
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

    pub fn handle_time_change(&mut self, p_x: f64, p_y: f64, dt: f64) {
        match self.orb_type {
            OrbType::Homing => self.homing_time_change(p_x, p_y, dt),
            OrbType::Roaming => self.roaming_time_change(dt),
        }
    }
}
