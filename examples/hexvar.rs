use nannou::prelude::*;
use nannou::app::LoopMode;
use sketches::{arc, captured_frame_path};

const SIZE: f32 = 25.0;
const WIDTH: f32 = 900.0;
const HEIGHT: f32 = 900.0;

fn main() {
    nannou::sketch(view).size(WIDTH as u32, HEIGHT as u32).run()
}

fn view(app: &App, frame: Frame) {
    app.set_loop_mode(LoopMode::loop_once());
    let h = (PI / 3.0).sin() * SIZE;
    let width2 = WIDTH / 2.0;
    let height2 = HEIGHT / 2.0;
    let draw = app.draw();
    draw.background().color(BLACK);
    for i in 0..=(WIDTH / (SIZE * 3.0)) as usize {
        for j in 0..=(HEIGHT / h) as usize + 1 {
            let mut x = i as f32 * SIZE * 3.0 + (SIZE / 2.0) - width2;
            let y = j as f32 * h - height2;
            if j % 2 > 0 {
                x += SIZE * 1.5;
            }
            let angle = random_range(0, 3) as f32 * PI / 3.0;
            let d = draw.translate(vec3(x, y, 0.0));
            let d = d.rotate(angle);
            d.line()
                .points(pt2(0.0, -h), pt2(0.0, h))
                .color(WHITE)
                .stroke_weight(2.0);
            arc(&d, 300.0, 120.0, SIZE / 2.0, WHITE, 2.0).x_y(-SIZE, 0.0);
            arc(&d, 120.0, 120.0, SIZE / 2.0, WHITE, 2.0).x_y(SIZE, 0.0);
        }
    }
    let file_path = captured_frame_path(app, &frame);
    app.main_window().capture_frame(file_path);
    draw.to_frame(app, &frame).unwrap();
}

