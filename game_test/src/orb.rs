use std::f64::consts::PI;
use utils::{
    random_f64_less_than,
    pos_neg,
    sign,
    rad_0to2pi,
    rotate_y,
};

pub struct Orb {
    pub r: f64,
    pub x: f64,
    pub y: f64,
    pub orb_type: OrbType,
    dir_rad: f64,
    speed: f64,
    max_rad_turn: f64,
}

pub enum OrbType {
    Homing,
    Roaming,
}

impl Orb {
    pub fn new(
        r: f64, height: u32, width: u32, speed: f64, orb_type: OrbType
    ) -> Orb {
        Orb {
            r: r,
            x: pos_neg() as f64 * random_f64_less_than((width/2) as f64),
            y: -random_f64_less_than((height/2) as f64),
            orb_type: orb_type,
            dir_rad: random_f64_less_than(2.0 * PI),
            speed: speed,
            max_rad_turn: PI / random_f64_less_than(10.0),
        }
    }

    pub fn roaming_time_change(&mut self, p_x: f64, p_y: f64, dt: f64) {
        let dir_rad = self.dir_rad;

        let vx = -self.speed * dir_rad.cos();
        let vy = -self.speed * dir_rad.sin();
        let new_x = self.x + vx * dt;
        let new_y = self.y + vy * dt;

        let get_dist_path = |x: f64, y: f64| -> f64 {
            let dx = x - p_x;
            let dy = y - p_y;
            (dx * dir_rad.cos() + dy * dir_rad.sin()).abs()
        };

        let old_dist_path = get_dist_path(self.x, self.y);
        let new_dist_path = get_dist_path(new_x, new_y);

        if new_y > 0.0 {
            self.dir_rad = rotate_y(dir_rad);
        } else if new_dist_path > 300.0 && new_dist_path > old_dist_path {
            self.dir_rad = rad_0to2pi(PI + self.dir_rad);
        } else {
            self.x = new_x;
            self.y = new_y;
        }
    }

    pub fn homing_time_change(&mut self, p_x: f64, p_y: f64, dt: f64) {
        // Strange; only works when i set the speed to be negative
        let dir_rad = self.dir_rad;
        let vx = -self.speed * dir_rad.cos();
        let vy = -self.speed * dir_rad.sin();

        let new_x = self.x + vx * dt;
        let new_y = self.y + vy * dt;

        let dx = p_x - self.x;
        let dy = p_y - self.y;

        let dir_ideal = dy.atan2(dx) + PI;

        let turn_amt = dir_ideal - dir_rad;

        let new_dir_rad = if turn_amt.abs() > self.max_rad_turn {
            rad_0to2pi(sign(turn_amt) * self.max_rad_turn + dir_rad)
        } else {
            dir_ideal
        };

        if new_y > 0.0 {
            self.dir_rad = rotate_y(dir_rad);
        } else {
            self.x = new_x;
            self.y = new_y;
            self.dir_rad = new_dir_rad;
        }
    }

    pub fn handle_time_change(&mut self, p_x: f64, p_y: f64, dt: f64) {
        match self.orb_type {
            OrbType::Homing => self.homing_time_change(p_x, p_y, dt),
            OrbType::Roaming => self.roaming_time_change(p_x, p_y, dt),
        }
    }
}
