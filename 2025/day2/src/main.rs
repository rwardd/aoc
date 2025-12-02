use std::fs;

fn part1(input: &[(u64, u64)]) -> u64 {
    input
        .iter()
        .flat_map(|a| {
            (a.0..=a.1).filter(|id| {
                let num_digits = u64::ilog10(*id) + 1;
                if !num_digits.is_multiple_of(2) {
                    return false;
                }

                let divisor = u64::pow(10, num_digits / 2);
                id / divisor == id % divisor
            })
        })
        .sum()
}

fn main() -> anyhow::Result<()> {
    let input = "./src/input.txt";

    let part1_input: Vec<(u64, u64)> = fs::read_to_string(input)?
        .trim()
        .split([','])
        .map(|range| range.split('-'))
        .filter_map(|mut range| Some((range.next()?.parse().ok()?, range.next()?.parse().ok()?)))
        .collect();

    println!("part one: {}", part1(&part1_input));

    Ok(())
}
