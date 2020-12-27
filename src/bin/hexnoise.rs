use getopts::Options;
use nannou::prelude::*;
use std::env;

use sketches::{arc, captured_frame_path};

const SIZE: f32 = 25.0;
const WIDTH: f32 = 900.0;
const HEIGHT: f32 = 900.0;

fn main() {
    nannou::app(model).update(update).run()
}

struct Model {
    line_width: f32,
    angles: Vec<Vec<f32>>,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(WIDTH as u32, HEIGHT as u32)
        .view(view)
        .build()
        .unwrap();
    let h = (PI / 3.0).sin() * SIZE;
    let m = (WIDTH / (SIZE * 3.0)) as usize;
    let n = (HEIGHT / h) as usize + 1;
    let mut angles = vec![vec![0.0; n+1]; m+1];
    for i in 0..=m {
        for j in 0..=n {
            angles[i][j] = random_range(0, 3) as f32 * PI / 3.0;
        }
    }

    dbg!(angles.len());

    Model {
        line_width: 25.0,
        angles,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.line_width > 2.0 {
        model.line_width -= 0.05;
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

    let h = (PI / 3.0).sin() * SIZE;
    let width2 = WIDTH / 2.0;
    let height2 = HEIGHT / 2.0;
    let draw = app.draw();

    if frame.nth() == 0 {
        frame.clear(BLACK);
    }
    draw.rect().w_h(WIDTH, HEIGHT).color(BLACK);
    for i in 0..=(WIDTH / (SIZE * 3.0)) as usize {
        for j in 0..=(HEIGHT / h) as usize + 1 {
            let mut x = i as f32 * SIZE * 3.0 + (SIZE / 2.0) - width2;
            let y = j as f32 * h - height2;
            if j % 2 > 0 {
                x += SIZE * 1.5;
            }
            let angle = model.angles[i][j];
            // let angle = random_range(0, 3) as f32 * PI / 3.0;
            let d = draw.translate(vec3(x, y, 0.0));
            let d = d.rotate(angle);
            let c = if model.line_width > 2.05 { GRAY } else { WHITE };
            d.line()
                .points(pt2(0.0, -h), pt2(0.0, h))
                .color(c)
                .stroke_weight(model.line_width);
            arc(&d, 300.0, 120.0, SIZE / 2.0, c, model.line_width).x_y(-SIZE, 0.0);
            arc(&d, 120.0, 120.0, SIZE / 2.0, c, model.line_width).x_y(SIZE, 0.0);
        }
    }
    if png {
        let file_path = captured_frame_path(app, &frame);
        app.main_window().capture_frame(file_path);
    }
    draw.to_frame(app, &frame).unwrap();
}
