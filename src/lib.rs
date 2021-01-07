use nannou::color::{Alpha, IntoLinSrgba, Lab, Laba};
use nannou::ease::cubic::ease_in_out;
use nannou::math::{Basis2, Rad};
use nannou::prelude::*;
use nannou::{
    color::white_point::D65,
    draw::{primitive::Path, Drawing},
};

pub fn img_path(app: &App) -> std::path::PathBuf {
    app.project_path()
        .expect("failed to locate `project_path`")
        .join("img")
        .join(format!("{}", app.exe_name().unwrap()))
        .with_extension("png")
}

pub fn gif_path(app: &App, frame: &Frame) -> std::path::PathBuf {
    app.project_path()
        .expect("failed to locate `project_path`")
        .join(app.exe_name().unwrap())
        .join(format!("frame_{:03}", frame.nth()))
        .with_extension("png")
}

pub fn clock(frame: u64) -> f32 {
    let t = (frame % 180) as f32 / 180.;
    ease_in_out(t, 0., 1., 1.)
}

// Use random_rgba instead.
pub fn random_color() -> Alpha<Lab<D65, f32>, f32> {
    let l: f32 = random_range(0.0, 100.0);
    let a: f32 = random_range(-128.0, 127.0);
    let b: f32 = random_range(-128.0, 127.0);
    let o: f32 = random_range(0.5, 1.0);
    Laba::new(l, a, b, o)
}

pub fn random_rgb() -> LinSrgba {
    let l: f32 = random_range(0.0, 100.0);
    let a: f32 = random_range(-128.0, 127.0);
    let b: f32 = random_range(-128.0, 127.0);
    Laba::new(l, a, b, 1.0).into_lin_srgba()
}

pub fn random_rgba() -> LinSrgba {
    let l: f32 = random_range(0.0, 100.0);
    let a: f32 = random_range(-128.0, 127.0);
    let b: f32 = random_range(-128.0, 127.0);
    let o: f32 = random_range(0.5, 1.0);
    Laba::new(l, a, b, o).into_lin_srgba()
}

pub fn set_opacity(c: LinSrgba, o: f32) -> LinSrgba {
    srgba(
        c.red as f32 / 255.,
        c.green as f32 / 255.,
        c.blue as f32 / 255.,
        o,
    )
    .into_lin_srgba()
}

// Use set_opacity instead, just around to support older sketches.
pub fn with_opacity(c: nannou::color::Srgb<u8>, o: f32) -> nannou::color::rgb::Srgba {
    srgba(
        c.red as f32 / 255.,
        c.green as f32 / 255.,
        c.blue as f32 / 255.,
        o,
    )
}

pub fn rotate_pt(p: Point2<f32>, turn: f32) -> Point2<f32> {
    let rad = Rad(turns_to_rad(turn));
    let rot: Basis2<f32> = Rotation2::from_angle(rad);
    let q = rot.rotate_point(p.into());
    pt2(q.x, q.y)
}

pub fn circle_mask<T>(draw: &Draw, width: f32, height: f32, radius: f32, color: T)
where
    T: IntoLinSrgba<f32>,
{
    use nannou::geom::path::Builder;
    let mut builder = Builder::new();
    let w2 = width / 2.;
    let h2 = height / 2.;
    builder = builder.move_to(pt2(-w2, h2));
    builder = builder.line_to(pt2(w2, h2));
    builder = builder.line_to(pt2(w2, -h2));
    builder = builder.line_to(pt2(-w2, -h2));
    builder = builder.move_to(pt2(0., -radius));
    builder = builder.arc(pt2(0., 0.), vec2(radius, radius), TAU, 0.);
    builder = builder.close();

    let p = builder.build();

    // draw arc
    draw.path().fill().color(color).events(p.iter());
}

pub fn arc<C>(
    draw: &Draw,
    start_deg: f32,
    angle_deg: f32,
    radius: f32,
    color: C,
    weight: f32,
) -> Drawing<Path<f32>, f32>
where
    C: IntoLinSrgba<f32>,
{
    // ) -> DrawingPath {
    let start = start_deg as usize;
    let end = start + angle_deg as usize;
    let pts = (start..=end).map(|i| {
        let theta = i as f32 / 360.0 * TAU;
        pt2(radius * theta.cos(), radius * theta.sin())
    });

    draw.polyline()
        .join_round()
        .color(color)
        .weight(weight)
        .points(pts)
}

