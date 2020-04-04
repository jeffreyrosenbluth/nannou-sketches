use nannou::noise::NoiseFn;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run()
}

fn noise(p: Point2, scale: f64) -> f32 {
    let qx = p.x as f64 / scale;
    let qy = p.y as f64 / scale;
    let nn = nannou::noise::HybridMulti::new();
    let r = nn.get([qx, qy]);
    ((r + 1.) / 2. * scale * TAU as f64) as f32
}

struct Particle {
    position: Vector2,
    velocity: Vector2,
    direction: Vector2,
}

impl Particle {
    fn new(x: f32, y: f32) -> Particle {
        Particle {
            position: vec2(x, y),
            velocity: vec2(0., 0.),
            direction: vec2(0., 0.),
        }
    }

    fn go(&mut self, speed: f32) {
        let angle = noise(self.position, 100.);
        self.direction.x = angle.cos();
        self.direction.y = angle.sin();
        self.velocity = self.direction.with_magnitude(speed);
        self.position += self.velocity;
    }
    fn edges(&mut self, rect: &Rect) {
        if self.position.x > rect.right()
            || self.position.x < rect.left()
            || self.position.y > rect.top()
            || self.position.y < rect.bottom()
        {
            self.position.x = random_range(rect.left(), rect.right());
            self.position.y = random_range(rect.bottom(), rect.top());
        }
    }
}

struct Model {
    a: Vec<Particle>,
    b: Vec<Particle>,
    c: Vec<Particle>,
    speed: f32,
    n: usize,
}

fn model(app: &App) -> Model {
    app.new_window().size(800, 800).view(view).build().unwrap();
    let win = app.window_rect();
    let mut a: Vec<Particle> = vec![];
    let mut b: Vec<Particle> = vec![];
    let mut c: Vec<Particle> = vec![];
    let speed = 2.;
    let n = 200;
    for _ in 0..n {
        a.push(Particle::new(
            random_range(win.left(), win.right()),
            random_range(win.bottom(), win.top()),
        ));
        b.push(Particle::new(
            random_range(win.left(), win.right()),
            random_range(win.bottom(), win.top()),
        ));
        c.push(Particle::new(
            random_range(win.left(), win.right()),
            random_range(win.bottom(), win.top()),
        ));
    }
    Model { a, b, c, speed, n }
}

fn update(app: &App, m: &mut Model, _update: Update) {
    let win = app.window_rect();
    for i in 0..m.n {
        m.a[i].go(m.speed);
        m.b[i].go(m.speed);
        m.c[i].go(m.speed);
        m.a[i].edges(&win);
        m.b[i].edges(&win);
        m.c[i].edges(&win);
    }
}

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();
    if frame.nth() == 0 {
        frame.clear(WHITE);
    }
    for i in 0..m.n {
        let r = map_range(i as f32, 0., m.n as f32, 1.0, 2.0);
        draw.ellipse().xy(m.a[i].position).color(BLUE).w_h(r, r);
        draw.ellipse()
            .xy(m.b[i].position)
            .color(CORNFLOWERBLUE)
            .w_h(r, r);
        draw.ellipse().xy(m.c[i].position).color(WHITE).w_h(r, r);
    }
    draw.to_frame(app, &frame).unwrap();
}
