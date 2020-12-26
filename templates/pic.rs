use nannou::prelude::*;
use nannou::app::LoopMode;
use sketches::captured_frame_path;

const WIDTH: u32 = 900;
const HEIGHT: u32 = 600;

fn main() {
    nannou::sketch(view).size(WIDTH, HEIGHT).run()
}

fn view(app: &App, frame: Frame) {
    app.set_loop_mode(LoopMode::loop_once());
    let draw = app.draw();
    draw.background().color(PLUM);

    let file_path = captured_frame_path(app, &frame);
    app.main_window().capture_frame(file_path);

    draw.to_frame(app, &frame).unwrap();
}