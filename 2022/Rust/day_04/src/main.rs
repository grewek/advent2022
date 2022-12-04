use std::fs;
use std::io;

fn parse_section_data(input: &str) -> Vec<i32> {
    input
        .split_terminator(&[',','\n'])
        .flat_map(|section| section.split('-')
                .map(|value|
                     value.parse::<i32>().expect("Error not a valid value"))
            )
        .collect()
}

fn check_fits_in_range(min_a: i32, min_b: i32, max_a: i32, max_b: i32) -> bool {
    if min_a >= min_b && max_a <= max_b || min_b >= min_a && max_b <= max_a {
        return true;
    }

    return false;
}

fn check_overlaps(min_a: i32, min_b: i32, max_a: i32, max_b: i32) -> bool {
    if max_a < min_b || max_b < min_a {
        return false;
    }

    return true;
}

fn part_a(section_data: &[i32]) -> usize {
    section_data
        .chunks(4)
        .filter(|data| check_fits_in_range(data[0], data[2], data[1], data[3]))
        .count()
}

fn part_b(section_data: &[i32]) -> usize {
    section_data
        .chunks(4)
        .filter(|data| check_overlaps(data[0], data[2], data[1], data[3]))
        .count()
}

fn main() -> Result<(), io::Error> {
    let input = fs::read_to_string("../../input/day_04.txt")?;
    let section_data = parse_section_data(&input);

    println!("There are sections that completly fit into each other for {} elves", part_a(&section_data));
    println!("Thera are sections that overlap for {} elves", part_b(&section_data));
    Ok(())
}

