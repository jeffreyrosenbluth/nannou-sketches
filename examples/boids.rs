use nannou::prelude::*;
use nannou::ui::prelude::*;
use nannou::Draw;
use sketches::quadtree::*;
use sketches::with_opacity;

fn main() {
    nannou::app(model).update(update).run();
}

const MAXFORCE: f32 = 0.05;
const MAXSPEED: f32 = 2.5;

struct Model {
    boids: Vec<Boid>,
    qtree: Box<QNode<Boid>>,
    ui: Ui,
    ids: Ids,
    sep_strength: f32,
    sep_radius: f32,
    ali_strength: f32,
    ali_radius: f32,
    coh_strength: f32,
    coh_radius: f32,
    grid: bool,
}
widget_ids! {
    struct Ids {
        sep_strength,
        sep_radius,
        ali_strength,
        ali_radius,
        coh_strength,
        coh_radius,
        reset,
        grid,
        fps,
    }
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

    fn borders(&mut self, win: &nannou::prelude::Rect) {
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
    let mut ui = app.new_ui().build().unwrap();
    let ids = Ids::new(ui.widget_id_generator());
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
    let sep_strength = 1.5;
    let sep_radius = 25.0;
    let ali_strength = 1.0;
    let ali_radius = 75.0;
    let coh_strength = 1.0;
    let coh_radius = 100.0;
    let grid = false;
    Model {
        boids,
        qtree,
        ui,
        ids,
        sep_strength,
        sep_radius,
        ali_strength,
        ali_radius,
        coh_strength,
        coh_radius,
        grid,
    }
}

fn update(app: &App, m: &mut Model, _update: Update) {
    let ui = &mut m.ui.set_widgets();

    fn slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
        widget::Slider::new(val, min, max)
            .w_h(150.0, 24.0)
            .label_font_size(12)
            .rgb(75. / 255., 136. / 255., 162. / 255.)
            .label_rgb(211. / 255., 212. / 255., 217. / 255.)
            .border(0.5)
            .border_rgb(37. / 255., 38. / 255., 39. / 255.)
    }

    let sep_label = format!("Separation Strength: {:.1}", m.sep_strength);
    for value in slider(m.sep_strength, 0.0, 3.0)
        .top_left_with_margin(20.0)
        .label(&sep_label[..])
        .set(m.ids.sep_strength, ui)
    {
        m.sep_strength = value;
    }

    let sep_label = format!("Separation Radius: {:.0}", m.sep_radius);
    for value in slider(m.sep_radius, 0.0, 200.0)
        .down(10.0)
        .label(&sep_label[..])
        .set(m.ids.sep_radius, ui)
    {
        m.sep_radius = value;
    }

    let ali_label = format!("Alignment Strength: {:.1}", m.ali_strength);
    for value in slider(m.ali_strength, 0.0, 3.0)
        .down(10.0)
        .label(&ali_label[..])
        .set(m.ids.ali_strength, ui)
    {
        m.ali_strength = value;
    }

    let ali_label = format!("Alignment Radius: {:.0}", m.ali_radius);
    for value in slider(m.ali_radius, 0.0, 200.0)
        .down(10.0)
        .label(&ali_label[..])
        .set(m.ids.ali_radius, ui)
    {
        m.ali_radius = value;
    }

    let coh_label = format!("Cohesion Strength: {:.1}", m.coh_strength);
    for value in slider(m.coh_strength, 0.0, 3.0)
        .down(10.0)
        .label(&coh_label[..])
        .set(m.ids.coh_strength, ui)
    {
        m.coh_strength = value;
    }

    let coh_label = format!("Cohesion Radius: {:.0}", m.coh_radius);
    for value in slider(m.coh_radius, 0.0, 200.0)
        .down(10.0)
        .label(&coh_label[..])
        .set(m.ids.coh_radius, ui)
    {
        m.coh_radius = value;
    }

    for _click in widget::Button::new()
        .down(20.0)
        .w_h(150.0, 30.0)
        .label("Reset")
        .label_font_size(12)
        .rgb(37. / 255., 38. / 255., 39. / 255.)
        .label_rgb(211. / 255., 212. / 255., 217. / 255.)
        .border(0.0)
        .set(m.ids.reset, ui)
    {
        m.sep_strength = 1.5;
        m.sep_radius = 25.0;
        m.ali_strength = 1.0;
        m.ali_radius = 75.0;
        m.coh_strength = 1.0;
        m.coh_radius = 100.0;
    }

    for _click in widget::Button::new()
        .down(10.0)
        .w_h(150.0, 30.0)
        .label("Toggle Grid")
        .label_font_size(12)
        .rgb(37. / 255., 38. / 255., 39. / 255.)
        .label_rgb(211. / 255., 212. / 255., 217. / 255.)
        .border(0.0)
        .set(m.ids.grid, ui)
    {
        m.grid = !m.grid
    }

    let fps_label = format!("fps {:.0}", app.fps().min(60.0));
    let _frame_rate = widget::TextBox::new(&fps_label[..])
        .bottom_left_with_margin(20.0)
        .w_h(150.0, 30.0)
        .font_size(12)
        .text_color(color::Color::Rgba(211./255., 212./255., 217./255., 1.))
        .rgb(0. / 255., 0. / 255., 0. / 255.)
        .set(m.ids.fps, ui);

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
        let sep_flock = quad_tree.points_in_circle(bl, tr, boid.pos(), m.sep_radius);
        let ali_flock = quad_tree.points_in_circle(bl, tr, boid.pos(), m.ali_radius);
        let coh_flock = quad_tree.points_in_circle(bl, tr, boid.pos(), m.coh_radius);
        sep.push(boid.separate(&sep_flock, m.sep_radius) * m.sep_strength);
        ali.push(boid.align(&ali_flock) * m.ali_strength);
        coh.push(boid.cohesion(&coh_flock) * m.coh_strength);
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
    if m.grid {
        draw_qtree(m.qtree.clone(), bl, tr, &draw);
    }
    for boid in &m.boids {
        display(&boid, &draw, &m);
    }
    draw.to_frame(app, &frame).unwrap();
    m.ui.draw_to_frame(app, &frame).unwrap();
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
        .stroke_color(rgb8(37, 38, 39))
        .stroke_weight(1.0);
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

fn display(boid: &Boid, draw: &Draw, m: &Model) {
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
    let clear = with_opacity(BLACK, 0.0);

    if *highlight {
        c = WHITE;
        draw.ellipse()
            .color(clear)
            .w_h(m.coh_radius * 2., m.coh_radius * 2.)
            .xy(*position)
            .stroke_weight(0.5)
            .stroke_color(rgb8(211, 212, 217));

        draw.ellipse()
            .color(clear)
            .w_h(m.ali_radius * 2., m.ali_radius * 2.)
            .xy(*position)
            .stroke_weight(0.5)
            .stroke_color(rgb8(75, 136, 162));

        draw.ellipse()
            .color(clear)
            .w_h(m.sep_radius * 2., m.sep_radius * 2.)
            .xy(*position)
            .stroke_weight(0.5)
            .stroke_color(rgb8(252, 81, 48));
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
