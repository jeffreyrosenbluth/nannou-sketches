use nannou::prelude::*;
use nannou::Draw;
use sketches::quadtree::*;

fn main() {
    nannou::app(model).update(update).run();
}

const MAXFORCE: f32 = 0.05;
const MAXSPEED: f32 = 2.5;

struct Model {
    boids: Vec<Boid>,
    qtree: Box<QNode<Boid>>,
}

#[derive(Clone, PartialEq)]
struct Boid {
    position: Vector2,
    velocity: Vector2,
    acceleration: Vector2,
    r: f32,
    highlight: bool,
}

impl Position for Boid {
    fn pos(&self) -> Point2 {
        self.position
    }
}

impl Boid {
    fn new(x: f32, y: f32) -> Self {
        let position = pt2(x, y);
        let velocity = vec2(0.0, 0.0);
        let acceleration = vec2(0.0, 0.0);
        let r = 2.0;
        let highlight = false;

        Boid {
            position,
            velocity,
            acceleration,
            r,
            highlight,
        }
    }

    fn tally(
        &self,
        boids: &Vec<Boid>,
        acc: impl Fn(&Boid) -> Vector2,
        steer: impl Fn(Vector2, i32) -> Vector2,
    ) -> Vector2 {
        let mut sum = vec2(0., 0.);
        let mut count = 0;
        for b in boids {
            if b != self {
                sum += acc(b);
                count += 1;
            }
        }
        if count > 0 {
            return steer(sum, count);
        } else {
            return vec2(0., 0.);
        }
    }

    fn align(&self, boids: &Vec<Boid>) -> Vector2 {
        let steer = |s: Vector2, c: i32| {
            ((s / c as f32).with_magnitude(MAXSPEED) - self.velocity).limit_magnitude(MAXFORCE)
        };
        self.tally(boids, &|b: &Boid| b.velocity, &steer)
    }

    fn separate(&self, boids: &Vec<Boid>, dist: f32) -> Vector2 {
        let acc = |b: &Boid| (self.position - b.position).with_magnitude(1. / dist);
        let steer = |s: Vector2, _c: i32| {
            if s.magnitude() > 0. {
                return (s.with_magnitude(MAXSPEED) - self.velocity).limit_magnitude(MAXFORCE);
            } else {
                return vec2(0., 0.);
            }
        };
        self.tally(boids, &acc, &steer)
    }

    fn cohesion(&self, boids: &Vec<Boid>) -> Vector2 {
        let steer = |s: Vector2, c: i32| self.seek(s / c as f32);
        self.tally(boids, &|b: &Boid| b.position, &steer)
    }

    fn update(&mut self) {
        self.velocity += self.acceleration;
        self.velocity.limit_magnitude(MAXSPEED);
        self.position += self.velocity;
        self.acceleration *= 0.;
    }

    fn seek(&self, target: Vector2) -> Vector2 {
        let desired = target - self.position;
        let des = desired.with_magnitude(MAXSPEED);
        let steer = des - self.velocity;
        steer.limit_magnitude(MAXFORCE)
    }

    fn borders(&mut self, win: &Rect) {
        let l = win.left();
        let r = win.right();
        let t = win.top();
        let b = win.bottom();
        let rd = self.r;
        match self.position {
            Vector2 { x, .. } if x < l - rd => self.position.x = r + rd,
            Vector2 { y, .. } if y < b - rd => self.position.y = t + rd,
            Vector2 { x, .. } if x > r + rd => self.position.x = l - rd,
            Vector2 { y, .. } if y > t + rd => self.position.y = b - rd,
            _ => (),
        };
    }
}

fn model(app: &App) -> Model {
    const BOID_COUNT: usize = 1000;
    app.new_window()
        .size(1500, 1000)
        .view(view)
        .build()
        .unwrap();
    let bl = app.window_rect().bottom_left();
    let tr = app.window_rect().top_right();
    let mut boids = Vec::new();
    for _ in 0..BOID_COUNT {
        let x = random_range(bl.x, tr.x);
        let y = random_range(bl.y, tr.y);
        boids.push(Boid::new(x, y));
    }
    boids[0].highlight = true;
    let qtree = Box::new(QNode::Points(vec![]));
    Model { boids, qtree }
}

