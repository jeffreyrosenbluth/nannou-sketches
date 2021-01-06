use getopts::Options;
use nannou::noise::NoiseFn;
use nannou::prelude::*;
use std::env;

use sketches::{border, img_path};

const WIDTH: u32 = 900;
const HEIGHT: u32 = 600;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    distortion: f64,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(WIDTH, HEIGHT)
        .view(view)
        .build()
        .unwrap();
    Model { distortion: 0.03 }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.distortion >= -0.03 {
        model.distortion -= 0.00001;
    }
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
    // if frame.nth() == 0 {
    draw.background().color(BLACK);
    // }
    let y0 = -(HEIGHT as f32) / 2.0;
    let y1 = HEIGHT as f32 / 2.0;
    let mut ys = vec![];
    let delta_y = 4.0;
    let mut y = y0;
    while y < y1 {
        ys.push(y);
        y += delta_y;
    }

    let nn = nannou::noise::OpenSimplex::new();
    for l in 0..WIDTH / 4 {
        let x = l as f32 * 4.0 - (WIDTH as f32 / 2.0);
        let mut ps = vec![];
        let k = model.distortion;
        for y in &ys {
            let delta = x * nn.get([k * x as f64, k * *y as f64]) as f32;
            ps.push(pt2(x + delta, *y))
        }
        draw.polyline().weight(1.0).color(WHITE).points(ps);
    }

    border(app, &draw, 50.0);

    if png && (model.distortion * 100.0) as i32 == 1 {
        let file_path = img_path(app);
        app.main_window().capture_frame(file_path);
    }

    draw.to_frame(app, &frame).unwrap();
}
