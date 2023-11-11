use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug)]
pub struct GameOfLife {
    max_x: usize,
    max_y: usize,
    cells: Vec<Vec<bool>>,
}

impl GameOfLife {
    pub fn new(max_x: usize, max_y: usize) -> Self {
        GameOfLife {
            max_x,
            max_y,
            cells: vec![vec![false; max_x]; max_y],
        }
    }

    pub fn load(&mut self, filename: &str) -> Result<()> {
        let content = fs::read_to_string(filename)?;
        let data: Vec<Point> = serde_json::from_str(&content)?;
        self.cells = vec![vec![false; self.max_x]; self.max_y];
        data.iter()
            .for_each(|point| self.set_alive(point.x, point.y));
        Ok(())
    }

    pub fn set_alive(&mut self, x: usize, y: usize) {
        if x < self.max_x && y < self.max_y {
            self.cells[y][x] = true;
        }
    }

    pub fn cells(&self) -> CellsIterator {
        CellsIterator::new(&self.cells, self.max_x, self.max_y)
    }

    pub fn step(&mut self) {
        let mut next_gen = vec![vec![false; self.max_x]; self.max_y];

        for y in 0..self.max_y {
            for x in 0..self.max_x {
                let adjacent_cells = self.get_adjacent_cells(x as i32, y as i32);
                let is_alive = self.cells[y][x];

                // A cell is alive in the next state if:
                // - It is currently alive and has 2 or 3 living neighbors, or
                // - It is currently dead and has exactly 3 living neighbors.
                next_gen[y][x] = adjacent_cells == 3 || (is_alive && adjacent_cells == 2);
            }
        }

        self.cells = next_gen;
    }

    fn get_adjacent_cells(&self, x: i32, y: i32) -> u8 {
        let mut adjacents = 0;
        let max_x = self.max_x as i32;
        let max_y = self.max_y as i32;

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let px = (x + dx + max_x) % max_x;
                let py = (y + dy + max_y) % max_y;

                if self.cells[py as usize][px as usize] {
                    adjacents += 1;
                }
            }
        }

        return adjacents;
    }
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
    pub alive: bool,
}

pub struct CellsIterator<'a> {
    max_x: usize,
    max_y: usize,
    cells: &'a [Vec<bool>],
    current: Cell,
}

impl<'a> CellsIterator<'a> {
    pub fn new(cells: &'a [Vec<bool>], max_x: usize, max_y: usize) -> Self {
        CellsIterator {
            max_x,
            max_y,
            cells,
            current: Cell {
                x: 0,
                y: 0,
                alive: if cells.is_empty() || cells[0].is_empty() {
                    false
                } else {
                    cells[0][0]
                },
            },
        }
    }
}

impl Iterator for CellsIterator<'_> {
    type Item = Cell;
    fn next(&mut self) -> Option<Self::Item> {
        let mut next = self.current.clone();

        next.x += 1;
        if next.x == self.max_x {
            next.x = 0;
            next.y += 1;
        }

        if next.y == self.max_y {
            None
        } else {
            next.alive = self.cells[next.y][next.x];
            let result = self.current.clone();
            self.current = next;
            Some(result)
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Point {
    x: usize,
    y: usize,
}
