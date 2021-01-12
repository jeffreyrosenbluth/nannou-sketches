use getopts::Options;
use nannou::prelude::*;
use std::env;

use sketches::{gen_points, gif_path};

const WIDTH: f32 = 1000.0;
const HEIGHT: f32 = 1000.0;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    start_pts: Vec<Point2>,
    end_pts: Vec<Point2>,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(WIDTH as u32, HEIGHT as u32)
        .view(view)
        .build()
        .unwrap();

    Model {
        start_pts: vec![],
        end_pts: vec![],
    }
}

fn sq(x: f32) -> Point2 {
    let x = x % 1.0;
    if x < 0.25 {
        pt2(4.0 * x, 0.0)
    } else if x < 0.5 {
        pt2(1.0, 4.0 * (x - 0.25))
    } else if x < 0.75 {
        pt2(4.0 * (0.75 - x), 1.0)
    } else {
        pt2(0.0, 4.0 * (1.0 - x))
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let n = app.elapsed_frames();
    let k = 800.0;
    let w = k / 2.0;
    let f = |p| k * sq(p).x - w;
    let g = |p| k * sq(p).y - w;
    let start_points = gen_points(f, g, 1.0 / 400.0, 1.0);
    model.start_pts = start_points;
    let gap = n as f32 / 720.0;
    let f = |p| k * sq(p + gap).x - w;
    let g = |p| k * sq(p + gap).y - w;
    let end_points = gen_points(f, g, 1.0 / 400.0, 1.0);
    model.end_pts = end_points;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("p", "png", "save frames to file as png.");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    let png = matches.opt_present("p");

    let draw = app.draw();
    draw.background().color(BLACK);

    for (i, s) in model.start_pts.iter().enumerate() {
        let e = model.end_pts[i];
        draw.line().color(WHITE).weight(0.5).points(*s, e);
    }

    if png {
        let file_path = gif_path(app, &frame);
        app.main_window().capture_frame(file_path);
        app.set_loop_mode(LoopMode::loop_ntimes(720));
    }

    draw.to_frame(app, &frame).unwrap();
}
