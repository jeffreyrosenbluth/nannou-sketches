use nannou::{color::white_point::D65, draw::{Drawing, primitive::Path}};
use nannou::color::{Alpha, IntoLinSrgba, Lab, Laba};
use nannou::ease::cubic::ease_in_out;
use nannou::math::{Basis2, Rad};
use nannou::prelude::*;

pub fn captured_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
    app.project_path()
        .expect("failed to locate `project_path`")
        .join("img")
        .join(format!("{}_{:03}", app.exe_name().unwrap(), frame.nth()))
        .with_extension("png")
}

pub fn clock(frame: u64) -> f32 {
    let t = (frame % 180) as f32 / 180.;
    ease_in_out(t, 0., 1., 1.)
}

pub fn random_color() -> Alpha<Lab<D65, f32>, f32> {
    let l: f32 = random_range(0.0, 100.0);
    let a: f32 = random_range(-128.0, 127.0);
    let b: f32 = random_range(-128.0, 127.0);
    let o: f32 = random_range(0.5, 1.0);
    Laba::new(l, a, b, o)
}

pub fn with_opacity(c: nannou::color::Srgb<u8>, o: f32) -> nannou::color::rgb::Srgba {
    srgba(
        c.red as f32 / 255.,
        c.green as f32 / 255.,
        c.blue as f32 / 255.,
        o,
    )
}

pub fn rotate_pt(p: Point2<f32>, turn: f32) -> Point2<f32> {
    let rad = Rad(turns_to_rad(turn));
    let rot: Basis2<f32> = Rotation2::from_angle(rad);
    let q = rot.rotate_point(p.into());
    pt2(q.x, q.y)
}

pub fn circle_mask<T>(draw: &Draw, width: f32, height: f32, radius: f32, color: T)
where
    T: IntoLinSrgba<f32>,
{
    use nannou::geom::path::Builder;
    let mut builder = Builder::new();
    let w2 = width / 2.;
    let h2 = height / 2.;
    builder = builder.move_to(pt2(-w2, h2));
    builder = builder.line_to(pt2(w2, h2));
    builder = builder.line_to(pt2(w2, -h2));
    builder = builder.line_to(pt2(-w2, -h2));
    builder = builder.move_to(pt2(0., -radius));
    builder = builder.arc(pt2(0., 0.), vec2(radius, radius), TAU, 0.);
    builder = builder.close();

    let p = builder.build();

    // draw arc
    draw.path().fill().color(color).events(p.iter());
}

pub fn arc<C> (
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