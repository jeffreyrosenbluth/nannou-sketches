use getopts::Options;
use nannou::prelude::*;
use std::env;

use sketches::img_path;

const WIDTH: f32 = 1200.0;
const HEIGHT: f32 = 900.0;

fn main() {
    nannou::sketch(view).size(WIDTH as u32, HEIGHT as u32).run()
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

    let draw = draw.x_y(100.0, -50.0);

    let rings = 150;
    for i in 0..rings {
        let r = (rings - i) as f32 * 5.0;
        let alpha = 1.0 - i as f32 / rings as f32;
        let alpha = (alpha * 8.0 * TAU).sin() / (8.0 * alpha);
        let alpha = map_range(alpha, 0.0, 1.0, 0.0, 0.4);
        draw.ellipse()
            .color(srgba(1.0 - alpha, alpha, 1.0 - alpha, alpha))
            .w_h(1.4 * r, r);
        draw.ellipse()
            .color(srgba(0.0, 0.0, 0.0, 0.6))
            .w_h(60.0, 40.0);
        let offset = 100.0;
        draw.rect()
            .color(srgba(alpha, alpha, alpha, alpha))
            .x_y(-WIDTH / 2.0 + offset, HEIGHT / 2.0 - offset - 50.0)
            .w_h(1.25 * r, 1.25 * r);
        draw.ellipse()
            .color(srgba(0.0, 0.0, 0.0, 0.6))
            .w_h(50.0, 50.0)
            .x_y(-WIDTH / 2.0 + offset, HEIGHT / 2.0 - offset - 50.0);
        let offset = 800.0;
        draw.rect()
            .color(srgba(alpha, alpha, alpha, alpha))
            .x_y(-WIDTH / 2.0 + offset + 200.0, HEIGHT / 2.0 - offset - 50.0)
            .w_h(1.25 * r, 1.25 * r);
        draw.ellipse()
            .color(srgba(0.0, 0.0, 0.0, 0.6))
            .w_h(50.0, 50.0)
            .x_y(-WIDTH / 2.0 + offset + 200.0, HEIGHT / 2.0 - offset - 50.0);
    }

    if png {
        let file_path = img_path(app);
        app.main_window().capture_frame(file_path);
    }

    draw.to_frame(app, &frame).unwrap();
}
