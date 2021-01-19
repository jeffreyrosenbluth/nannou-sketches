use colorous;
use getopts::Options;
use nannou::app::LoopMode;
use nannou::noise::NoiseFn;
use nannou::prelude::*;
use rand_distr::{Distribution, Geometric};
use std::env;

use sketches::{img_path, Grid};

const WIDTH: f32 = 1200.0;
const HEIGHT: f32 = 900.0;
const GRID_SPACING: f32 = 1.0;
const LENGTH: usize = 500;
const K: f64 = 0.025;
const LINES: usize = 700;
const STEP: f32 = 8.0;

fn main() {
    nannou::sketch(view).size(WIDTH as u32, HEIGHT as u32).run()
}

fn view(app: &App, frame: Frame) {
    let geo = Geometric::new(0.5).unwrap();
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("p", "png", "save frames to file as png.");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    let png = matches.opt_present("p");

    app.set_loop_mode(LoopMode::loop_once());
    let draw = app.draw();
    let colors = colorous::REDS;
    let c = colors.eval_rational(2, 10);
    let kolor = srgb8(c.r, c.g, c.b);
    draw.background().color(kolor);

    let nn = nannou::noise::BasicMulti::new();

    let grid = Grid::new(1.1 * WIDTH, 1.1 * HEIGHT, GRID_SPACING, |x, y| {
        TAU * nn.get([K * x as f64, K * y as f64]) as f32
    });

    for l in 0..LINES {
        let mut loc = pt2(-WIDTH / 2.0, random_range(-HEIGHT / 2.0, HEIGHT / 2.0));
        let mut points = vec![];
        for _i in 0..LENGTH {
            points.push(loc);
            let angle = &grid.get(loc.x, loc.y);
            let angle = map_range(*angle, 0.0, TAU, 0.0, PI);
            loc.x += STEP * angle.cos();
            loc.y += STEP * angle.sin();
        }
        let w = geo.sample(&mut rand::thread_rng()) as f32;
        let c = colors.eval_rational(l % 100, 100);
        let kolor = srgb8(c.r, c.g, c.b);
        draw.polygon().stroke_weight(w).color(kolor).points(points);
    }

    if png {
        let file_path = img_path(app);
        app.main_window().capture_frame(file_path);
    }

    draw.to_frame(app, &frame).unwrap();
}
