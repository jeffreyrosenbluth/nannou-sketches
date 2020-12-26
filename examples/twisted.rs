use nannou::noise::NoiseFn;
use nannou::prelude::*;
use sketches::captured_frame_path;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let rect = app.window_rect();
    let nn = nannou::noise::OpenSimplex::new();
    if frame.nth() == 0 {
        draw.background().color(BLACK);
        let x0 = rect.left();
        let x1 = rect.right();
        let delta = (x1 - x0) / 500.0;
        let y0 = rect.bottom();
        let y1 = rect.top();
        let mut x = x0;
        let mut y = y0;
        let z = random();
        while x < x1 {
            let angle = nn.get([0.01 * x as f64, z]) as f32;
            let draw = draw.rotate(angle);
            draw.line()
                .points(pt2(x, y0), pt2(x, y1))
                .weight(1.0)
                .color(WHITE);
            draw.line()
                .points(pt2(x0, y), pt2(x1, y))
                .weight(1.0)
                .color(WHITE);
            x += delta;
            y += delta;
        }
        if frame.nth() == 0 {
            draw.rect()
                .wh(app.window_rect().wh())
                .color(srgba(0.0, 0.0, 0.0, 0.75));
            let file_path = captured_frame_path(app, &frame);
            app.main_window().capture_frame(file_path);
        }
    }
    draw.to_frame(app, &frame).unwrap();
}
