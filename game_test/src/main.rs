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
    Button,
    Key
};

use opengl_graphics::{
    OpenGL,
    GlGraphics,
};

pub enum XDir {
    Left,
    Right,
    Static,
}

pub struct State {
    x: f64,
    y: f64,
    y_vel: f64,
    x_speed: f64,
    x_dir: XDir,
    left_pressed: bool,
    right_pressed: bool,
}


impl State {
    fn new(x_speed_in: f64) -> State {
        State {
            x: 0.0,
            y: 0.0,
            y_vel: 0.0,
            x_speed: x_speed_in,
            x_dir: XDir::Static,
            left_pressed: false,
            right_pressed: false,
        }
    }

    fn handle_button(&mut self, button: Button) {
        match button {
            Button::Keyboard(Key::Up) => {
                self.jump();
            },
            Button::Keyboard(Key::Left) => {
                self.left_pressed = true;
                self.change_x_dir(XDir::Left);
            },
            Button::Keyboard(Key::Right) => {
                self.right_pressed = true;
                self.change_x_dir(XDir::Right);
            },
            _ => (),
        }
    }
    fn handle_release(&mut self, button: Button) {
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

    fn jump(&mut self) {
        const JUMP_VEL: f64 = -500.0; // since negative is actually up
        println!("Jump called; y_vel: {}", self.y_vel);
        if self.y >= 0.0 {
            // Jumper must be on ground
            self.change_y_vel(JUMP_VEL);
        }
    }

    fn handle_time_change(&mut self, dt: f64) {
        let g = 1500.0;
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
        let x_speed = self.x_speed;
        match self.x_dir {
            XDir::Left => self.change_x_pos(x - x_speed * dt),
            XDir::Right => self.change_x_pos(x + x_speed * dt),
            XDir::Static => (),
        }
    }
}


pub struct App {
    gl: GlGraphics,
    state: State,
    s: f64,
}

impl App {

    fn render(&mut self, args: &RenderArgs) {
        use graphics::{
            clear,
            rectangle,
            line,
            Transformed,
        };
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        let square = rectangle::square(0.0, 0.0, self.s);
        let horiz_line: [f64; 4] = [0.0, 0.0, 0.0, 1.0];

        let x_origin = (args.width / 2) as f64;
        let y_origin = (args.height / 2) as f64;
        let (x, y) = (
            x_origin + self.state.x - self.s,
            y_origin + self.state.y - self.s,
            );
        self.gl.draw(args.viewport(), |c, gl| {
            clear(WHITE, gl);
            let transform = c.transform.trans(x, y);
            rectangle(BLUE, square, transform, gl);
            let ground_trans = c.transform.trans(0.0, y_origin);
            line(
                BLACK, args.width as f64,
                horiz_line, ground_trans, gl
                );
        })
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Basic kinematic equations
        self.state.handle_time_change(args.dt);
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
    let x_speed = 300.0;
    let mut app = App {
        gl: GlGraphics::new(opengl),
        state: State::new(x_speed),
        s: 50.0,
    };
    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(button) = e.press_args() {
            app.state.handle_button(button);
        }

        if let Some(button) = e.release_args() {
            app.state.handle_release(button);
        }
    }
}
