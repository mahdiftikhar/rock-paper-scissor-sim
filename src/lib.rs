extern crate rand;
extern crate sdl2;

use rand::random;

#[derive(Debug, Clone, Copy)]
pub enum PixelType {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
pub struct Grid {
    rows: usize,
    cols: usize,
    pub grid: Vec<Vec<PixelType>>,
}

impl Grid {
    pub fn new(rows: usize, cols: usize) -> Grid {
        let mut grid: Vec<Vec<PixelType>> = vec![];
        for _ in 0..rows {
            let mut temp_row = vec![];
            for _ in 0..cols {
                let random_number = random::<u8>() % 3;

                let pixel_type: PixelType = match random_number {
                    0 => PixelType::Rock,
                    1 => PixelType::Paper,
                    2 => PixelType::Scissors,
                    _ => unreachable!(),
                };

                temp_row.push(pixel_type);
            }
            grid.push(temp_row);
        }
        Grid { rows, cols, grid }
    }

    pub fn get_rows(&self) -> usize {
        self.rows
    }

    pub fn get_cols(&self) -> usize {
        self.cols
    }

    pub fn get_neighbours(grid: &Vec<Vec<PixelType>>, i: i32, j: i32) -> Vec<PixelType> {
        let mut neighbors = vec![];
        for x in i - 1..i + 2 {
            for y in j - 1..j + 2 {
                if grid.get(x as usize).is_none() || grid[x as usize].get(y as usize).is_none() {
                    continue;
                }
                neighbors.push(grid[x as usize][y as usize]);
            }
        }

        neighbors
    }

    pub fn update_players(&mut self) {
        let mut grid = self.grid.clone();

        for i in 0..self.rows {
            for j in 0..self.cols {
                // let counter: HashMap<u8> = hash
                let neighbors = Grid::get_neighbours(&self.grid, i as i32, j as i32);
                let mut paper_counter = 0;
                let mut rock_counter = 0;
                let mut scissor_counter = 0;

                for neighbor in neighbors.iter() {
                    match neighbor {
                        PixelType::Paper => paper_counter += 1,
                        PixelType::Rock => rock_counter += 1,
                        PixelType::Scissors => scissor_counter += 1,
                    }
                }

                grid[i][j] = match self.grid[i][j] {
                    PixelType::Rock => {
                        if paper_counter >= 3 {
                            PixelType::Paper
                        } else {
                            PixelType::Rock
                        }
                    }
                    PixelType::Paper => {
                        if scissor_counter >= 3 {
                            PixelType::Scissors
                        } else {
                            PixelType::Paper
                        }
                    }
                    PixelType::Scissors => {
                        if rock_counter >= 3 {
                            PixelType::Rock
                        } else {
                            PixelType::Scissors
                        }
                    }
                };
            }
        }
        self.grid = grid;
    }
}
