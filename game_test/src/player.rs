use piston::input::{
    Button,
    Key,
};

pub enum XMovement {
    Left,
    Right,
    Static,
}

pub enum YState {
    InAir,
    OnGround,
}

pub struct Player {
    pub x: f64,
    pub y: f64,
    y_vel: f64,
    x_speed: f64,
    x_boost: f64,
    x_movement: XMovement,
    y_state: YState,
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

    fn change_y_pos(&mut self, y: f64) {
        self.y = y;
    }

    fn change_x_pos(&mut self, x: f64) {
        self.x = x;
    }

    fn change_y_vel(&mut self, y_vel: f64) {
        self.y_vel = y_vel;
    }

    fn change_x_boost(&mut self, x_boost: f64) {
        self.x_boost = x_boost;
    }

    fn boost(&mut self) {
        let boost_vel = 2.0 * self.x_speed;
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

    fn change_x_movement(&mut self, x_movement: XMovement) {
        self.x_movement = x_movement;
    }

    fn change_y_state(&mut self, y_state: YState) {
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


trait ButtonInteraction {
    #[allow(unused_variables)]
    fn handle_button(player: &mut Player, button: Button) {}

    #[allow(unused_variables)]
    fn handle_release(player: &mut Player, button: Button) {}
}

trait TimeChange {
    #[allow(unused_variables)]
    fn handle_time_change(player: &mut Player, dt: f64) {}
}

impl ButtonInteraction for XMovement {
    fn handle_button(player: &mut Player, button: Button) {
        match player.x_movement {
            XMovement::Static => {
                match button {
                    Button::Keyboard(Key::Right) => {
                        player.change_x_movement(XMovement::Right);
                    },
                    Button::Keyboard(Key::Left) => {
                        player.change_x_movement(XMovement::Left);
                    },
                    _ => (),
                }
            }
            XMovement::Left => {
                match button {
                    Button::Keyboard(Key::Right) => {
                        player.change_x_movement(XMovement::Right);
                        player.change_x_boost(0.0);
                    },
                    _ => (),
                }
            },
            XMovement::Right => {
                match button {
                    Button::Keyboard(Key::Right) => {
                        player.change_x_movement(XMovement::Left);
                        player.change_x_boost(0.0);
                    },
                    _ => (),
                }
            }
        }
    }

    fn handle_release(player: &mut Player, button: Button) {
        match player.x_movement {
            XMovement::Left => {
                match button {
                    Button::Keyboard(Key::Left) => {
                        player.change_x_movement(XMovement::Static);
                        player.change_x_boost(0.0);
                    },
                    _ => (),
                }
            },
            XMovement::Right => {
                match button {
                    Button::Keyboard(Key::Right) => {
                        player.change_x_movement(XMovement::Static);
                        player.change_x_boost(0.0);
                    },
                    _ => (),
                }
            },
            _ => (),
        }
    }
}

impl ButtonInteraction for YState {
    fn handle_button(player: &mut Player, button: Button) {
        match player.y_state {
            YState::OnGround => match button {
                Button::Keyboard(Key::Up) => {
                    player.jump();
                    player.change_y_state(YState::InAir);
                },
                _ => (),
            },
            YState::InAir => match button {
                Button::Keyboard(Key::Space) => match player.x_movement {
                    XMovement::Left | XMovement::Right => player.boost(),
                    _ => (),
                },
                _ => (),
            }
        }
    }

}

impl TimeChange for XMovement {
    fn handle_time_change(player: &mut Player, dt: f64) {
        let x = player.x;
        let x_boost = player.x_boost;
        let x_speed = player.x_speed;
        let x_total_speed = x_boost + x_speed;
        match player.x_movement {
            XMovement::Left => {
                player.change_x_pos(x - x_total_speed * dt)
            },
            XMovement::Right => {
                player.change_x_pos(x + x_total_speed * dt)
            },
            XMovement::Static => (),
        }
    }
}

impl TimeChange for YState {
    fn handle_time_change(player: &mut Player, dt: f64) {
        // Update the time variable
        player.time_s += dt;
        match player.y_state {
            YState::OnGround => {
                player.change_x_boost(0.0);
            },
            YState::InAir => {
                // Basic kinematic equations
                let g = 2000.0;
                let prev_y_vel = player.y_vel;
                let new_y_vel = player.y_vel + g * dt;
                let ave_y_vel = (prev_y_vel + new_y_vel) / 2.0;
                let expected_y = player.y + ave_y_vel * dt;
                if expected_y > 0.0 {
                    player.change_y_pos(0.0);
                    player.change_y_vel(0.0);
                    player.change_y_state(YState::OnGround);
                } else {
                    player.change_y_pos(expected_y);
                    player.change_y_vel(new_y_vel);
                }

                let x_total_speed = player.x_boost + player.x_speed;
                let x_boost = player.x_boost;
                if player.y >= 0.0 {
                    player.change_x_boost(0.0);
                } else {
                    let drag_const = 3.0;
                    let drag_vel = drag_const * x_total_speed * dt;
                    let new_boost = 0f64.max(x_boost - drag_vel);
                    player.change_x_boost(new_boost);
                }
            }
        }

    }
}

