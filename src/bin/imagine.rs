use nannou::{color::IntoLinSrgba, draw::theme};
use nannou::image;
use nannou::image::GenericImageView;
use nannou::prelude::*;
use sketches::img_path;
use std::env;

fn main() {
    nannou::app(model).run();
}

enum Style {
    NegY,
    NegYup,
    NegYright,
    NegYleft,
    PosY,
    PosYup,
    PosYright,
    PosYleft,
    Line,
    Miter,
    CircleIn,
    CircleOut,
}
struct Model {
    image: image::DynamicImage,
    texture: wgpu::Texture,
    style: Style,
}

fn model(app: &App) -> Model {
    let args: Vec<String> = env::args().collect();
    let filename;
    if args.len() != 2 {
        panic! {"{}","Must provide a filename argument"};
    } else {
        filename = args[1].clone();
    }
    let assets = app.assets_path().unwrap();
    let img_path = assets.join(filename);
    let image = image::open(&img_path).unwrap();
    let w = image.width();
    let h = image.height();
    app.new_window()
        .size(w, h)
        .view(view)
        .key_pressed(key_pressed)
        .key_released(key_released)
        .build()
        .unwrap();
    app.set_loop_mode(LoopMode::Wait);
    let texture = wgpu::Texture::from_path(app, &img_path).unwrap();
    Model {
        texture,
        image,
        style: Style::NegY,
    }
}

// Draw the state of your `Model` into the given `Frame` here.
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    frame.clear(BLACK);

    let width = model.image.width();
    let height = model.image.height();
    let w32 = width as f32;
    let h32 = height as f32;

    draw.texture(&model.texture);

    let t = height as f32 / 2.0;
    let b = -t;
    for img_x in 0..width {
        let x = img_x as f32 - w32 / 2.0;
        let y = h32 / w32 * x;
        match model.style {
            Style::NegY => {
                let img_y = map_range(y, b, t, 0.0, h32 - 1.0) as u32;
                let c = mk_srgb(model.image.get_pixel(img_x, img_y));
                draw.line().color(c).points(pt2(x, -y), pt2(x, -h32 / 2.0));
            }
            Style::NegYup => {
                let img_y = map_range(y, b, t, 0.0, h32 - 1.0) as u32;
                let c = mk_srgb(model.image.get_pixel(img_x, img_y));
                draw.line().color(c).points(pt2(x, -y), pt2(x, h32 / 2.0));
            }
            Style::NegYright => {
                let img_y = map_range(y, b, t, 0.0, h32 - 1.0) as u32;
                let c = mk_srgb(model.image.get_pixel(img_x, img_y));
                draw.line().color(c).points(pt2(x, -y), pt2(w32, -y));
            }
            Style::NegYleft => {
                let img_y = map_range(y, b, t, 0.0, h32 - 1.0) as u32;
                let c = mk_srgb(model.image.get_pixel(img_x, img_y));
                draw.line().color(c).points(pt2(x, -y), pt2(-w32, -y));
            }
            Style::PosY => {
                let img_y = map_range(-y, b, t, 0.0, h32 - 1.0) as u32;
                let c = mk_srgb(model.image.get_pixel(img_x, img_y));
                draw.line().color(c).points(pt2(x, y), pt2(x, -h32 / 2.0));
            }
            Style::PosYup => {
                let img_y = map_range(-y, b, t, 0.0, h32 - 1.0) as u32;
                let c = mk_srgb(model.image.get_pixel(img_x, img_y));
                draw.line().color(c).points(pt2(x, y), pt2(x, h32 / 2.0));
            }
            Style::PosYright => {
                let img_y = map_range(-y, b, t, 0.0, h32 - 1.0) as u32;
                let c = mk_srgb(model.image.get_pixel(img_x, img_y));
                draw.line().color(c).points(pt2(x, y), pt2(w32, y));
            }
            Style::PosYleft => {
                let img_y = map_range(-y, b, t, 0.0, h32 - 1.0) as u32;
                let c = mk_srgb(model.image.get_pixel(img_x, img_y));
                draw.line().color(c).points(pt2(x, y), pt2(-w32, y));
            }
            Style::Line => {
                let img_y = map_range(y, b, t, 0.0, h32 - 1.0) as u32;
                let c = mk_srgb(model.image.get_pixel(img_x, img_y));
                draw.line()
                    .color(c)
                    .points(pt2(x, -h32 / 2.0), pt2(x, h32 / 2.0));
            }
            Style::Miter => {
                let img_y = map_range(y, b, t, 0.0, h32 - 1.0) as u32;
                let c = mk_srgb(model.image.get_pixel(img_x, img_y));
                draw.line()
                    .color(c)
                    .points(pt2(x, -y), pt2(x, -h32 / 2.0));
                draw.line()
                    .color(c)
                    .points(pt2(x, -y), pt2(w32, -y));
            }
            Style::CircleIn => {
                let diameter = height / 3 * 2;
                let r = diameter as f32 / 2.0;
                let n = (diameter as f32 * PI) as u32;
                for t in 0..n / 2 {
                    let theta = 2.0 * t as f32 / n as f32 * TAU;
                    let p = pt2(r * theta.cos(), r * theta.sin());
                    let img_x = (r * theta.cos()) as u32 + width / 2;
                    let img_y = (-r * theta.sin()) as u32 + height / 2;
                    let c = mk_srgb(model.image.get_pixel(img_x, img_y));
                    draw.line().color(c).weight(2.0).points(pt2(0.0, 0.0), p);
                }
            }
            Style::CircleOut => {
                let diameter = height / 3 * 2;
                let r = diameter as f32 / 2.0;
                let n = (diameter as f32 * PI) as u32;
                for t in 0..n / 2 {
                    let theta = 2.0 * t as f32 / n as f32 * TAU;
                    let p = pt2(r * theta.cos(), r * theta.sin());
                    let q = pt2(3.0 * r * theta.cos(), 3.0 *  r * theta.sin());
                    let img_x = (r * theta.cos()) as u32 + width / 2;
                    let img_y = (-r * theta.sin()) as u32 + height / 2;
                    let c = mk_srgb(model.image.get_pixel(img_x, img_y));
                    draw.line().color(c).weight(4.0).points(p, q);
                }
            }
        }
    }
    draw.to_frame(app, &frame).unwrap();
}

fn mk_srgb(c: image::Rgba<u8>) -> LinSrgba {
    let r = c[0];
    let g = c[1];
    let b = c[2];
    srgba8(r, g, b, 255).into_lin_srgba()
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Key1 => {
            model.style = Style::NegY;
        }
        Key::Key2 => {
            model.style = Style::NegYup;
        }
        Key::Key3 => {
            model.style = Style::NegYright;
        }
        Key::Key4 => {
            model.style = Style::NegYleft;
        }
        Key::Key5 => {
            model.style = Style::PosY;
        }
        Key::Key6 => {
            model.style = Style::PosYup;
        }
        Key::Key7 => {
            model.style = Style::PosYright;
        }
        Key::Key8 => {
            model.style = Style::PosYleft;
        }
        Key::Key9 => {
            model.style = Style::Line;
        }
        Key::Key0 => {
            model.style = Style::Miter;
        }
        Key::Q => {
            model.style = Style::CircleIn;
        }
        Key::W => {
            model.style = Style::CircleOut;
        }
        _otherkey => (),
    }
}

fn key_released(app: &App, _model: &mut Model, key: Key) {
    match key {
        Key::S => {
            let file_path = img_path(app);
            app.main_window().capture_frame(file_path);
        }
        _otherkey => (),
    }
}
