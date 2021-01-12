use getopts::Options;
use nannou::app::LoopMode;
use nannou::noise::NoiseFn;
use nannou::prelude::*;
use std::env;

use sketches::{img_path, random_rgb, Grid};

const WIDTH: f32 = 1200.0;
const HEIGHT: f32 = 900.0;
const GRID_SPACING: f32 = 2.0;
const LENGTH: usize = 500;
const K: f64 = 0.003;

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

    for i in 0..(grid.cols() / 4) {
        let mut l1 = pt2(
            -WIDTH / 2.0 +  GRID_SPACING * i as f32,
            0.0,
        );
        let mut l2 = pt2(
            -WIDTH / 2.0 + GRID_SPACING * (i + 1) as f32,
            0.0,
        );
        let mut up = vec![];
        for _i in 0..LENGTH {
            up.push(l1);
            let angle = &grid.get(l1.x, l1.y);
            l1.x += 10.0 * angle.cos();
            l1.y += 10.0 * angle.sin();
        }
        let mut dn = vec![];
        for _i in 0..LENGTH {
            dn.push(l2);
            let angle = &grid.get(l2.x, l2.y);
            l2.x += 10.0 * angle.cos();
            l2.y += 10.0 * angle.sin();
        }
        dn.reverse();
        up.extend(dn);

        draw.polygon().points(up).color(random_rgb());
    }

    if png {
        let file_path = img_path(app);
        app.main_window().capture_frame(file_path);
    }

    draw.to_frame(app, &frame).unwrap();
}
