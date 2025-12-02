use anyhow::Context;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn part1(input: BufReader<File>) -> anyhow::Result<i32> {
    let mut dial: i32 = 50;
    let mut ans = 0;
    for rotation in input.lines() {
        let rotation = rotation?;
        let direction = rotation.chars().next().context("bail")?;
        let amount = rotation.get(1..).context("bail")?.parse::<i32>()?;

        match direction {
            'R' => dial = (dial + amount) % 100,
            'L' => dial = (dial - amount + 100) % 100,
            _ => {}
        }

        if dial == 0 {
            ans += 1;
        }
    }

    Ok(ans)
}

// I can't be bothered being smart about this lol
fn part2(input: BufReader<File>) -> anyhow::Result<i32> {
    let mut dial: i32 = 50;
    let mut ans = 0;
    for rotation in input.lines() {
        let rotation = rotation?;
        let direction = rotation.chars().next().context("bail")?;
        let mut amount = rotation.get(1..).context("bail")?.parse::<i32>()?;

        match direction {
            'R' => {
                while amount > 0 {
                    dial = (dial + 1) % 100;
                    amount -= 1;
                    if dial == 0 {
                        ans += 1;
                    }
                }
            }

            'L' => {
                while amount > 0 {
                    dial = (dial - 1) % 100;
                    amount -= 1;
                    if dial == 0 {
                        ans += 1;
                    }
                }
            }
            _ => {}
        }
    }

    Ok(ans)
}

fn main() -> anyhow::Result<()> {
    let input = "./src/input.txt";

    println!("part one: {}", part1(BufReader::new(File::open(input)?))?);
    println!("part two: {}", part2(BufReader::new(File::open(input)?))?);

    Ok(())
}
