use piston::input::{
    Button,
    Key,
};

pub enum XDir {
    Left,
    Right,
    Static,
}

pub struct PlayerState {
    pub x: f64,
    pub y: f64,
    y_vel: f64,
    x_speed: f64,
    x_boost: f64,
    x_dir: XDir,
    pub time_s: f64,
    left_pressed: bool,
    right_pressed: bool,
}

impl PlayerState {
    pub fn new(x_speed_in: f64) -> PlayerState {
        PlayerState {
            x: 0.0,
            y: 0.0,
            y_vel: 0.0,
            x_speed: x_speed_in,
            x_boost: 0.0,
            x_dir: XDir::Static,
            time_s: 0.0,
            left_pressed: false,
            right_pressed: false,
        }
    }

    pub fn handle_button(&mut self, button: Button) {
        match button {
            Button::Keyboard(Key::Up) => {
                self.jump();
            },
            Button::Keyboard(Key::Left) => {
                self.left_pressed = true;
                self.change_x_dir(XDir::Left);
                self.change_x_boost(0.0);
            },
            Button::Keyboard(Key::Right) => {
                self.right_pressed = true;
                self.change_x_dir(XDir::Right);
                self.change_x_boost(0.0);
            },
            Button::Keyboard(Key::Space) => {
                match self.x_dir {
                    XDir::Left | XDir::Right => self.boost(),
                    _ => (),
                }
            },
            _ => (),
        }
    }

    pub fn handle_release(&mut self, button: Button) {
        match button {
            Button::Keyboard(Key::Left) => {
                if !self.right_pressed {
                    self.change_x_dir(XDir::Static);
                }
                self.left_pressed = false;
            },
            Button::Keyboard(Key::Right) => {
                if !self.left_pressed {
                    self.change_x_dir(XDir::Static);
                }
                self.right_pressed = false;
            },
            _ => (),
        }
    }

    pub fn handle_time_change(&mut self, dt: f64) {
        // Update the time variable
        self.time_s += dt;
        // Basic kinematic equations
        let g = 2000.0;
        let prev_y_vel = self.y_vel;
        let new_y_vel = self.y_vel + g * dt;
        let ave_y_vel = (prev_y_vel + new_y_vel) / 2.0;
        let expected_y = self.y + ave_y_vel * dt;
        if expected_y > 0.0 {
            self.change_y_pos(0.0);
            self.change_y_vel(0.0);
        } else {
            self.change_y_pos(expected_y);
            self.change_y_vel(new_y_vel);
        }
        // Handle X movement
        let x = self.x;
        let x_boost = self.x_boost;
        let x_speed = self.x_speed;
        let x_total_speed = x_boost + x_speed;
        match self.x_dir {
            XDir::Left => self.change_x_pos(x - x_total_speed * dt),
                XDir::Right => self.change_x_pos(x + x_total_speed * dt),
                XDir::Static => (),
        }

        if self.y >= 0.0 {
            self.change_x_boost(0.0);
        } else {
            let drag_const = 15.0;
            let drag_vel = drag_const * x_total_speed * dt;
            let new_boost = 0f64.max(x_boost - drag_vel);
            self.change_x_boost(new_boost);
        }
    }

    fn change_y_pos(&mut self, y: f64) {
        self.y = y;
    }

    fn change_x_pos(&mut self, x: f64) {
        self.x = x;
    }

    fn change_y_vel(&mut self, y_vel: f64) {
        self.y_vel = y_vel;
    }

    fn change_x_dir(&mut self, x_dir: XDir) {
        self.x_dir = x_dir;
    }

    fn change_x_boost(&mut self, x_boost: f64) {
        self.x_boost = x_boost;
    }

    fn boost(&mut self) {
        let boost_vel = 5.0 * self.x_speed;
        if self.y < 0.0 && self.x_boost == 0.0 {
            self.change_x_boost(boost_vel);
        }

    }

    fn jump(&mut self) {
        const JUMP_VEL: f64 = -850.0; // since negative is actually up
        if self.y >= 0.0 {
            // Jumper must be on ground
            self.change_y_vel(JUMP_VEL);
        }
    }
}
