use nannou::prelude::*;

const CAPACITY: usize = 64;

pub fn blq(bl: Point2, tr: Point2) -> (Point2, Point2) {
    (bl, (bl + tr) / 2.0)
}

pub fn brq(bl: Point2, tr: Point2) -> (Point2, Point2) {
    (
        pt2((bl.x + tr.x) / 2.0, bl.y),
        pt2(tr.x, (bl.y + tr.y) / 2.0),
    )
}

pub fn tlq(bl: Point2, tr: Point2) -> (Point2, Point2) {
    (
        pt2(bl.x, (bl.y + tr.y) / 2.0),
        pt2((bl.x + tr.x) / 2.0, tr.y),
    )
}

pub fn trq(bl: Point2, tr: Point2) -> (Point2, Point2) {
    ((bl + tr) / 2.0, tr)
}

pub trait Position {
    fn pos(&self) -> Point2;
}

impl Position for Point2 {
    fn pos(&self) -> Point2 {
        *self
    }
}

#[derive(Debug, Clone)]
pub struct Quadrants<T> {
    pub bl: Box<QNode<T>>,
    pub br: Box<QNode<T>>,
    pub tl: Box<QNode<T>>,
    pub tr: Box<QNode<T>>,
}

impl<T> Quadrants<T> {
    pub fn new(bl: QNode<T>, br: QNode<T>, tl: QNode<T>, tr: QNode<T>) -> Self {
        let bl = Box::new(bl);
        let br = Box::new(br);
        let tl = Box::new(tl);
        let tr = Box::new(tr);
        Self { bl, br, tl, tr }
    }
}

#[derive(Debug, Clone)]
pub enum QNode<T> {
    Points(Vec<T>),
    Quad(Quadrants<T>),
}

impl<T: Position + Clone> QNode<T> {
    pub fn split(&mut self, bl: Point2, tr: Point2) {
        let mut bl_quad = vec![];
        let mut br_quad = vec![];
        let mut tl_quad = vec![];
        let mut tr_quad = vec![];
        match self {
            QNode::Points(ps) => {
                let midx = (bl.x + tr.x) / 2.0;
                let midy = (bl.y + tr.y) / 2.0;
                for p in ps {
                    if p.pos().x <= midx {
                        if p.pos().y <= midy {
                            bl_quad.push(p.clone());
                        } else {
                            tl_quad.push(p.clone());
                        }
                    } else {
                        if p.pos().y <= midy {
                            br_quad.push(p.clone());
                        } else {
                            tr_quad.push(p.clone());
                        }
                    }
                }
            }
            _ => panic!("Only Points Nodes can be split"),
        }
        let quadrants = Quadrants::new(
            QNode::Points(bl_quad),
            QNode::Points(br_quad),
            QNode::Points(tl_quad),
            QNode::Points(tr_quad),
        );
        *self = QNode::Quad(quadrants);
    }

    pub fn insert(&mut self, p: T, bl: Point2, tr: Point2) {
        let midx = (bl.x + tr.x) / 2.0;
        match self {
            QNode::Points(pts) => {
                pts.push(p);
                if pts.len() > CAPACITY {
                    self.split(bl, tr);
                }
            }
            QNode::Quad(q) if p.pos().x <= midx => {
                let midy = (bl.y + tr.y) / 2.0;
                let mid = pt2(midx, midy);
                if p.pos().y <= midy {
                    q.bl.insert(p, bl, mid);
                } else {
                    q.tl.insert(p, pt2(bl.x, midy), pt2(midx, tr.y));
                }
            }
            QNode::Quad(q) => {
                let midy = (bl.y + tr.y) / 2.0;
                let mid = pt2(midx, midy);
                if p.pos().y <= midy {
                    q.br.insert(p, pt2(midx, bl.y), pt2(tr.x, midy));
                } else {
                    q.tr.insert(p, mid, tr);
                }
            }
        }
    }

    pub fn points_in_circle(&self, bl: Point2, tr: Point2, center: Point2, radius: f32) -> Vec<T> {
        let mut pts = vec![];
        if !intersects(bl, tr, center, radius) {
            return pts;
        }
        match self {
            QNode::Points(ps) => {
                for p in ps {
                    if (p.pos().x - center.x) * (p.pos().x - center.x)
                        + (p.pos().y - center.y) * (p.pos().y - center.y)
                        <= radius * radius
                    {
                        pts.push(p.clone());
                    }
                }
            }
            QNode::Quad(q) => {
                let (a, b) = blq(bl, tr);
                pts.append(&mut q.bl.points_in_circle(a, b, center, radius));

                let (a, b) = brq(bl, tr);
                pts.append(&mut q.br.points_in_circle(a, b, center, radius));

                let (a, b) = tlq(bl, tr);
                pts.append(&mut q.tl.points_in_circle(a, b, center, radius));

                let (a, b) = trq(bl, tr);
                pts.append(&mut q.tr.points_in_circle(a, b, center, radius));
            }
        }
        pts
    }
}

pub fn intersects(bl: Point2, tr: Point2, center: Point2, radius: f32) -> bool {
    center.x >= bl.x - radius
        && center.x < tr.x + radius
        && center.y >= bl.y - radius
        && center.y < tr.y + radius
}

#[cfg(test)]
mod tests {
    use nannou::prelude::*;

    use super::*;

    #[test]
    fn test_split() {
        let pts = vec![];
        let mut qt = QNode::Points(pts);
        for _ in 0..1000 {
            qt.insert(
                pt2(random_range(0.0, 3.0), random_range(0.0, 3.0)),
                pt2(0., 0.),
                pt2(3., 3.),
            );
        }
        let c = qt.points_in_circle(pt2(0.0, 0.0), pt2(3.0, 3.0), pt2(2.0, 0.5), 0.2);
        dbg!(c);
    }
}
