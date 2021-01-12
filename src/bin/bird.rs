use getopts::Options;
use nannou::prelude::*;
use std::env;

use sketches::{gen_points, img_path};

const WIDTH: f32 = 1200.0;
const HEIGHT: f32 = 700.0;

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


fn sx(t: f32) -> f32 {
    1.5 * (TAU * t + PI / 2.5).sin().pow(3.0)
}

fn sy(t: f32) -> f32 {
    0.35 * (3.0 * TAU * t).cos().pow(4.0)
}

fn ex(t: f32) -> f32 {
    0.2 * (3.0 * TAU * t).sin()
}

fn ey(t: f32) -> f32 {
    -0.8 * (TAU * t - PI / 3.0).sin().pow(2.0)
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let k = 350.0;
    let f = |p| k * sx(p);
    let g = |p| k * sy(p);
    let start_points = gen_points(f, g, 1.0 / 500.0, 1.0);
    model.start_pts = start_points;
    let f = |p| k * ex(p);
    let g = |p| k * ey(p);
    let end_points = gen_points(f, g, 1.0 / 500.0, 1.0);
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
        draw.line().color(DARKKHAKI).weight(0.5).points(*s, e);
    }

    if png {
        let file_path = img_path(app);
        app.main_window().capture_frame(file_path);
    }
    app.set_loop_mode(LoopMode::loop_once());
    draw.to_frame(app, &frame).unwrap();
}