pub fn border(app: &App, draw: &Draw, width: f32) {
    let (xy, wh) = app.window_rect().xy_wh();
    draw.rect()
        .xy(xy)
        .wh(wh)
        .color(srgba(0.0, 0.0, 0.0, 0.0))
        .stroke(BLACK)
        .stroke_weight(width);
}

pub struct Grid<T> {
    pub width: f32,
    pub height: f32,
    pub spacing: f32,
    pub grid: Vec<T>,
    pub pts: Vec<Point2>,
}

impl<T> Grid<T>
where
    T: Copy,
{
    pub fn new(width: f32, height: f32, spacing: f32, gen: impl Fn(f32, f32) -> T) -> Self {
        let rows = (height / spacing) as usize;
        let cols = (width / spacing) as usize;
        let mut grid = vec![];
        let mut pts = vec![];
        for i in 0..rows {
            let y = -height / 2.0 + i as f32 * spacing;
            for j in 0..cols {
                let x = -width / 2.0 + j as f32 * spacing;
                grid.push(gen(x, y));
                pts.push(pt2(x, y));
            }
        }
        Self {
            width,
            height,
            spacing,
            grid,
            pts,
        }
    }

    pub fn rows(&self) -> usize {
        (self.height / self.spacing) as usize
    }

    pub fn cols(&self) -> usize {
        (self.width / self.spacing) as usize
    }

    pub fn get(&self, x: f32, y: f32) -> T {
        let n = self.rows();
        let m = self.cols();
        let xn = x + self.width / 2.0;
        let yn = y + self.height / 2.0;

        let mut col = if xn < 0.0 {
            0
        } else {
            ((x + self.width / 2.0) / self.spacing) as usize
        };
        let mut row = if yn < 0.0 {
            0
        } else {
            ((y + self.height / 2.0) / self.spacing) as usize
        };

        while col >= m {
            col -= 1;
        }
        while row >= n {
            row -= 1;
        }

        self.grid[row * m + col]
    }

    pub fn iter<'a>(&'a self) -> GridIter<'a, T> {
        GridIter {
            grid: self,
            i: 0,
            j: 0,
        }
    }

    pub fn x_bounds(&self) -> (f32, f32) {
        (-self.width / 2.0, self.width / 2.0)
    }

    pub fn y_bounds(&self) -> (f32, f32) {
        (-self.height / 2.0, self.height / 2.0)
    }
}

pub struct GridIter<'a, T>
where
    T: Copy,
{
    grid: &'a Grid<T>,
    i: usize,
    j: usize,
}

impl<'a, T> Iterator for GridIter<'a, T>
where
    T: Copy,
{
    type Item = (Point2, T);

    fn next(&mut self) -> Option<Self::Item> {
        let n = (self.grid.width / self.grid.spacing) as usize;
        if self.i * n + self.j >= self.grid.grid.len() {
            return None;
        };
        let x = -self.grid.width / 2.0 + self.j as f32 * self.grid.spacing;
        let y = -self.grid.height / 2.0 + self.i as f32 * self.grid.spacing;
        let result = (pt2(x, y), self.grid.grid[self.i * n + self.j]);

        if self.j >= n - 1 {
            self.j = 0;
            self.i += 1;
        } else {
            self.j += 1;
        };

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_test() {
        let grid = Grid::new(200.0, 100.0, 10.0, |x, y| (x, y));
        assert_eq!(grid.get(0.0, 0.0), (0.0, 0.0));
        assert_eq!(grid.get(15.0, 25.0), (10.0, 20.0));
        assert_eq!(grid.get(-25.0, 25.0), (-30.0, 20.0));
        assert_eq!(grid.get(29.0, -29.0), (20.0, -30.0));
        assert_eq!(grid.get(-80.0, -29.0), (-80.0, -30.0));
    }

    #[test]
    fn get_test_bounds() {
        let grid = Grid::new(200.0, 100.0, 10.0, |x, y| (x, y));
        assert_eq!(grid.get(-100.0, -50.0), (-100.0, -50.0));
        assert_eq!(grid.get(99.0, 49.0), (90.0, 40.0));
        assert_eq!(grid.get(200.0, 100.0), (90.0, 40.0));
        assert_eq!(grid.get(-200.0, -100.0), (-100.0, -50.0));
    }
}
