use std::{collections::HashSet, fmt, u32};

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn coordinates(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum GridTile {
    Empty,
    Wall,
    Box,
    BoxLeft,
    BoxRight,
}

pub struct Grid {
    cells: Vec<Vec<GridTile>>,
    robot_position: (usize, usize),
    robot_moves: Vec<Direction>,
}

impl Grid {
    pub fn parse(grid_input: &str, moves_input: &str) -> Self {
        let width = grid_input.lines().next().unwrap().len();
        let height = grid_input.lines().count();
        let mut robot_position = (0, 0);
        let mut robot_moves = vec![];
        let mut cells = vec![vec![GridTile::Empty; width]; height];
        for (i, line) in grid_input.lines().enumerate() {
            for (j, character) in line.chars().enumerate() {
                match character {
                    '#' => cells[i][j] = GridTile::Wall,
                    '.' => cells[i][j] = GridTile::Empty,
                    'O' => cells[i][j] = GridTile::Box,
                    '@' => {
                        robot_position = (i, j);
                    }
                    _ => unreachable!(),
                }
            }
        }
        for move_character in moves_input.chars() {
            match move_character {
                '^' => robot_moves.push(Direction::Up),
                'v' => robot_moves.push(Direction::Down),
                '<' => robot_moves.push(Direction::Left),
                '>' => robot_moves.push(Direction::Right),
                _ => {}
            }
        }
        Grid {
            cells: cells,
            robot_position: robot_position,
            robot_moves: robot_moves,
        }
    }

    fn is_out_of_bounds(&self, x: usize, y: usize) -> bool {
        x >= self.cells.len() || y >= self.cells[0].len()
    }

    fn push(&mut self, mut x: usize, mut y: usize, dx: isize, dy: isize) -> bool {
        let mut box_positions = vec![];

        while !self.is_out_of_bounds(x, y) && self.cells[x][y] == GridTile::Box {
            box_positions.push((x, y));
            x = (x as isize + dx) as usize;
            y = (y as isize + dy) as usize;
        }

        if self.is_out_of_bounds(x, y) || self.cells[x][y] == GridTile::Wall {
            return false;
        }

        for &(bx, by) in box_positions.iter().rev() {
            let new_x = (bx as isize + dx) as usize;
            let new_y = (by as isize + dy) as usize;
            self.cells[bx][by] = GridTile::Empty;
            self.cells[new_x][new_y] = GridTile::Box;
        }

        true
    }

    fn search_double_boxes(
        &mut self,
        box_x: usize,
        box_y: usize,
        direction: &Direction,
        boxes: &mut HashSet<(usize, usize, usize, usize)>,
    ) -> bool {
        match self.cells[box_x][box_y] {
            GridTile::Empty => {
                return true;
            }
            GridTile::Wall => {
                return false;
            }
            GridTile::BoxLeft => match direction {
                Direction::Left | Direction::Right => {
                    boxes.insert((box_x, box_y, box_x, box_y + 1));
                    return self.search_double_boxes(
                        box_x,
                        (box_y as isize + direction.coordinates().1) as usize,
                        direction,
                        boxes,
                    );
                }
                Direction::Up | Direction::Down => {
                    boxes.insert((box_x, box_y, box_x, box_y + 1));
                    return self.search_double_boxes(
                        (box_x as isize + direction.coordinates().0) as usize,
                        box_y,
                        direction,
                        boxes,
                    ) && self.search_double_boxes(
                        (box_x as isize + direction.coordinates().0) as usize,
                        box_y + 1,
                        direction,
                        boxes,
                    );
                }
            },
            GridTile::BoxRight => match direction {
                Direction::Left | Direction::Right => {
                    boxes.insert((box_x, box_y - 1, box_x, box_y));
                    return self.search_double_boxes(
                        box_x,
                        (box_y as isize + direction.coordinates().1) as usize,
                        direction,
                        boxes,
                    );
                }
                Direction::Up | Direction::Down => {
                    boxes.insert((box_x, box_y - 1, box_x, box_y));
                    return self.search_double_boxes(
                        (box_x as isize + direction.coordinates().0) as usize,
                        box_y - 1,
                        direction,
                        boxes,
                    ) && self.search_double_boxes(
                        (box_x as isize + direction.coordinates().0) as usize,
                        box_y,
                        direction,
                        boxes,
                    );
                }
            },
            _ => {}
        }
        true
    }

    fn push_double_box(&mut self, box_x: usize, box_y: usize, direction: &Direction) -> bool {
        let mut boxes: HashSet<(usize, usize, usize, usize)> = HashSet::new();
        if self.search_double_boxes(box_x, box_y, direction, &mut boxes) {
            let (dx, dy) = direction.coordinates();
            for b in boxes.iter() {
                self.cells[b.0][b.1] = GridTile::Empty;
                self.cells[b.2][b.3] = GridTile::Empty;
            }
            for b in boxes.iter() {
                self.cells[(b.0 as isize + dx) as usize][(b.1 as isize + dy) as usize] =
                    GridTile::BoxLeft;
                self.cells[(b.2 as isize + dx) as usize][(b.3 as isize + dy) as usize] =
                    GridTile::BoxRight;
            }
            return true;
        }
        false
    }

    pub fn move_robot(&mut self) {
        let robot_moves = self.robot_moves.clone();
        for movement in robot_moves {
            let (dx, dy) = movement.coordinates();

            let (x, y) = self.robot_position;
            let next_x = (x as isize + dx) as usize;
            let next_y = (y as isize + dy) as usize;

            if self.is_out_of_bounds(next_x, next_y) || self.cells[next_x][next_y] == GridTile::Wall
            {
                continue;
            }

            match self.cells[next_x][next_y] {
                GridTile::Empty => {
                    self.robot_position = (next_x, next_y);
                }
                GridTile::Box => {
                    if self.push(next_x, next_y, dx, dy) {
                        self.robot_position = (next_x, next_y);
                    }
                }
                GridTile::BoxLeft | GridTile::BoxRight => {
                    if self.push_double_box(next_x, next_y, &movement) {
                        self.robot_position = (next_x, next_y);
                    }
                }
                _ => {}
            }
        }
    }

    fn gps_sum(&self) -> u32 {
        let mut sum = 0;

        for (y, row) in self.cells.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                match cell {
                    GridTile::Box | GridTile::BoxLeft => {
                        sum += (x + (y * 100)) as u32;
                    }
                    _ => {}
                }
            }
        }

        sum
    }

    pub fn expand(&mut self) {
        let mut expanded_cells = Vec::new();

        for row in &self.cells {
            let mut expanded_row = Vec::new();
            for &cell in row {
                match cell {
                    GridTile::Wall => {
                        expanded_row.push(GridTile::Wall);
                        expanded_row.push(GridTile::Wall);
                    }
                    GridTile::Box => {
                        expanded_row.push(GridTile::BoxLeft);
                        expanded_row.push(GridTile::BoxRight);
                    }
                    GridTile::Empty => {
                        expanded_row.push(GridTile::Empty);
                        expanded_row.push(GridTile::Empty);
                    }
                    _ => {}
                }
            }
            expanded_cells.push(expanded_row);
        }

        let robot_position = (
            self.robot_position.0,
            self.robot_position.1 * 2, // Double the column index for the robot's position
        );
        self.robot_position = robot_position;
        self.cells = expanded_cells
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, row) in self.cells.iter().enumerate() {
            for (j, &tile) in row.iter().enumerate() {
                let symbol = if (i, j) == self.robot_position {
                    '@'
                } else {
                    match tile {
                        GridTile::Empty => '.',
                        GridTile::Wall => '#',
                        GridTile::Box => 'O',
                        GridTile::BoxLeft => '[',
                        GridTile::BoxRight => ']',
                    }
                };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn generator(input: &str) -> Grid {
    let (grid_input, moves_input) = input.split_once("\n\n").unwrap();
    let grid = Grid::parse(grid_input, moves_input);
    grid
}

pub fn part_one(mut grid: Grid) -> u32 {
    grid.move_robot();
    grid.gps_sum()
}

pub fn part_two(mut grid: Grid) -> u32 {
    grid.expand();
    grid.move_robot();
    grid.gps_sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {}

    #[test]
    fn test_part_two_left_shift() {
        let test_input = "#######
#...#.#
#.....#
#.@OO.#
#..O..#
#.....#
#######

>>>";
        assert_eq!(part_two(generator(&test_input)), 1024)
    }

    #[test]
    fn test_part_two_right_shift() {
        let test_input = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<<<";
        assert_eq!(part_two(generator(&test_input)), 1014)
    }
}
