use nannou::ease::cubic::ease_in_out;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run()
}

fn update(app: &App, m: &mut Model, _update: Update) {}

struct Model {}

fn model(app: &App) -> Model {
    app.new_window().size(600, 600).view(view).build().unwrap();
    Model {}
}

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);
    draw.to_frame(app, &frame).unwrap();
}
