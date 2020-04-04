use nannou::prelude::*;
use nannou::ease::cubic::ease_in_out;
use nannou::math::{Basis2, Rad};
use nannou::color::white_point::D65;
use nannou::color::{Alpha, Lab, Laba};

pub fn captured_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
    // Create a path that we want to save this frame to.
    app.project_path()
        .expect("failed to locate `project_path`")
        // Capture all frames to a directory called `/<path_to_nannou>/nannou/simple_capture`.
        .join(app.exe_name().unwrap())
        // Name each file after the number of the frame.
        .join(format!("image_{:03}", frame.nth()))
        // The extension will be PNG. We also support tiff, bmp, gif, jpeg, webp and some others.
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