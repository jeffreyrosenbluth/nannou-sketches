use getopts::Options;
use nannou::prelude::*;
use std::env;

use sketches::captured_frame_path;

const WIDTH: u32 = 700;
const HEIGHT: u32 = 700;
const POINTS: u32 = 2000;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    loc: u32,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(WIDTH, HEIGHT)
        .view(view)
        .build()
        .unwrap();
    Model { loc: 1 }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.loc < POINTS {
        model.loc += 1;
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
    if frame.nth() == 0 {
        draw.background().color(BLACK);
    }

    let r1 = 6.0 * (model.loc as f32).sqrt();
    let theta1 = 2.4 * model.loc as f32;
    let x1 = r1 * theta1.cos();
    let y1 = r1 * theta1.sin();
    draw.ellipse().x_y(x1, y1).color(INDIGO).w_h(9.0, 9.0);

    let r0 = 6.0 * (model.loc as f32).sqrt() - 2.0;
    let theta0 = 2.4 * model.loc as f32;
    let x0 = r0 * theta0.cos();
    let y0 = r0 * theta0.sin();
    draw.ellipse()
        .x_y(x0, y0)
        .color(WHITE)
        .w_h(5.0, 5.0)
        .stroke(GRAY)
        .stroke_weight(2.0);

    draw.rect()
        .wh(app.window_rect().wh())
        .color(srgba(0.0, 0.0, 0.0, 0.001));

    if png && model.loc == POINTS {
        let file_path = captured_frame_path(app, &frame);
        app.main_window().capture_frame(file_path);
    }
    if model.loc >= POINTS {
        app.set_loop_mode(LoopMode::loop_once());
    }

    draw.to_frame(app, &frame).unwrap();
}
