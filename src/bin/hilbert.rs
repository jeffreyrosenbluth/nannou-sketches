use nannou::geom::range::Range;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run()
}

#[derive(Debug)]
struct Model {
    points: Vec<Point2>,
    count: usize,
    scale: i32,
}

fn generate(ps: Vec<Point2>) -> Vec<Point2> {
    let n = ps.len();
    let m = (n as f32).sqrt();
    let mut result: Vec<Point2> = vec![pt2(0., 0.); 4 * n];
    for (i, p) in ps.iter().enumerate() {
        result[i] = pt2(p.y, p.x);
        result[i + n] = pt2(p.x, p.y + m);
        result[i + 2 * n] = pt2(p.x + m, p.y + m);
        result[i + 3 * n] = pt2(2. * m - 1. - p.y, m - 1. - p.x);
    }
    result
}

fn model(app: &App) -> Model {
    app.new_window().size(800, 800).view(view).build().unwrap();
    let points = vec![
        pt2(-0.5, -0.5),
        pt2(-0.5, 0.5),
        pt2(0.5, 0.5),
        pt2(0.5, -0.5),
    ];
    let count = 4;
    let scale = 0;
    Model {
        points,
        count,
        scale,
    }
}

fn update(_app: &App, m: &mut Model, _update: Update) {
    let k = pow(2, 9);
    if m.count == m.points.len() + 5 {
        if m.scale < 1 {
            m.scale = k / 4;
        } else {
            m.scale /= 2;
            m.points = generate(m.points.clone());
        }
        m.count = 1;
    } else {
        m.count += 1;
    }
}

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();
    // let win = app.window_rect();
    let k = pow(2, 9) as f32;
    if frame.nth() == 0 {
        frame.clear(BLACK);
    }
    if m.count == 1 {
        // draw.lines(pt2(-k / 2., k / 2.), pt2(k / 2., k / 2.));
    } else if m.count < m.points.len() {
        let p1 = m.points[m.count - 1];
        let p2 = m.points[m.count];
        let r = Range::new(m.scale as f32, 3. * m.scale as f32);
        draw.line()
            .weight(6.0)
            .caps_round()
            .color(ORANGE)
            .x_y(-k / 2., -k / 2.)
            .points(
                pt2(r.lerp(p1.x), r.lerp(p1.y)),
                pt2(r.lerp(p2.x), r.lerp(p2.y)),
            );
    }
    draw.to_frame(app, &frame).unwrap();
}
