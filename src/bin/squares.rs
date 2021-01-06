// Inspired by Bees and Bombs:
// https://beesandbombs.tumblr.com/post/178493871934/squares-turning#notes

use nannou::ease::cubic::ease_in_out;
use nannou::prelude::*;
use sketches::img_path;

fn main() {
    nannou::app(model).update(update).run()
}

const SZ: u32 = 520;

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

fn update(app: &App, m: &mut Model, _update: Update) {
    let mut position = m.position.clone();
    let mut rotation = m.rotation.clone();
    let t = app.elapsed_frames() % 180;
    if t == 90 {
        for (i, row) in m.position.iter().enumerate() {
            for (j, p) in row.iter().enumerate() {
                let phase = if m.sq_color == BLACK { -1. } else { 1. };
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
    for (i, col) in m.position.iter().enumerate() {
        for (j, _p) in col.iter().enumerate() {
            let t = clock(app.elapsed_frames());
            rotation[i][j] = PI / 2. * t;
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
    // if app.elapsed_frames() < 360 {
    //     let file_path = captured_frame_path(app, &frame);
    //     app.main_window().capture_frame(file_path);
    // }
}

fn square(draw: &Draw, position: Point2, rot: f32, col: Rgb<u8>) {
    draw.rect()
        .w_h(40., 40.)
        .xy(position)
        .rotate(rot)
        .color(col);
}
