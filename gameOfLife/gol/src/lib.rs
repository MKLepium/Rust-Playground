use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Universe {
    width: u32,
    height: u32,
    generations: u32,
    live_cells: u32,
    array: Vec<bool>,
}

#[derive(Debug)]
pub struct Game {
    universe: Universe,
}

impl Game {
    /*
    Creates new Game Object, call can be Game::new(None, None) to create 12x12 size
    */
    pub fn new(width: Option<u32>, height: Option<u32>) -> Self {
        Self {
            universe: Universe::new(width.unwrap_or(12), height.unwrap_or(12)),
        }
    }
    pub fn randomize(&mut self) {
        self.universe.randomize();
    }

    pub fn change_size(&mut self, width: u32, height: u32) -> String {
        self.universe.generations = 0;
        self.universe.live_cells = 0;
        self.universe = Universe::new(width, height);
        self.universe.randomize();
        self.universe.live_cells_count();
        self.serialize()
    }

    pub fn change_value(&mut self, index: usize) {
        // the index could be out of bounds and kill the server if a mallicious user gets into play
        if *self.universe.array.get_mut(index).unwrap() {
            self.universe.array[index] = false;
        } else {
            self.universe.array[index] = true;
        }
        self.universe.live_cells_count();
    }

    pub fn negate_everything(&mut self) -> String {
        self.universe.generations = 0;
        self.universe.live_cells = 0;
        self.universe.array = vec![false; self.universe.array.len()];
        self.serialize()
    }

    pub fn serialize(&mut self) -> String {
        // It is safe to do so, according to serde documentation
        let x: String = serde_json::to_string_pretty(&self.universe).unwrap();
        x
    }
    pub fn tick(&mut self) {
        self.universe.tick();
        //println!("Hi from tick");
    }
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    pub fn randomize(&mut self) {
        self.generations = 0;
        self.live_cells_count();
        let mut new = Vec::new();
        let mut rand = rand::thread_rng();
        while new.len() != self.array.len() {
            let random = rand.gen_range(0.0..1.0);
            if random <= 0.7 {
                new.push(false);
            } else {
                new.push(true);
            }
        }
        self.array = new;
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.array[idx] as u8;
            }
        }
        count
    }

    pub fn live_cells_count(&mut self){
        self.live_cells = 0;
        for idx in 0..self.array.len() {
            if self.array[idx] == true {
                self.live_cells += 1;
            }
        }
    }

    pub fn new(width: u32, height: u32) -> Self {
        let x = width * height;
        let size = x as usize;
        Self {
            width,
            height,
            generations: 0,
            live_cells: 0,
            array: vec![false; size],
        }
    }

    pub fn tick(&mut self) {
        let mut next = self.array.clone();
        self.generations += 1;
        self.live_cells_count();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.array[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (true, x) if x < 2 => false,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (true, 2) | (true, 3) => true,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (true, x) if x > 3 => false,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (false, 3) => true,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };
                next[idx] = next_cell;
            }
        }
        self.array = next;
    }
}
