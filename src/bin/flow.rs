use getopts::Options;
use nannou::app::LoopMode;
use nannou::prelude::*;
use std::env;

use sketches::{img_path, Grid};

const WIDTH: f32 = 1200.0;
const HEIGHT: f32 = 900.0;
const GRID_SPACING: f32 = 20.0;

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

    let grid = Grid::new(2.0 * WIDTH, 2.0 * HEIGHT, GRID_SPACING, |x, _| x / HEIGHT * PI);

    for (p, v) in grid.iter() {
        let x = p.x + 10.0 * v.cos();
        let y = p.y + 10.0 * v.sin();
        draw.line().points(p, pt2(x, y)).color(GREEN);
    }


    let (xl, xr) = grid.x_bounds();
    let (yb, yt) = grid.y_bounds();
    let mut loc = pt2(-WIDTH / 4.0, 0.0);

    for _i in 0..400 {
        if loc.x <= xl || loc.x >= xr || loc.y <= yb || loc.y >= yt { break }
        draw.ellipse().xy(loc).w_h(4.0, 4.0).color(WHITE);
        let angle = &grid.get(loc.x, loc.y);
        loc.x += 2.0 * angle.cos();
        loc.y += 2.0 * angle.sin();
    }

    if png {
        let file_path = img_path(app);
        app.main_window().capture_frame(file_path);
    }

    draw.to_frame(app, &frame).unwrap();
}
