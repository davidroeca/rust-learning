extern crate piston;
extern crate graphics;
extern crate piston_window;
extern crate opengl_graphics;

use piston_window::{
    PistonWindow,
    Events,
    WindowSettings,
};

use piston::input::{
    RenderEvent,
    UpdateEvent,
    PressEvent,
    ReleaseEvent,
    RenderArgs,
    UpdateArgs,
};

use opengl_graphics::{
    OpenGL,
    GlGraphics,
};
use opengl_graphics::glyph_cache::GlyphCache;

mod utils;
mod player;
mod orb;
use player::Player;

use orb::{Orb, OrbType};
use utils::random_f64_less_than;

///
/// Takes the center of a circle and a square, along with side length/radius
/// and comuptes whether or not the two overlap
///
fn square_circle_overlap(cx: f64, cy: f64, cr: f64, sx: f64, sy: f64, ss: f64)
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

pub struct App {
    gl: GlGraphics,
    player: Player,
    orbs: Vec<Orb>,
    s: f64,
}

impl App {

    pub fn new(s: f64, glref: OpenGL) -> App {
        let orbs: Vec<Orb> = Vec::new();
        let x_speed = 300.0;
        App {
            gl: GlGraphics::new(glref),
            player: Player::new(x_speed),
            orbs: orbs,
            s: s,
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::{
            clear,
            rectangle,
            ellipse,
            line,
            text,
            Transformed,
        };
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        let square = rectangle::square(0.0, 0.0, self.s);
        let horiz_line: [f64; 4] = [0.0, 0.0, 0.0, 1.0];

        let x_origin = (args.width / 2) as f64;
        let y_origin = (args.height / 2) as f64;
        let ground_y = y_origin + self.s / 2.0;
        let (x, y) = (
            x_origin + self.player.x - self.s / 2.0,
            y_origin + self.player.y - self.s / 2.0,
            );
        let orb_iter = self.orbs.iter();
        let top_dir = utils::get_project_dir().expect("top directory not found");
        let font_path = top_dir.join("font/Xolonium-Regular.ttf");
        let mut glyph_cache = GlyphCache::new(font_path)
            .expect("failed to load font");
        let (text_x, text_y) = (
            10.0, 50.0
            );
        let sec_int = self.player.time_s as i32;
        self.gl.draw(args.viewport(), |c, gl| {
            clear(WHITE, gl);
            let text_trans = c.transform.trans(text_x, text_y);
            text(
                BLACK, 40, sec_int.to_string().as_ref(), &mut glyph_cache,
                text_trans, gl
                );
            let transform = c.transform.trans(x, y);
            rectangle(BLUE, square, transform, gl);
            let ground_trans = c.transform.trans(0.0, ground_y);
            line(
                BLACK, args.width as f64,
                horiz_line, ground_trans, gl
                );
            for o in orb_iter {
                let color = match o.orb_type {
                    OrbType::Homing => RED,
                    OrbType::Roaming => GREEN,
                };
                let o_x = x_origin + o.x - o.r;
                let o_y = y_origin + o.y - o.r;
                let sq = rectangle::square(0.0, 0.0, o.r * 2.0);
                let o_trans = c.transform.trans(o_x, o_y);
                ellipse(color, sq, o_trans, gl);
            }
        })
    }

    fn update(&mut self, args: &UpdateArgs) {
        let num_orbs = 20;
        let p_x = self.player.x;
        let p_y = self.player.y;
        let p_s = self.s;

        let mut vec_deleted = Vec::new();
        // Handle collisions
        for (i, o) in self.orbs.iter().enumerate() {
            if square_circle_overlap(o.x, o.y, o.r, p_x, p_y, p_s) {
                vec_deleted.push(i);
            }
        }

        // Go backwards to ensure that order remains in tact
        for i in vec_deleted.iter().rev() {
            self.orbs.remove(*i);
        }

        for o in self.orbs.iter_mut() {
            o.handle_time_change(p_x, p_y, args.dt);
        }
        self.player.handle_time_change(args.dt);
        if self.orbs.len() == 0 {
            for _ in 0..num_orbs/2 {
                let orb_speed = 50.0 + random_f64_less_than(200.0);
                self.orbs.push(Orb::new(
                        6.0, 800, 800, orb_speed, OrbType::Homing
                        ));
            }
            for _ in 0..num_orbs/2 {
                let orb_speed = 50.0 + random_f64_less_than(200.0);
                self.orbs.push(Orb::new(
                        6.0, 800, 800, orb_speed, OrbType::Roaming
                        ));
            }
        }
    }

}

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new(
        "Platformer Prototype",
        [800, 800],
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut app = App::new(30.0, opengl);
    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(button) = e.press_args() {
            app.player.handle_button(button);
        }

        if let Some(button) = e.release_args() {
            app.player.handle_release(button);
        }
    }
}
