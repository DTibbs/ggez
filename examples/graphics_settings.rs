extern crate ggez;
extern crate clap;
use clap::{Arg, App};
use ggez::*;
use ggez::graphics::{DrawMode, Point2};

struct MainState {
    pos_x: f32,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState { pos_x: 0.0 };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.pos_x = self.pos_x % 800.0 + 1.0;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::circle(ctx,
                         DrawMode::Line(3.0),
                         Point2::new(400.0, 300.0),
                         100.0,
                         4.0)?;
        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() {
    let matches = App::new("graphics settings example")
        .arg(Arg::with_name("msaa")
                 .short("m")
                 .value_name("N")
                 .help("Number of MSAA samples to do (powers of 2 from 1 to 16)")
                 .takes_value(true))
        .get_matches();

    let msaa: u32 = matches
        .value_of("msaa")
        .unwrap_or("1")
        .parse()
        .expect("Option msaa needs to be a number!");
    let mut c = conf::Conf::new();
    c.window_mode.samples =
        conf::NumSamples::from_u32(msaa).expect("Option msaa needs to be 1, 2, 4, 8 or 16!");
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
