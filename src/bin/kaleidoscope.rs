use nannou::color::white_point::D65;
use nannou::color::{Alpha, Lab, Laba};
use nannou::math::{Basis2, Rad};
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run()
}

struct Settings {
    size: u32,
    dots: u32,
    mirrors: usize,
}

impl Settings {
    fn new() -> Self {
        let size = 900;
        let dots = 75;
        let mirrors = 10;
        Settings {
            size,
            dots,
            mirrors,
        }
    }
}

struct Model {
    balls: Vec<Vec<Ball>>,
}

#[derive(Clone)]
struct Ball {
    position: Point2,
    velocity: Vector2,
    a: f32,
    b: f32,
    angle: f32,
    color: Alpha<Lab<D65, f32>, f32>,
}

impl Ball {
    fn new() -> Self {
        let position = pt2(0.0, 0.0);
        let velocity = vec2(1.0, 0.0);
        let a = 25.0;
        let b = 25.0;
        let angle = 0.0;
        let color = random_color();
        Ball {
            position,
            velocity,
            a,
            b,
            angle,
            color,
        }
    }

    fn update(&mut self) {
        self.position += self.velocity;
    }

    fn display(&self, draw: &Draw) {
        draw.ellipse()
            .xy(self.position)
            .color(self.color)
            .w_h(self.a, self.b)
            .z_turns(self.angle);
    }
}

fn random_ball() -> Ball {
    let mut ball = Ball::new();
    ball.position = pt2(random_range(-200.0, 200.0), random_range(-200.0, 200.0));
    ball.velocity = vec2(random(), random()).normalize_to(0.5);
    ball.color = random_color();
    ball.a = random_range(20.0, 100.0);
    let circle: bool = random();
    match circle {
        true => ball.b = ball.a,
        false => ball.b = random_range(20.0, 100.0),
    }
    ball
}

fn model(app: &App) -> Model {
    let settings = Settings::new();
    app.new_window()
        .size(settings.size, settings.size)
        .view(view)
        .build()
        .unwrap();
    let bs = (0..settings.dots).map(|_| random_ball());
    let gbs: Vec<Vec<Ball>> = bs.map(|b| mirror(b, settings.mirrors)).collect();
    Model { balls: gbs }
}

fn update(_app: &App, m: &mut Model, _update: Update) {
    for g in &mut m.balls {
        let gs = g.iter().map(|h| h.position.magnitude() as i32);
        let m = gs.max();
        for b in &mut g.iter_mut() {
            if m.unwrap() > 350 {
                b.velocity *= -1.0;
            }
            b.update();
        }
    }
}

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    for g in &m.balls {
        for b in g.iter() {
            b.display(&draw)
        }
    }
    draw.to_frame(app, &frame).unwrap();
}

fn mirror(ball: Ball, n: usize) -> Vec<Ball> {
    let mut reflect = false;
    let mut balls = vec![ball; n];
    for (i, b) in balls.iter_mut().enumerate() {
        let rotation = i as f32 / n as f32;
        let mut p = b.position;
        let mut v = b.velocity;
        if reflect {
            p = pt2(-b.position.x, b.position.y);
            v = pt2(-b.velocity.x, b.velocity.y)
        }
        b.angle = -rotation;
        p = rotate_pt(p, rotation);
        b.position = p;
        v = rotate_pt(v, rotation);
        b.velocity = v;
        reflect = !reflect;
    }
    balls
}

fn rotate_pt(p: Point2<f32>, turn: f32) -> Point2<f32> {
    let rad = Rad(turns_to_rad(turn));
    let rot: Basis2<f32> = Rotation2::from_angle(rad);
    let q = rot.rotate_point(p.into());
    pt2(q.x, q.y)
}

fn random_color() -> Alpha<Lab<D65, f32>, f32> {
    let l: f32 = random_range(0.0, 100.0);
    let a: f32 = random_range(-128.0, 127.0);
    let b: f32 = random_range(-128.0, 127.0);
    let o: f32 = random_range(0.5, 1.0);
    Laba::new(l, a, b, o)
}
