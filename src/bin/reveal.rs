use nannou::geom::path::Builder;
use nannou::prelude::*;
use sketches::{captured_frame_path, with_opacity};

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    rects: Vec<Rect>,
    alpha: f32,
    // texture: wgpu::Texture,
}

fn model(app: &App) -> Model {
    app.new_window().size(800, 800).view(view).build().unwrap();
    Model {
        rects: Vec::new(),
        alpha: 1.0,
    }
}

fn update(app: &App, m: &mut Model, _update: Update) {
    let rect = app.window_rect();
    let w2 = rect.w() / 2.;
    let h2 = rect.h() / 2.;
    let w = random_range(20., 100.);
    let h = random_range(20., 100.);
    let x = random_range(-w2, w2);
    let y = random_range(-h2, h2);
    m.rects.push(Rect::from_x_y_w_h(x, y, w, h));
    if m.rects.len() > 1000 {
        m.rects.remove(0);
    }
    m.alpha -= 1. / 360.;
    m.alpha = if m.alpha <= 0.0 { 0.0 } else { m.alpha };
}

fn cutout(builder: Box<Builder>, r: Rect) -> Builder {
    builder
        .move_to(r.top_left())
        .line_to(r.top_right())
        .line_to(r.bottom_right())
        .line_to(r.bottom_left())
        .line_to(r.top_left())
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let rect = app.window_rect();
    let step_x = 5.;
    let step_y = 5.;

    let size = vec2(step_x, step_y);
    let r = nannou::geom::Rect::from_wh(size)
        .align_left_of(rect)
        .align_top_of(rect);
    let mut grid_y = 0.0;
    while grid_y < rect.h() {
        let mut grid_x = 0.0;
        while grid_x < rect.w() {
            let r = r.shift_x(grid_x).shift_y(-grid_y);
            let hue = grid_x / rect.w();
            let saturation = 1.0 - (grid_y / rect.h());
            draw.rect().xy(r.xy()).wh(r.wh()).hsl(hue, saturation, 0.5);
            grid_x += step_x;
        }
        grid_y += step_y;
    }
    let mut builder = Builder::new();
    let w2 = rect.w() / 2.;
    let h2 = rect.h() / 2.;

    builder = builder.move_to(pt2(-w2, h2));
    builder = builder.line_to(pt2(w2, h2));
    builder = builder.line_to(pt2(w2, -h2));
    builder = builder.line_to(pt2(-w2, -h2));
    for r in model.rects.iter() {
        builder = cutout(Box::new(builder), *r);
    }
    builder = builder.close();
    let p = builder.build();

    // draw arc
    draw.path()
        .fill()
        .color(with_opacity(WHITE, model.alpha))
        .events(p.iter());

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();

    // if app.elapsed_frames() < 360 {
    //     let file_path = captured_frame_path(app, &frame);
    //     app.main_window().capture_frame(file_path);
    // }
}
