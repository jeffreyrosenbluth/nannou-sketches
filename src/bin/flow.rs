use getopts::Options;
use nannou::app::LoopMode;
use nannou::noise::NoiseFn;
use nannou::prelude::*;
use std::env;

use sketches::{img_path, random_rgb, Grid};

const WIDTH: f32 = 1200.0;
const HEIGHT: f32 = 900.0;
const GRID_SPACING: f32 = 50.0;
const LENGTH: usize = 1000;
const K: f64 = 0.002;
const LINES: usize = 50;

fn main() {
    nannou::sketch(view).size(WIDTH as u32, HEIGHT as u32).run()
}

fn view(app: &App, frame: Frame) {
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
    draw.background().color(CORNSILK);

    let nn = nannou::noise::BasicMulti::new();

    let grid = Grid::new(1.1 * WIDTH, 1.1 * HEIGHT, GRID_SPACING, |x, y| {
         TAU * nn.get([K * x as f64, K * y as f64]) as f32
    });

    for _ in 0..LINES {
        let mut loc = pt2(
            random_range(-WIDTH / 2.0, WIDTH / 2.0),
            random_range(-HEIGHT / 2.0, HEIGHT / 2.0),
        );
        let mut points = vec![];
        for _i in 0..LENGTH {
            points.push(loc);
            let angle = &grid.get(loc.x, loc.y);
            loc.x += angle.cos();
            loc.y += angle.sin();
        }
        draw.polygon()
            .points(points)
            .color(random_rgb());
    }

    if png {
        let file_path = img_path(app);
        app.main_window().capture_frame(file_path);
    }

    draw.to_frame(app, &frame).unwrap();
}
