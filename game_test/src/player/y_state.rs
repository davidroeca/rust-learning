use piston::input::{
    Button,
    Key,
};

use player::{
    Player,
    ButtonInteraction,
    TimeChange,
    XMovement,
};

#[derive(Debug)]
pub enum YState {
    InAir,
    OnGround,
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
