use piston::input::Button;
use player::player_struct::Player;

pub trait ButtonInteraction {
    #[allow(unused_variables)]
    fn handle_button(player: &mut Player, button: Button) {}

    #[allow(unused_variables)]
    fn handle_release(player: &mut Player, button: Button) {}
}

pub trait TimeChange {
    #[allow(unused_variables)]
    fn handle_time_change(player: &mut Player, dt: f64) {}
}
