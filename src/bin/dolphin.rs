use nannou::color::IntoLinSrgba;
use nannou::math::{Basis2, Rad};
use nannou::prelude::*;
use sketches::img_path;

fn main() {
    nannou::app(model).update(update).run()
}

#[derive(Debug)]
struct Model {
    bg_angle: f32,
    sm_angle: f32,
}

fn model(app: &App) -> Model {
    app.new_window().size(603, 603).view(view).build().unwrap();
    Model {
        bg_angle: 0.,
        sm_angle: 0.,
    }
}

fn update(_app: &App, m: &mut Model, _update: Update) {
    m.bg_angle += 1. / 720.;
    m.sm_angle += 1. / 360.;
}

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(LIGHTGRAY);
    circles(&draw, m.sm_angle + 1. / 12., INDIGO, 70.);
    grid(&draw);
    circles(&draw, m.bg_angle, with_opacity(PURPLE, 0.98), 80.);
    draw.to_frame(app, &frame).unwrap();

    // if app.elapsed_frames() < 120 {
    //     let file_path = captured_frame_path(app, &frame);
    //     app.main_window().capture_frame(file_path);
    // }
}

fn rotate_pt(p: Point2<f32>, turn: f32) -> Point2<f32> {
    let rad = Rad(turns_to_rad(turn));
    let rot: Basis2<f32> = Rotation2::from_angle(rad);
    let q = rot.rotate_point(p.into());
    pt2(q.x, q.y)
}

fn circles<C>(draw: &Draw, angle: f32, c: C, s: f32)
where
    C: Copy + IntoLinSrgba<f32>,
{
    let angles = 0..6;
    let angles = angles.map(|a| a as f32 / 6.);
    let angles = angles.map(|x| x + angle);
    let pts = angles.map(|a| rotate_pt(pt2(0., 200.), a));
    for p in pts {
        draw.ellipse()
            .color(c)
            .w_h(s, s)
            .xy(p)
            .stroke_weight(2.)
            .stroke(WHITE);
    }
}

fn with_opacity(c: nannou::color::Srgb<u8>, o: f32) -> nannou::color::rgb::Srgba {
    srgba(
        c.red as f32 / 255.,
        c.green as f32 / 255.,
        c.blue as f32 / 255.,
        o,
    )
}

fn grid(draw: &Draw) {
    let spacing = 50.;
    let sz = 6;
    let end = sz as f32 * spacing;
    let xs = -sz..=sz;
    let xs = xs.map(|x| x as f32 * spacing);
    let gray = rgb(0.2, 0.2, 0.2);
    for x in xs {
        draw.line()
            .points(pt2(x + 3., -end), pt2(x + 3., end))
            .color(gray)
            .stroke_weight(5.)
            .caps_square();
        draw.line()
            .points(pt2(x, -end), pt2(x, end))
            .color(DIMGRAY)
            .caps_square()
            .stroke_weight(5.);
        draw.line()
            .points(pt2(-end, x - 3.), pt2(end, x - 3.))
            .color(gray)
            .caps_square()
            .stroke_weight(5.);
        draw.line()
            .points(pt2(-end, x), pt2(end, x))
            .color(DIMGRAY)
            .caps_square()
            .stroke_weight(5.);
    }
}
