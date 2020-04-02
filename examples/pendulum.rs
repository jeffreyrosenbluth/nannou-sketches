use nannou::prelude::*;

fn clock(frame: u64) -> f32 {
    let rate = 360;
    (frame % rate) as f32 / rate as f32
}

fn main() {
    nannou::app(model).update(update).run()
}

fn update(app: &App, m: &mut Model, _update: Update) {
    let t = TAU * clock(app.elapsed_frames());
    m.x = 340. * t.cos();
    m.y = 375. / 2. * t.sin();
}

struct Model {
    x: f32,
    y: f32,
}

fn model(app: &App) -> Model {
    app.new_window().size(600, 600).view(view).build().unwrap();
    Model { x: 0., y: 0. }
}

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    let frames = app.elapsed_frames() % 360;
    let size = 2.5 * (300. - m.y).sqrt();
    if frames < 180 {
        bob(&draw, m.x, m.y, size)
    };
    // draw.rect().no_fill().stroke(WHITE).stroke_weight(20.).w_h(580., 580.,);
    stripe(&draw, -290., 20., app.window_rect().h());
    stripe(&draw, 290., 20., app.window_rect().h());
    stripe(&draw, 100., 20., app.window_rect().h());
    stripe(&draw, -100., 20., app.window_rect().h());
    if frames >= 180 {
        bob(&draw, m.x, m.y, size)
    };
    draw.ellipse()
        .color(DARKGOLDENROD)
        .w_h(50., 25.)
        .x_y(0., 300.);
    draw.to_frame(app, &frame).unwrap();
}

fn stripe(draw: &app::Draw, x: f32, w: f32, h: f32) {
    draw.rect().color(DIMGRAY).w_h(w, h).x_y(x + 3., 0.);
    draw.rect().color(WHITE).w_h(w, h).x_y(x, 0.);
}

fn bob(draw: &app::Draw, x: f32, y: f32, size: f32) {
    draw.line()
        .points(pt2(0., 300.), pt2(x, y))
        .color(DARKGOLDENROD)
        .stroke_weight(3.);
    draw.ellipse().w_h(size, size).color(ORANGE).x_y(x, y);
}
