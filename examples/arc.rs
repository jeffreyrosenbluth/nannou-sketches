use nannou::prelude::*;
use nannou::draw::Drawing;
use nannou::draw::primitive::Path;
use nannou::color::conv::IntoLinSrgba;

const WIDTH: f32 = 1200.0;
const HEIGHT: f32 = 900.0;
fn main() {
    nannou::sketch(view).size(WIDTH as u32, HEIGHT as u32).run()
}

fn arc<C> (
    draw: &Draw,
    start_deg: f32,
    angle_deg: f32,
    radius: f32,
    color: C,
    weight: f32
) -> Drawing< Path<f32>, f32> where C: IntoLinSrgba<f32> {
// ) -> DrawingPath {
    let start = start_deg as usize;
    let end = start + angle_deg as usize;
    let pts = (start..=end).map(|i| {
        let theta = i as f32 / 360.0 * TAU;
        pt2(radius * theta.cos(), radius * theta.sin())
    });
    draw.polyline().join_round().color(color).weight(weight).points(pts)
}

fn view(app: &App, frame: Frame) {
    if frame.nth() > 0 {
        return;
    };

    let draw = app.draw();
    draw.background().color(WHITE);
    arc(&draw, 180.0, 45.0, 200.0, BLUE, 4.0).x_y(-100.0, -100.0);
    draw.to_frame(app, &frame).unwrap();
}
