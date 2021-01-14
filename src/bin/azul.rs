use getopts::Options;
use nannou::prelude::*;
use std::{env, fmt::Display, writeln};

use sketches::gif_path;

const SIZE: usize = 3;
const WIDTH: u32 = 900;
const HEIGHT: u32 = 600;

fn main() {
    let mut board = Board::new(SIZE);
    for _ in 0..board.size * board.size {
        let ((r, c), _infl) = board.max_influence();
        board = board.place(r, c);
    }
    println!("Value: {}", board.value());
    println!("{}", board);
    // nannou::app(model).update(update).run();
}

#[derive(Debug, Clone, Copy)]
struct Cell {
    order: Option<usize>,
    value: usize,
    influence: usize,
}

impl Cell {
    fn new() -> Self {
        Self {
            order: None,
            value: 0,
            influence: 0,
        }
    }
}

fn scan(v: &Vec<Cell>, n: i32) -> usize {
    let mut i = n - 1;
    let mut j = n + 1;
    let mut val = 0;
    while i >= 0 && v[i as usize].order.is_some() {
        val += 1;
        i -= 1;
    }
    while j < v.len() as i32 && v[j as usize].order.is_some() {
        val += 1;
        j += 1;
    }
    val
}

#[derive(Debug, Clone)]
struct Board {
    size: usize,
    cells: Vec<Cell>,
    next: usize,
}

impl Board {
    fn new(size: usize) -> Self {
        let cells = vec![Cell::new(); size * size];
        Self {
            size,
            cells,
            next: 0,
        }
    }

    fn taken(&self) -> usize {
        let mut count = 0;
        for r in 0..self.size {
            for c in 0..self.size {
                if self.get(r, c).order.is_some() {
                    count += 1;
                }
            }
        }
        count
    }

    fn remaining(&self) -> usize {
        self.size * self.size - self.taken()
    }

    fn get_mut(&mut self, row: usize, col: usize) -> &mut Cell {
        &mut self.cells[row * self.size + col]
    }

    fn get(&self, row: usize, col: usize) -> Cell {
        self.cells[row * self.size + col]
    }

    fn value(&self) -> usize {
        self.cells.iter().fold(0, |acc, x| acc + x.value)
    }

    fn row(&self, i: usize) -> Vec<Cell> {
        let mut r = vec![];
        for j in 0..self.size {
            r.push(self.get(i, j));
        }
        r
    }

    fn col(&self, i: usize) -> Vec<Cell> {
        let mut c = vec![];
        for j in 0..self.size {
            c.push(self.get(j, i));
        }
        c
    }

    fn cell_value(&self, row: usize, col: usize) -> usize {
        let r = self.row(row);
        let c = self.col(col);
        let val;
        let r_val = scan(&r, col as i32);
        let c_val = scan(&c, row as i32);
        if r_val == 0 {
            val = c_val + 1;
        } else if c_val == 0 {
            val = r_val + 1;
        } else {
            val = r_val + c_val + 2;
        }
        val
    }

    fn place(&self, row: usize, col: usize) -> Board {
        let mut board = self.clone();
        let cell = board.get_mut(row, col);
        if cell.order.is_some() {
            panic!("Cell ({},{}) is already taken", row, col);
        }
        cell.order = Some(self.next);
        cell.value = self.cell_value(row, col);
        board.next += 1;
        board
    }

    fn influence(&self, row: usize, col: usize) -> i32 {
        let board = self.clone().place(row, col);
        let val = board.value();
        let mut infl = 0;
        for r in 0..board.size {
            for c in 0..board.size {
                if board.get(r, c).order.is_some() {
                    continue;
                }
                let b = board.place(r, c);
                infl += b.value() - val;
            }
        }
        infl as i32
    }

    fn influences(&self) -> Board {
        let mut board = self.clone();
        for r in 0..board.size {
            for c in 0..board.size {
                if board.get(r, c).order.is_some() {
                    continue;
                }
                board.get_mut(r, c).influence = board.influence(r, c) as usize;
            }
        }
        board
    }

    fn max_influence(&self) -> ((usize, usize), i32) {
        let mut coords = (0, 0);
        let mut max_infl = -1;
        let board = self.influences();
        for r in 0..self.size {
            for c in 0..board.size {
                if board.get(r, c).order.is_some() {
                    continue;
                }
                let infl = board.get(r, c).influence as i32;
                if infl > max_infl {
                    max_infl = infl;
                    coords = (r, c);
                }
            }
        }
        (coords, max_infl)
    }

    fn display_influence(&self) {
        for r in 0..self.size {
            for x in self.row(r) {
                print!("{:3}", x.influence);
            }
            println!();
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.size {
            for x in self.row(r) {
                match x.order {
                    Some(o) => {
                        write!(f, "{:3}", o)?;
                    }
                    None => {
                        write!(f, " -")?;
                    }
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

struct Model {}

fn model(app: &App) -> Model {
    app.new_window()
        .size(WIDTH, HEIGHT)
        .view(view)
        .build()
        .unwrap();
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("p", "png", "save frames to file as png.");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    let png = matches.opt_present("p");

    let draw = app.draw();
    if frame.nth() == 0 {
        draw.background().color(BLACK);
    }

    if png {
        let file_path = gif_path(app, &frame);
        app.main_window().capture_frame(file_path);
    }

    draw.to_frame(app, &frame).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_test() {
        let mut v = vec![Cell::new(); 5];
        assert_eq!(scan(&v, 2), 0);
        assert_eq!(scan(&v, 0), 0);
        assert_eq!(scan(&v, 4), 0);
        v[3].order = Some(0);
        assert_eq!(scan(&v, 2), 1);
        assert_eq!(scan(&v, 4), 1);
        assert_eq!(scan(&v, 1), 0);
        v[2].order = Some(0);
        assert_eq!(scan(&v, 1), 2);
        assert_eq!(scan(&v, 0), 0);
        assert_eq!(scan(&v, 4), 2);
        v[0].order = Some(0);
        assert_eq!(scan(&v, 1), 3);
        assert_eq!(scan(&v, 4), 2);
    }

    #[test]
    fn infl_test() {
        // let board = Board::new(4);
        // assert_eq!(board.influence(1, 1), 19);
        // assert_eq!(board.influence(0, 0), 17);
        let board = Board::new(3);
        let board = board.place(1, 1);
        let board = board.place(0, 0);
        let board = board.place(2, 2);
        // board.get_mut(1, 1).order = Some(0);
        // let board = board.place(0, 1);
        let board = board.influences();
        board.display_influence();
        dbg!(board.max_influence());
        // board.get_mut(0, 0).order = Some(1);
        // board.get_mut(2, 2).order = Some(2);
        // dbg!(board.influence(0, 2));
        // dbg!(board.influence(0, 1));
    }
}
