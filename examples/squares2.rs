use nannou::ease::cubic::ease_in_out;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run()
}

const SZ: u32 = 527;

#[derive(Debug)]
struct Model {
    position: [[Point2; 11]; 11],
    rotation: [[f32; 11]; 11],
    direction: [[Point2; 11]; 11],
    sq_color: Rgb<u8>,
    bg_color: Rgb<u8>,
}

fn clock(frame: u64) -> f32 {
    let t = (frame % 180) as f32 / 180.;
    ease_in_out(t, 0., 1., 1.)
}

fn model(app: &App) -> Model {
    app.new_window().size(SZ, SZ).view(view).build().unwrap();
    let mut position = [[pt2(0., 0.); 11]; 11];
    let mut direction = [[pt2(0., 0.); 11]; 11];
    let mut rotation = [[0.; 11]; 11];
    let xs = -5..=5;
    let xs = xs.map(|x| x as f32 * 56.57);
    let ys = xs.clone();
    for (i, x) in xs.enumerate() {
        for (j, y) in ys.clone().enumerate() {
            position[i][j] = pt2(x, y);
            rotation[i][j] = 0.;
            direction[i][j] = if i % 2 == 0 {
                pt2(0., 1.)
            } else {
                pt2(0., -1.)
            }
        }
    }
    let bg_color = BLACK;
    let sq_color = WHITE;
    Model {
        position,
        rotation,
        direction,
        sq_color,
        bg_color,
    }
}

fn next_dir(d: Point2) -> Point2 {
    match d {
        Point2 { x: 0., y: 1. } => pt2(1., 0.),
        Point2 { x: 0., y: -1. } => pt2(-1., 0.),
        Point2 { x: 1., y: 0. } => pt2(0., -1.),
        Point2 { x: -1., y: 0. } => pt2(0., 1.),
        _ => pt2(0., 0.),
    }
}

fn borders(width: f32, height: f32, p: Point2) -> Point2 {
    let l = -width / 2.;
    let r = width / 2.;
    let t = height / 2.;
    let b = -height / 2.;
    let mut u = p.x;
    let mut v = p.y;
    let gap = 16.57;
    match p {
        Vector2 { x, .. } if x < l - gap => u = r + gap,
        Vector2 { y, .. } if y < b - gap => v = t + gap,
        Vector2 { x, .. } if x > r + gap => u = l - gap,
        Vector2 { y, .. } if y > t + gap => v = b - gap,
        _ => (),
    };
    pt2(u, v)
}

fn update(app: &App, m: &mut Model, _update: Update) {
    // if app.elapsed_frames() >= 92 { return };
    let n_frames = (app.elapsed_frames() % 180) as f32;
    let shift = 56.57 / 180.;
    let mut position = m.position.clone();
    let mut rotation = m.rotation.clone();
    let t = app.elapsed_frames() % 180;
    if t == 90 {
    for (i, row) in m.position.iter().enumerate() {
        for (j, p) in row.iter().enumerate() {
            let phase = if m.sq_color == BLACK {-1.} else {1.};
            position[i][j] = pt2(p.x + phase * 56.57 / 2., p.y + phase * 56.57 / 2.);
        }
    }
        if m.sq_color == WHITE {
            m.sq_color = BLACK;
            m.bg_color = WHITE;
        } else {
            m.sq_color = WHITE;
            m.bg_color = BLACK;
        }
    }
    for (i, row) in m.position.iter().enumerate() {
        for (j, p) in row.iter().enumerate() {
            let t = clock(app.elapsed_frames());
            rotation[i][j] = PI / 2. * t;
            // if i % 2 == 0 && t < 0.5 {
            //     rotation[i][j] = -PI / 2. * t;
            //     position[i][j] = *p + pt2(0., shift);
            // } else if i % 2 == 1 && t < 0.5 {
            //     rotation[i][j] = PI / 2. * t;
            //     position[i][j] = *p + pt2(0., -shift);
            // } else if j % 2 == 0 && t >= 0.5 {
            //     rotation[i][j] = PI / 2. * t;
            //     position[i][j] = *p + pt2(shift, 0.);
            // } else {
            //     rotation[i][j] = -PI / 2. * t;
            //     position[i][j] = *p + pt2(-shift, 0.);
            // }
            position[i][j] = borders(SZ as f32, SZ as f32, position[i][j]);
        }
    }
    m.position = position;
    m.rotation = rotation;
}

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(m.bg_color);
    for (i, row) in m.position.iter().enumerate() {
        for (j, p) in row.iter().enumerate() {
            square(&draw, *p, m.rotation[i][j], m.sq_color);
        }
    }
    draw.to_frame(app, &frame).unwrap();
}

fn square(draw: &app::Draw, position: Point2, rot: f32, col: Rgb<u8>) {
    draw.rect()
        .w_h(40., 40.)
        .xy(position)
        .rotate(rot)
        .color(col);
}
