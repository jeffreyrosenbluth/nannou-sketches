use getopts::Options;
use nannou::prelude::*;
use std::env;

use sketches::captured_frame_path;

const WIDTH: u32 = 900;
const HEIGHT: u32 = 600;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {}

fn model(app: &App) -> Model {
    app.new_window()
        .size(WIDTH, HEIGHT)
        .view(view)
        .build()
        .unwrap();
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

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

    if png {
        let file_path = captured_frame_path(app, &frame);
        app.main_window().capture_frame(file_path);
    }

    draw.to_frame(app, &frame).unwrap();
}
