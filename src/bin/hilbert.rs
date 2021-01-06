use getopts::Options;
use nannou::noise::NoiseFn;
use nannou::prelude::*;
use std::env;

use sketches::img_path;

const WIDTH: f32 = 700.0;

const ORDER: usize = 6;

fn main() {
    nannou::app(model).update(update).run()
}

#[derive(Debug)]
struct Model {
    path: Vec<Point2>,
    index: usize,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(WIDTH as u32 + 100, WIDTH as u32 + 100)
        .view(view)
        .build()
        .unwrap();

    let n = pow(2, ORDER) as usize;
    let total = n * n;
    let mut path = vec![];
    let nn = nannou::noise::OpenSimplex::new();

    for i in 0..total {
        path.push(hilbert(i, ORDER));
        let m = WIDTH / n as f32;
        path[i] *= m;
        path[i] += vec2(m / 2.0, m / 2.0);
        let x = path[i].x;
        let y = path[i].y;
        let delta_x = 0.04 * WIDTH * nn.get([0.01 * x as f64, 0.01 * y as f64, 0.0]) as f32;
        let delta_y = 0.04 * WIDTH * nn.get([0.01 * x as f64, 0.01 * y as f64, 0.1]) as f32;
        path[i] = pt2(x + delta_x, y + delta_y);
    }
    path = path
        .into_iter()
        .map(|p| p - vec2(WIDTH / 2.0, WIDTH / 2.0))
        .collect();

    Model { path, index: 1 }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.index < model.path.len() - 1 {
        model.index += 1;
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
    if frame.nth() == 0  {
        frame.clear(BLACK);
    }

    draw.line()
        .weight(2.0)
        .caps_round()
        .color(WHITE)
        .points(model.path[model.index - 1], model.path[model.index]);

    if png && frame.nth() == pow(2, ORDER) * pow(2, ORDER) - 1 {
        let file_path = img_path(app);
        app.main_window().capture_frame(file_path);
    }

    if model.index >= pow(2, ORDER) * pow(2, ORDER) {
        app.set_loop_mode(LoopMode::loop_once());
    }

    draw.to_frame(app, &frame).unwrap();
}

fn hilbert(k: usize, order: usize) -> Point2 {
    let points = vec![pt2(0.0, 0.0), pt2(0.0, 1.0), pt2(1.0, 1.0), pt2(1.0, 0.0)];
    let mut v = points[k & 3];
    let mut i = k;

    for j in 1..order {
        i >>= 2;
        let index = i & 3;
        let n = pow(2, j) as f32;
        match index {
            0 => {
                let temp = v.x;
                v.x = v.y;
                v.y = temp;
            }
            1 => {
                v.y += n;
            }
            2 => {
                v.x += n;
                v.y += n;
            }
            3 => {
                let temp = n - 1.0 - v.x;
                v.x = n - 1.0 - v.y;
                v.y = temp;
                v.x += n;
            }
            _ => {}
        }
    }
    v
}
