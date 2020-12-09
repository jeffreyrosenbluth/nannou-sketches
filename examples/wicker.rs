use nannou::noise::NoiseFn;
use nannou::prelude::*;
use primes::is_prime;
use sketches::captured_frame_path;
use sketches::with_opacity;

const H: f32 = 900.0;
const W: f32 = 1200.0;
fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    slope: f32,
    b: f32,
    thickness: f32,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(W as u32, H as u32)
        .view(view)
        .build()
        .unwrap();
    Model {
        slope: 0.0,
        b: -H / 2.0,
        thickness: 10.0,
    }
}

fn update(app: &App, m: &mut Model, _update: Update) {
    let nn = nannou::noise::Perlin::new();
    let a = app.elapsed_frames();
    let offset = nn.get([a as f64 / 70., 0.0]) as f32;
    m.b += 10.0;
    if m.b > H / 2.0 {
        m.b = -H / 2.0;
    }
    m.slope = offset;
    m.thickness = 5.0 + (nn.get([a as f64 / 70., 0.137]) as f32 + 1.0) * 7.0;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let rect = app.window_rect();
    let x0 = rect.left();
    let x1 = rect.right();
    let y0 = model.b + model.slope * x0;
    let y1 = model.b + model.slope * x1;
    if frame.nth() == 0 {
        draw.background().color(CORNSILK);
    }
    let c = if is_prime(frame.nth()) {
        with_opacity(GOLDENROD, 0.075)
    } else {
        with_opacity(BLACK, 0.1)
    };
    draw.line()
        .points(pt2(x0, y0), pt2(x1, y1))
        .weight(model.thickness)
        .color(c);
    draw.to_frame(app, &frame).unwrap();

    // if frame.nth() % 80 == 0 && frame.nth() < 360 * 80 {
    //     let file_path = captured_frame_path(app, &frame);
    //     app.main_window().capture_frame(file_path);
    // }
}
