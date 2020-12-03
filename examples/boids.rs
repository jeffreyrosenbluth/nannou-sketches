use nannou::prelude::*;
use nannou::Draw;
use sketches::quadtree::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    boids: Vec<Boid>,
}

#[derive(Clone, PartialEq)]
struct Boid {
    position: Vector2,
    velocity: Vector2,
    acceleration: Vector2,
    r: f32,
    max_force: f32,
    max_speed: f32,
}

impl Position for Boid {
    fn pos(&self) -> Point2 {
        self.position
    }
}

impl Boid {
    fn new(x: f32, y: f32) -> Self {
        let position = vec2(x, y);
        let velocity = vec2(random_range(-1.0, 1.0), random_range(-1.0, 1.0)).with_magnitude(4.0);
        let acceleration = vec2(0.0, 0.0);
        let r = 3.0;
        let max_force = 1.0;
        let max_speed = 4.0;

        Boid {
            position,
            velocity,
            acceleration,
            r,
            max_force,
            max_speed,
        }
    }

    fn tally(
        &self,
        boids: &Vec<Boid>,
        dist: f32,
        acc: impl Fn(&Boid) -> Vector2,
        steer: impl Fn(Vector2, i32) -> Vector2,
    ) -> Vector2 {
        let mut sum = vec2(0., 0.);
        let mut quad_tree = QNode::Points(vec![]);
        for b in boids {
            if b != self {
                quad_tree.insert(b.clone(), vec2(-750.0, -500.0), vec2(750.0, 500.0));
            }
        }
        let flock =
            quad_tree.points_in_circle(vec2(-750.0, -500.0), vec2(750.0, 500.0), self.pos(), dist);
        let n = flock.len();
        for ref b in flock {
            sum += acc(b);
        }
        if n > 0 {
            return steer(sum, n as i32);
        }
        vec2(0., 0.)
    }

    fn align(&self, boids: &Vec<Boid>, dist: f32) -> Vector2 {
        let steer = |s: Vector2, c: i32| {
            ((s / c as f32).with_magnitude(self.max_speed) - self.velocity)
                .limit_magnitude(self.max_force)
        };
        self.tally(boids, dist, &|b: &Boid| b.velocity, &steer)
    }

    fn separate(&self, boids: &Vec<Boid>, dist: f32) -> Vector2 {
        let acc = |b: &Boid| (self.position - b.position).with_magnitude(1. / dist);
        let steer = |s: Vector2, _c: i32| {
            if s.magnitude() > 0. {
                return (s.with_magnitude(self.max_speed) - self.velocity)
                    .limit_magnitude(self.max_force);
            } else {
                return vec2(0., 0.);
            }
        };
        self.tally(boids, dist, &acc, &steer)
    }

    fn cohesion(&self, boids: &Vec<Boid>, dist: f32) -> Vector2 {
        let steer = |s: Vector2, c: i32| self.seek(s / c as f32);
        self.tally(boids, dist, &|b: &Boid| b.position, &steer)
    }

    fn update(&mut self) {
        self.velocity += self.acceleration;
        self.velocity.limit_magnitude(self.max_speed);
        self.position += self.velocity;
        self.acceleration *= 0.;
    }

    fn seek(&self, target: Vector2) -> Vector2 {
        let desired = target - self.position;
        let des = desired.with_magnitude(self.max_speed);
        let steer = des - self.velocity;
        steer.limit_magnitude(self.max_force)
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
    app.new_window()
        .size(1500, 1000)
        .view(view)
        .build()
        .unwrap();
    let mut boids = Vec::new();
    for _ in 0..1000 {
        let x = random_range(-750., 750.);
        let y = random_range(-500., 500.);
        boids.push(Boid::new(x, y));
    }
    Model { boids }
}

fn update(app: &App, m: &mut Model, _update: Update) {
    let mut sep = Vec::new();
    let mut ali = Vec::new();
    let mut coh = Vec::new();
    for boid in &m.boids {
        sep.push(boid.separate(&m.boids, 50.) * 1.2);
        ali.push(boid.align(&m.boids, 50.));
        coh.push(boid.cohesion(&m.boids, 100.));
    }
    for (i, boid) in m.boids.iter_mut().enumerate() {
        boid.acceleration += sep[i] + ali[i] + coh[i];
        boid.borders(&app.window_rect());
        boid.update();
    }
}

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    for boid in &m.boids {
        display(&boid, &draw);
    }
    draw.to_frame(app, &frame).unwrap();
}

fn display(boid: &Boid, draw: &Draw) {
    let Boid {
        position,
        velocity,
        r,
        ..
    } = boid;

    let theta = velocity.angle() + PI / 2.;
    let points = vec![
        pt2(0., -r * 2.),
        pt2(-r, r * 2.),
        pt2(0., *r),
        pt2(*r, r * 2.),
    ];
    draw.polygon()
        .points(points)
        .xy(*position)
        .color(PLUM)
        .rotate(theta);
}
