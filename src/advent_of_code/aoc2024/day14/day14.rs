use regex::Regex;

#[derive(Debug)]
struct Robot {
    position: [u8; 2],
    velocity: [i8; 2],
}

pub fn generator(input: &str) -> String {
    input.to_string()
}

fn parse_input(input: String) -> Vec<Robot> {
    let pattern = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut robots = Vec::new();

    for line in input.lines() {
        if let Some(regex_match) = pattern.captures(line) {
            let pos_x: u8 = regex_match[1].parse().unwrap();
            let pos_y: u8 = regex_match[2].parse().unwrap();
            let velocity_x: i8 = regex_match[3].parse().unwrap();
            let velocity_y: i8 = regex_match[4].parse().unwrap();

            let robot = Robot {
                position: [pos_x, pos_y],
                velocity: [velocity_x, velocity_y],
            };
            robots.push(robot)
        }
    }

    robots
}

pub fn part_one(input: String) -> u32 {
    let mut robots = parse_input(input);
    let grid_width: usize = 101;
    let grid_height: usize = 103;

    for robot in robots.iter_mut() {
        robot.position[0] = ((robot.position[0] as i16 + (robot.velocity[0] as i32 * 100) as i16)
            .rem_euclid(grid_width as i16)) as u8;
        robot.position[1] = ((robot.position[1] as i16 + (robot.velocity[1] as i32 * 100) as i16)
            .rem_euclid(grid_height as i16)) as u8;
    }

    let mid_x: u8 = (grid_width as u8) / 2;
    let mid_y: u8 = (grid_height as u8) / 2;

    let mut top_left = 0;
    let mut top_right = 0;
    let mut bottom_left = 0;
    let mut bottom_right = 0;

    for robot in &robots {
        let (x, y) = (robot.position[0], robot.position[1]);
        if x < mid_x && y < mid_y {
            top_left += 1;
        } else if x > mid_x && y < mid_y {
            top_right += 1;
        } else if x < mid_x && y > mid_y {
            bottom_left += 1;
        } else if x > mid_x && y > mid_y {
            bottom_right += 1;
        }
    }
    top_left * top_right * bottom_left * bottom_right
}

fn calculate_variance(
    robots: &Vec<Robot>,
    step: u32,
    grid_width: usize,
    grid_height: usize,
) -> (f64, f64) {
    let mut sum_y: i64 = 0;
    let mut sum_x: i64 = 0;
    let mut sum2_y: i64 = 0;
    let mut sum2_x: i64 = 0;

    for robot in robots {
        let pos_y = (((robot.position[1] as i64
            + step as i64 * (robot.velocity[1] as i64 + grid_height as i64))
            % grid_height as i64)
            + grid_height as i64)
            % grid_height as i64;
        let pos_x = (((robot.position[0] as i64
            + step as i64 * (robot.velocity[0] as i64 + grid_width as i64))
            % grid_width as i64)
            + grid_width as i64)
            % grid_width as i64;

        sum_y += pos_y;
        sum_x += pos_x;
        sum2_y += pos_y * pos_y;
        sum2_x += pos_x * pos_x;
    }

    let n = robots.len() as f64;
    let var_y = (sum2_y as f64 - (sum_y as f64 * sum_y as f64) / n) / n;
    let var_x = (sum2_x as f64 - (sum_x as f64 * sum_x as f64) / n) / n;

    (var_y, var_x)
}

pub fn part_two(input: String) -> u32 {
    let robots = parse_input(input);
    let grid_width: usize = 101;
    let grid_height: usize = 103;

    let mut step_y = 0;
    let mut step_x = 0;
    let mut min_var_y = f64::MAX;
    let mut min_var_x = f64::MAX;

    for step in (1..=103).rev() {
        let (var_y, var_x) = calculate_variance(&robots, step, grid_width, grid_height);

        if var_y < min_var_y {
            min_var_y = var_y;
            step_y = step;
        }
        if var_x < min_var_x {
            min_var_x = var_x;
            step_x = step;
        }
    }

    (step_x as i64
        + ((51 as i64 * (step_y as i64 - step_x as i64).rem_euclid(grid_height as i64))
            .rem_euclid(grid_height as i64))
            * grid_width as i64) as u32
}
#[cfg(test)]
mod tests {}
