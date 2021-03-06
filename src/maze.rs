use rand::{prelude::SliceRandom, Rng};
use std::collections::HashSet;

enum Direction {
    Up = 1,
    Right = 2,
    Down = 4,
    Left = 8,
}

type Grid = Vec<usize>;

struct Neighbour(Direction, usize);

fn get_neighbours(current_index: usize, columns: usize, rows: usize) -> Vec<Neighbour> {
    let mut neighbours: Vec<Neighbour> = Vec::new();

    if current_index >= columns * rows {
        return neighbours;
    }
    if current_index >= columns {
        neighbours.push(Neighbour(Direction::Up, current_index - columns));
    }
    if current_index % columns != columns - 1 {
        neighbours.push(Neighbour(Direction::Right, current_index + 1));
    }
    if current_index < columns * rows - columns {
        neighbours.push(Neighbour(Direction::Down, current_index + columns));
    }
    if current_index % columns > 0 {
        neighbours.push(Neighbour(Direction::Left, current_index - 1));
    }
    neighbours
}

fn walk<R: Rng>(grid: &mut Grid, columns: usize, rows: usize, rng: &mut R) {
    let size = columns * rows;
    let mut index = rng.gen_range(0..(size - 1));
    let mut stack = Vec::with_capacity(size);
    let mut visited: HashSet<usize> = HashSet::with_capacity(size);

    stack.push(index);
    visited.insert(index);

    while visited.len() < size {
        let neighbours: Vec<Neighbour> = get_neighbours(index, columns, rows)
            .into_iter()
            .filter(|x| !visited.contains(&x.1))
            .collect();

        if neighbours.len() > 0 {
            let neighbour = neighbours.choose(rng);
            match neighbour {
                None => {
                    println!("No neighbours!");
                }
                Some(neighbour) => {
                    match neighbour {
                        Neighbour(Direction::Up, next_index) => {
                            grid[index] ^= Direction::Up as usize;
                            grid[*next_index] ^= Direction::Down as usize;
                        }
                        Neighbour(Direction::Right, next_index) => {
                            grid[index] ^= Direction::Right as usize;
                            grid[*next_index] ^= Direction::Left as usize;
                        }
                        Neighbour(Direction::Down, next_index) => {
                            grid[index] ^= Direction::Down as usize;
                            grid[*next_index] ^= Direction::Up as usize;
                        }
                        Neighbour(Direction::Left, next_index) => {
                            grid[index] ^= Direction::Left as usize;
                            grid[*next_index] ^= Direction::Right as usize;
                        }
                    }

                    let next_index = neighbour.1;
                    index = next_index;
                    stack.push(next_index);
                    visited.insert(next_index);
                }
            }
            continue;
        }

        match stack.pop() {
            None => panic!("There shouldn't be an empty stack here"),
            Some(i) => {
                index = i;
            }
        }
    }
}

pub fn gen<R: Rng>(columns: usize, rows: usize, rng: &mut R) -> Grid {
    let size = columns * rows;
    let mut grid: Grid = vec![0; size];
    walk(&mut grid, columns, rows, rng);
    grid
}
