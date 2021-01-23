use getopts::Options;
use nannou::app::LoopMode;
use nannou::prelude::*;
use std::env;

use sketches::img_path;

const WIDTH: u32 = 900;
const HEIGHT: u32 = 900;

fn main() {
    nannou::sketch(view).size(WIDTH, HEIGHT).run()
}

fn view(app: &App, frame: Frame) {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("p", "png", "save frames to file as png.");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!("{}", f),
    };
    let png = matches.opt_present("p");

    app.set_loop_mode(LoopMode::loop_once());
    let draw = app.draw();
    draw.background().color(BLACK);

    for i in 0..65 {
        let theta = i as f32 / 64.0 * TAU;
        let draw = draw.rotate(theta);
        let r = 10.0 * (75.0 - i as f32) * theta.sin();
        let mut color = srgba(1.0, 1.0, 1.0, i as f32 / 100.0);
        if i == 10 {
            color = srgba(1.0, 0.0, 0.0, 1.0);
        }
        draw.ellipse()
            .no_fill()
            .stroke(color)
            .stroke_weight(1.0)
            // .stroke_weight(32.0)
            .x_y(i as f32 * 3.0, 0.0)
            .w_h(r, r);
    }
    let x = 64.0 * 3.0;
    draw.ellipse().color(RED).x_y(x, 0.0).w_h(75.0, 75.0);
    draw.ellipse().color(BLACK).x_y(x, 0.0).w_h(50.0, 50.0);

    if png {
        let file_path = img_path(app);
        app.main_window().capture_frame(file_path);
    }

    draw.to_frame(app, &frame).unwrap();
}
