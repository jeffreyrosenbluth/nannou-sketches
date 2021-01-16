use getopts::Options;
use nannou::prelude::*;
use std::env;

use sketches::gif_path;

const WIDTH: f32 = 700.0;
const HEIGHT: f32 = 700.0;
const CIRCLES: usize = 2500;
const VERTICES: usize = 30;
const SCALE: f32 = 300.0;
const WEIGHT: f32 = 2.0;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    time: f32,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(WIDTH as u32, HEIGHT as u32)
        .view(view)
        .build()
        .unwrap();
    Model { time: 0.0 }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.time = app.elapsed_frames() as f32 / 100.0;
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

    for c in 0..CIRCLES {
        let theta = map_range(c as f32, 0.0, CIRCLES as f32, 0.0, TAU);
        let center = center(theta, model.time);
        let size = size(theta, model.time);
        let color = color(theta, model.time);
        let mut vertices = vec![];

        for i in 0..VERTICES {
            let gamma = map_range(i as f32, 0.0, VERTICES as f32, 0.0, TAU);
            let x = center.x + gamma.cos() * size;
            let y = center.y + gamma.sin() * size;
            vertices.push(pt2(x, y));
        }
        draw.polygon()
            .no_fill()
            .stroke_color(color)
            .stroke_weight(WEIGHT)
            .points(vertices);
    }

    if png {
        let file_path = gif_path(app, &frame);
        app.main_window().capture_frame(file_path);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn center(theta: f32, time: f32) -> Vector2 {
    let direction = vec2(theta.cos(), theta.sin());
    let distance = 0.6 + 0.2 * (theta * 6.0 + (theta * 8.0 + time).cos()).cos();
    direction * distance * SCALE
}

fn size(theta: f32, time: f32) -> f32 {
    let offset = 0.2 + 0.12 * (theta * 9.0 - time * 2.0).cos();
    SCALE * offset
}

fn color(theta: f32, time: f32) -> nannou::color::Alpha<rgb::Rgb, f32> {
    let th = 8.0 * theta + time * 2.0;
    let r = 0.6 + 0.4 * th.cos();
    let g = 0.6 + 0.4 * (th - PI / 3.0).cos();
    let b = 0.6 + 0.4 * (th - PI * 2.0 / 3.0).cos();
    let a = 0.12;
    srgba(r, g, b, a)
}