fn update(app: &App, m: &mut Model, _update: Update) {
    let bl = app.window_rect().bottom_left();
    let tr = app.window_rect().top_right();
    let mut sep = Vec::new();
    let mut ali = Vec::new();
    let mut coh = Vec::new();
    let quad_tree = &mut QNode::Points(vec![]);
    for b in &m.boids {
        quad_tree.insert(b.clone(), bl, tr);
    }
    m.qtree = Box::new(quad_tree.clone());
    for boid in &m.boids {
        let sep_flock = quad_tree.points_in_circle(bl, tr, boid.pos(), 25.0);
        let flock = quad_tree.points_in_circle(bl, tr, boid.pos(), 100.0);
        sep.push(boid.separate(&sep_flock, 25.0) * 1.5);
        ali.push(boid.align(&flock));
        coh.push(boid.cohesion(&flock));
    }
    for (i, boid) in m.boids.iter_mut().enumerate() {
        boid.acceleration += sep[i] + ali[i] + coh[i];
        boid.borders(&app.window_rect());
        boid.update();
    }
}

fn view(app: &App, m: &Model, frame: Frame) {
    let bl = app.window_rect().bottom_left();
    let tr = app.window_rect().top_right();
    let draw = app.draw();
    draw.background().color(BLACK);
    draw_qtree(m.qtree.clone(), bl, tr, &draw);
    for boid in &m.boids {
        display(&boid, &draw);
    }
    draw.to_frame(app, &frame).unwrap();
}

fn centered_rect(bl: Point2, tr: Point2) -> (Point2, Point2) {
    ((bl + tr) / 2.0, tr - bl)
}

fn draw_rect(bl: Point2, tr: Point2, draw: &Draw) {
    let (ctr, dims) = centered_rect(bl, tr);
    draw.rect()
        .xy(ctr)
        .wh(dims)
        .color(BLACK)
        .stroke_color(GRAY)
        .stroke_weight(0.5);
}

fn draw_qtree(qtree: Box<QNode<Boid>>, bl: Point2, tr: Point2, draw: &Draw) {
    match *qtree {
        QNode::Points(_) => draw_rect(bl, tr, draw),
        QNode::Quad(qs) => {
            let (a, b) = blq(bl, tr);
            draw_rect(a, b, draw);
            draw_qtree(qs.bl, a, b, draw);
            let (a, b) = brq(bl, tr);
            draw_rect(a, b, draw);
            draw_qtree(qs.br, a, b, draw);
            let (a, b) = tlq(bl, tr);
            draw_rect(a, b, draw);
            draw_qtree(qs.tl, a, b, draw);
            let (a, b) = trq(bl, tr);
            draw_rect(a, b, draw);
            draw_qtree(qs.tr, a, b, draw);
        }
    }
}

fn display(boid: &Boid, draw: &Draw) {
    let Boid {
        position,
        velocity,
        r,
        highlight,
        ..
    } = boid;

    let theta = velocity.angle() + PI / 2.;
    let mut c = PLUM;
    let r = *r;

    if *highlight {
        c = RED;
        draw.ellipse().color(BLACK).w_h(200., 200.).xy(*position).stroke_weight(0.5).stroke(BLUE);
        draw.ellipse().color(BLACK).w_h(50., 50.).xy(*position).stroke_weight(0.5).stroke(YELLOW);
    }
    let points = vec![
        pt2(0., -r * 2.),
        pt2(-r, r * 2.),
        pt2(0., r),
        pt2(r, r * 2.),
    ];
    draw.polygon()
        .points(points)
        .xy(*position)
        .color(c)
        .rotate(theta);
}
