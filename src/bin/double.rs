use getopts::Options;
use itertools::interleave;
use nannou::app::LoopMode;
use nannou::noise::NoiseFn;
use nannou::prelude::*;
use std::env;

use sketches::{img_path, random_rgb, random_rgba, Grid};

const WIDTH: f32 = 1200.0;
const HEIGHT: f32 = 900.0;
const GRID_SPACING: f32 = 2.0;
const STEP_SIZE: f32 = 15.0;
const LENGTH: usize = 300;
const K: f64 = 0.002;
const K2: f64 = 0.004;
const LINES: usize = 1;

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
    draw.background().color(BLACK);

    let nn = nannou::noise::BasicMulti::new();
    let mm = nannou::noise::BasicMulti::new();

    let grid1 = Grid::new(1.1 * WIDTH, 1.1 * HEIGHT, GRID_SPACING, |x, y| {
        TAU * nn.get([K * x as f64, K * y as f64]) as f32
    });
    
    let grid2 = Grid::new(1.1 * WIDTH, 1.1 * HEIGHT, GRID_SPACING, |x, y| {
        TAU * mm.get([K2 * x as f64, K2 * y as f64]) as f32
    });

    for _ in 0..LINES {
        let mut loc1 = pt2(
            random_range(-WIDTH / 2.0, WIDTH / 2.0),
            random_range(-HEIGHT / 2.0, HEIGHT / 2.0),
        );
        let mut loc2 = loc1.clone();
        let mut points1 = vec![];
        let mut points2 = vec![];

        for _i in 0..LENGTH {
            points1.push(loc1);
            points2.push(loc2);
            let angle1 = &grid1.get(loc1.x, loc1.y);
            let angle2 = &grid2.get(loc2.x, loc2.y);
            loc1.x += STEP_SIZE * angle1.cos();
            loc1.y += STEP_SIZE * angle1.sin();
            loc2.x += STEP_SIZE * angle2.cos();
            loc2.y += STEP_SIZE *  angle2.sin();
        }
        // points2.reverse();
        // points1.extend(points2);
        // draw.polygon()
        //     .points(points1)
        //     .color(random_rgba());
        let points = interleave(points1, points2);
        draw.polyline().weight(2.0).color(WHITE).points(points);
    }

    if png {
        let file_path = img_path(app);
        app.main_window().capture_frame(file_path);
    }

    draw.to_frame(app, &frame).unwrap();
}
