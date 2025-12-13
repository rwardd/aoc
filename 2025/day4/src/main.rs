use std::fs;

use anyhow::Context;

const ADJACENTS: [(i32, i32); 8] = [
    (1, 1),
    (1, 0),
    (1, -1),
    (0, 1),
    (0, -1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

fn in_bounds(map_size: (i32, i32), position: (i32, i32)) -> bool {
    if position.0 < 0 || position.0 >= map_size.0 || position.1 < 0 || position.1 >= map_size.1 {
        return false;
    }

    true
}

fn is_marked(rows: &[String], position: (usize, usize)) -> anyhow::Result<bool> {
    Ok(rows[position.1]
        .chars()
        .nth(position.0)
        .context(format!("bail, {}, {}", position.0, position.1))?
        == '@')
}

fn check_adj(rows: &[String], position: (usize, usize)) -> anyhow::Result<bool> {
    let mut count = 0;
    for (x, y) in ADJACENTS {
        let row = y + position.1 as i32;
        let col = x + position.0 as i32;
        if in_bounds((rows.len() as i32, rows[0].len() as i32), (row, col))
            && is_marked(rows, (col as usize, row as usize))?
        {
            count += 1
        }
    }
    Ok(count < 4)
}

fn part2(rows: &mut [String]) -> anyhow::Result<u64> {
    let mut count = 0;
    let mut prev_count = 0;
    loop {
        for row in 0..rows.len() {
            for col in 0..rows[0].len() {
                if is_marked(rows, (col, row))? && check_adj(rows, (col, row))? {
                    prev_count += 1;

                    rows[row].replace_range(col..col + 1, ".");
                }
            }
        }

        if prev_count == 0 {
            break;
        }

        count += prev_count;
        prev_count = 0;
    }
    Ok(count)
}

fn part1(rows: &[String]) -> anyhow::Result<u32> {
    let mut count = 0;
    for row in 0..rows.len() {
        for col in 0..rows[0].len() {
            if is_marked(rows, (col, row))? && check_adj(rows, (col, row))? {
                count += 1;
            }
        }
    }
    Ok(count)
}

fn main() -> anyhow::Result<()> {
    let input = "./src/input.txt";

    let mut rows: Vec<String> = fs::read_to_string(input)?
        .lines()
        .map(String::from)
        .collect();

    println!("part 1 {}", part1(&rows)?);
    println!("part 2 {}", part2(&mut rows)?);

    Ok(())
}
