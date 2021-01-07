use colorous;
use getopts::Options;
use nannou::{app::LoopMode, color::IntoLinSrgba};
use nannou::prelude::*;
use nannou::color::{Alpha, white_point::D65, Lab, Laba};
use std::env;
use hex;

use sketches::{img_path, with_opacity, random_rgb,set_opacity};

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 900;
const WHEELS: usize = 30;

fn main() {
    nannou::sketch(view).size(WIDTH, HEIGHT).run()
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
    draw.background().color(WHITE);
    let colors = colorous::PURPLE_ORANGE;

    let cs = ["3e1618","ddefb3","3e3731","a4b3c5","ab616e"];
    let cs4 = ["03071e","370617","6a040f","9d0208","3c5233","6f732f","e85d04","7067cf","bc5f04","7c72a0"];
    // let cs = ["03071e","370617","6a040f","9d0208","3c5233","6f732f","e85d04","7067cf","bc5f04","7c72a0"];
    
    let mut rgbs = vec![];
    for s in &cs {
        let q = hex::decode(s).unwrap();
        rgbs.push(rgb8(q[0], q[1], q[2]));
    }

    for i in 0..WHEELS {
        let x = random_range(-(WIDTH as f32 / 2.0), WIDTH as f32 / 2.0);
        let y = random_range(-(HEIGHT as f32 / 2.0), HEIGHT as f32 / 2.0);
        let r = random_range(50.0, 200.0);
        let h = random_range(0.20, 0.4);
        let s = random_range(4.0, 27.0);
        // let c = colors.eval_rational(i, WHEELS);
        let c = rgbs[i % rgbs.len()]; 
        // let kolor = srgb8(c.r, c.g, c.b);
        // let c = random_color2();
        wheel(&draw, r, h, s, pt2(x, y), c.into_lin_srgba());
    }

    if png {
        let file_path = img_path(app);
        app.main_window().capture_frame(file_path);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn wheel(draw: &Draw, radius: f32, hub: f32, spokes: f32, pos: Point2, c: LinSrgba) {
    let clear = srgba(0.0, 0.0, 0.0, 0.0);
    let a = set_opacity(c, 0.4);
    let diameter = 2.0 * radius;
    let d = hub * radius;
    let mut angle = 0.0;
    let sw = random_range(1.0, 4.0);
    while angle < TAU {
        draw.line()
            .points(
                // pos,
                pt2(
                    pos.x + hub * radius / 2.0 * angle.cos(),
                    pos.y + hub * radius / 2.0 * angle.sin(),
                ),
                pt2(pos.x + radius * angle.cos(), pos.y + radius * angle.sin()),
            )
            .color(c)
            .stroke_weight(sw);
        angle += TAU / spokes;
    }
    draw.ellipse()
        .xy(pos)
        .color(clear)
        .w_h(diameter, diameter)
        .stroke_weight(10.0)
        .stroke_color(c);
    draw.ellipse()
        .xy(pos)
        .color(clear)
        .w_h(diameter - 12.0, diameter - 12.0)
        .stroke_weight(4.0)
        .stroke_color(GRAY);
    draw.ellipse().xy(pos).color(a).w_h(d, d);
    let c = with_opacity(BLACK, 0.75);
    draw.ellipse().xy(pos).color(c).w_h(10.0, 10.0);
}
