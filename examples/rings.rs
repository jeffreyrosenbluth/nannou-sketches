use nannou::noise::NoiseFn;
use nannou::prelude::*;

fn main() {
    nannou::app(model).run()
}

// fn update(app: &App, m: &mut Model, _update: Update) {}

struct Model {}

fn model(app: &App) -> Model {
    app.new_window().size(600, 600).view(view).build().unwrap();
    Model {}
}

fn view(app: &App, _m: &Model, frame: Frame) {
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
            let z = app.elapsed_frames() as f64 * 0.003;
            let offset = nn.get([x as f64, y as f64 + j as f64 * 0.03, z]) as f32;
            x *= rad + 2. * offset * rad;
            y *= rad + 2. * offset * rad;
            ps.push(pt2(x, y));
        }
        let alpha = j as f32 / rings as f32;
        draw.polyline().points(ps).color(with_opacity(ORANGE, alpha)).x_y(0., 0.);
    }
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
