use piston::input::{
    Button,
    Key,
};

use player::{
    Player,
    ButtonInteraction,
    TimeChange,
};

#[derive(Debug)]
pub enum XMovement {
    Left,
    Right,
    Static,
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
                    Button::Keyboard(Key::Left) => {
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
