use nannou::noise::NoiseFn;
use nannou::prelude::*;
use sketches::img_path;

fn main() {
    nannou::sketch(view).run()
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let rect = app.window_rect();
    let nn = nannou::noise::OpenSimplex::new();
    draw.background().color(BLACK);
    let rings = 50;
    let r = rect.w() * 0.35;
    for j in 0..rings {
        let mut ps: Vec<Point2> = vec![];
        let rad = (r / rings as f32) * (rings - j) as f32;
        for i in 0..150 {
            let mut x = (i as f32 / 100. * TAU).cos();
            let mut y = (i as f32 / 100. * TAU).sin();
            let a = app.elapsed_frames() % 360;
            let a = (a as f64) * PI_F64 / 180.;
            let u = 0.3 * a.cos();
            let v = 0.3 * a.sin();
            let offset = nn.get([x as f64, y as f64 + j as f64 * 0.03, u, v]) as f32;
            x *= rad + 2. * offset * rad;
            y *= rad + 2. * offset * rad;
            ps.push(pt2(x, y));
        }
        let alpha = j as f32 / rings as f32;
        draw.path()
            .stroke()
            .points(ps)
            .color(with_opacity(ORANGE, alpha))
            .x_y(0., 0.);
    }
    // if app.elapsed_frames() < 360 {
    //     let file_path = captured_frame_path(app, &frame);
    //     app.main_window().capture_frame(file_path);
    // }
    draw.to_frame(app, &frame).unwrap();
}

fn with_opacity(c: nannou::color::Srgb<u8>, o: f32) -> nannou::color::rgb::Srgba {
    srgba(
        c.red as f32 / 255.,
        c.green as f32 / 255.,
        c.blue as f32 / 255.,
        o,
    )
}
