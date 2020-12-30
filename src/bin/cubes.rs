use getopts::Options;
use nannou::app::LoopMode;
use nannou::prelude::*;
use std::env;

use sketches::captured_frame_path;

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 400;

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
    draw.background().color(BLACK);

    let p = pt2(-(WIDTH as f32) / 2.0 - 10.0, 50.0);
    let mut points = vec![p];
    let mut i = 0;
    let r0 = 25;
    let r1 = 90; 
    loop {
        let q = random_range(r0, r1) as f32;
        let s = if i % 2 == 0 { 1.0 } else { -1.0 };
        if points[i].x >= WIDTH as f32 / 2.0 {
            break;
        }
        points.push(points[i] + pt2(q, s * q));
        i += 1;
    }
    let pts2 = points.clone().into_iter().map(|p| pt2(p.x, p.y - 100.0));
    let points2: Vec<Point2> = pts2.collect();
    let mut points3 = vec![pt2(-(WIDTH as f32) / 2.0, -(HEIGHT as f32) / 2.0)];
    for j in 0..i {
        if j % 2 == 0 {
            points3.push(points2[j])
        } else {
            points3.push(points2[j + 1] - points2[j] + points2[j - 1])
        }
    }
    points3.push(pt2(WIDTH as f32 / 2.0, -(HEIGHT as f32) / 2.0));
    let mut points0 = vec![pt2(-(WIDTH as f32) / 2.0, HEIGHT as f32 / 2.0)];
    let mut ps = points.clone();
    points0.append(&mut ps);
    points0.push(pt2(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0));

    let c = CORNSILK;
    let w = 1.5;
    draw.polygon()
        .color(c)
        .join_miter()
        .points(points0);
    draw.polyline()
        .color(c)
        .weight(w)
        .join_miter()
        .points(points2.clone());
    draw.polygon()
        .color(c)
        .join_miter()
        .points(points3);
    for j in 0..=i {
        draw.line()
            .points(points[j], points2[j])
            .weight(w)
            .color(c);
    }

    if png {
        let file_path = captured_frame_path(app, &frame);
        app.main_window().capture_frame(file_path);
    }

    draw.to_frame(app, &frame).unwrap();
}
