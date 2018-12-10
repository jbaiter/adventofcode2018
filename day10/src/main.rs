use std::error::Error;
use std::io::{self, BufRead};
use std::str::FromStr;
use glutin_window::GlutinWindow;
use lazy_static::lazy_static;
use opengl_graphics::{ GlGraphics, OpenGL };
use piston::window::{AdvancedWindow, Window, WindowSettings, Size};
use piston::event_loop::*;
use piston::input::*;
use piston::input::keyboard::Key;
use regex::Regex;

type Result<T> = std::result::Result<T, Box<Error>>;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<Error>::from(format!($($tt)*))) }
}


#[derive(Debug, Clone)]
struct Star {
    x: f64,
    y: f64,
    velocity: (f64, f64),
}

impl FromStr for Star {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Star> {
        lazy_static! {
            static ref POINT_RE: Regex = Regex::new(
                r"^position=<\s*(?P<pos_x>-?\d+),\s*(?P<pos_y>-?\d+)> velocity=<\s*(?P<vel_x>-?\d+),\s*(?P<vel_y>-?\d+)>$").unwrap();
        }
        let caps = match POINT_RE.captures(s) {
            Some(caps) => Ok(caps),
            None       => err!("Bad point entry, could not parse: {}", s)
        }?;
        Ok(Star {
            x: caps["pos_x"].parse()?,
            y: caps["pos_y"].parse()?,
            velocity: (caps["vel_x"].parse()?, caps["vel_y"].parse()?),
        })
    }
}

impl Star {
    fn move_by(&mut self, delta_t: f64) {
        let (vel_x, vel_y) = self.velocity;
        self.x += vel_x * delta_t;
        self.y += vel_y * delta_t;
    }
}

struct Sky {
    runtime: f64,
    stars: Vec<Star>,
}

impl Sky {
    fn advance_by(&mut self, delta_t: f64) {
        for star in &mut self.stars {
            star.move_by(delta_t);
        }
        self.runtime += delta_t;
        self.recenter();
    }

    fn recenter(&mut self) {
        let (x0, y0, x1, y1) = self.viewport_boundaries();
        let cx = (x1 + x0) / 2.0;
        let cy = (y1 + y0) / 2.0;
        for star in &mut self.stars {
            star.x -= cx;
            star.y -= cy;
        }
    }

    fn viewport_boundaries(&self) -> (f64, f64, f64, f64) {
        // We shouldn't have any NaNs, so we just unwrap
        (self.stars.iter()
            .map(|s| s.x).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap(),
         self.stars.iter()
            .map(|s| s.y).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap(),
         self.stars.iter()
            .map(|s| s.x).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap(),
         self.stars.iter()
            .map(|s| s.y).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap())
    }

    fn viewport(&self) -> (usize, usize) {
        let (x0, y0, x1, y1) = self.viewport_boundaries();
        ((x1 - x0) as usize, (y1 - y0) as usize)
    }

    fn forward_to_viewport(&mut self, width: usize, height: usize) {
        while self.viewport() > (width, height) {
            self.advance_by(1.0);
        }
    }
}




pub struct App {
    gl: GlGraphics,
    sky: Sky,
    paused: bool,
    advance: Option<f64>,
}

impl App {
    fn new(gl: GlGraphics, stars: Vec<Star>) -> App {
        App {
            gl,
            sky: Sky { runtime: 0.0, stars: stars },
            paused: false,
            advance: None,
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const STAR_SIZE: f64 = 4.0;
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let circle = ellipse::circle(0.0, 0.0, STAR_SIZE / 3.0);
        let (cx, cy) = (args.width / 2.0,
                        args.height / 2.0);

        let stars = &mut self.sky.stars;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            for star in stars {
                let trans = c.transform.trans(
                    cx + star.x * STAR_SIZE,
                    cy + star.y * STAR_SIZE);
                ellipse(WHITE, circle, trans, gl);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        if self.paused {
            match self.advance {
                Some(delta) => {
                    self.sky.advance_by(delta);
                },
                None => ()
            };
            self.advance = None;
        } else {
            self.sky.advance_by(args.dt);
        }
    }

    fn handle_key(&mut self, key: &Key) {
        match key {
            Key::Space => self.paused = !self.paused,
            Key::Period => {
                if !self.paused {
                    self.paused = true;
                }
                self.advance = match self.advance {
                    Some(delta) => Some(delta + 1.0),
                    None        => {
                        let mut d = self.sky.runtime.round() - self.sky.runtime;
                        if d == 0.0 {
                            d = 1.0;
                        }
                        Some(d)
                    }
                };
            },
            Key::Comma => {
                if !self.paused {
                    self.paused = true;
                }
                self.advance = match self.advance {
                    Some(delta) => Some(delta - 1.0),
                    None        => Some(-1.0),
                };
            },
            _ => ()
        }
    }
}

fn main() -> Result<()> {
    let opengl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new(
            "aoc2018-day10: Step forward with the period key",
            [1024, 768]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()?;

    let stdin = io::stdin();
    let stars: Vec<Star> = stdin.lock().lines()
        .map(|l| l?.parse())
        .collect::<Result<_>>()?;

    let mut app = App::new(GlGraphics::new(opengl), stars);
    let Size {width: w, height: h} = window.size();
    let pix_per_star = 8.0 / 3.0;
    app.sky.forward_to_viewport((w / pix_per_star) as usize,
                                (h / pix_per_star) as usize);
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(btn) = e.press_args() {
            match btn {
                Button::Keyboard(key) => app.handle_key(&key),
                _ => (),
            };
        }

        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
        window.set_title(format!("Runtime: {:.2}", app.sky.runtime));
    };
    Ok(())
}
