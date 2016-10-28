use piston::input::{
    Button,
};

use player::{
    XMovement,
    YState,
    ButtonInteraction,
    TimeChange,
};

#[derive(Debug)]
pub struct Player {
    pub x: f64,
    pub y: f64,
    pub y_vel: f64,
    pub x_speed: f64,
    pub x_boost: f64,
    pub x_movement: XMovement,
    pub y_state: YState,
    pub time_s: f64,
}

impl Player {
    pub fn new(x_speed_in: f64) -> Player {
        Player {
            x: 0.0,
            y: 0.0,
            y_vel: 0.0,
            x_speed: x_speed_in,
            x_boost: 0.0,
            x_movement: XMovement::Static,
            y_state: YState::OnGround,
            time_s: 0.0,
        }
    }

    pub fn change_y_pos(&mut self, y: f64) {
        self.y = y;
    }

    pub fn change_x_pos(&mut self, x: f64) {
        self.x = x;
    }

    pub fn change_y_vel(&mut self, y_vel: f64) {
        self.y_vel = y_vel;
    }

    pub fn change_x_boost(&mut self, x_boost: f64) {
        self.x_boost = x_boost;
    }

    pub fn boost(&mut self) {
        let boost_vel = 2.0 * self.x_speed;
        if self.y < 0.0 && self.x_boost == 0.0 {
            self.change_x_boost(boost_vel);
        }

    }

    pub fn jump(&mut self) {
        const JUMP_VEL: f64 = -850.0; // since negative is actually up
        if self.y >= 0.0 {
            // Jumper must be on ground
            self.change_y_vel(JUMP_VEL);
        }
    }

    pub fn change_x_movement(&mut self, x_movement: XMovement) {
        self.x_movement = x_movement;
    }

    pub fn change_y_state(&mut self, y_state: YState) {
        self.y_state = y_state;
    }

    pub fn handle_button(&mut self, button: Button) {
        XMovement::handle_button(self, button);
        YState::handle_button(self, button);
    }

    pub fn handle_release(&mut self, button: Button) {
        XMovement::handle_release(self, button);
        YState::handle_release(self, button);
    }

    pub fn handle_time_change(&mut self, dt: f64) {
        XMovement::handle_time_change(self, dt);
        YState::handle_time_change(self, dt);
    }
}
