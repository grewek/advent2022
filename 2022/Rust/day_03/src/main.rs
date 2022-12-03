use std::fs;
use std::io;
use std::collections::HashSet;

#[derive(Debug)]
struct Rucksack {
    whole: HashSet<char>,
    front: HashSet<char>,
    back: HashSet<char>,
}

fn get_priority(letter: char) -> i32 {
    if char::is_ascii_uppercase(&letter) {
        return (letter as i32 - 65) + 27;
    } else if char::is_ascii_lowercase(&letter) {
        return (letter as i32 - 97) + 1;
    } else {
        panic!("Cannot convert {} because it is not in the valid range please use only ascii letters !", letter);
    }
}


fn parse_rucksack_content(input: &str) -> Vec<Rucksack> {
    input
        .split_terminator('\n')
        .map(|slice| { 
            let half_size = slice.len()/2;
            Rucksack {
                whole: slice.chars().fold(HashSet::new(), |mut set, c| { 
                    set.insert(c);  
                    set 
                }),

                front: slice.chars().take(half_size).fold(HashSet::new(), |mut set, c| { 
                    set.insert(c);  
                    set 
                }),
                back: slice.chars().skip(half_size).fold(HashSet::new(), |mut set, c| {
                    set.insert(c);
                    set
                })
            }
        })
        .collect()
}

fn main() -> Result<(), io::Error> {
    let input = fs::read_to_string("../../input/day_03.txt")?;
    let sets = parse_rucksack_content(&input);
    let priority_a = part_a(&sets);
    let priority_b = part_b(&sets);
    println!("The sum of all priorities is {}", priority_a);
    println!("The sum of the group badge priorities is {}", priority_b);

    Ok(())
}

fn part_a(rlist: &[Rucksack]) -> i32 {
    let mut score = 0;
    for  rucksack in rlist {
        for letter in rucksack.front.intersection(&rucksack.back) {
            score += get_priority(*letter);
        }
    }

    score
}

fn part_b(rlist: &[Rucksack]) -> i32 {
    let mut score = 0;
    for rucksack in rlist.chunks(3) {
        let r0 = &rucksack[0];
        let r1 = &rucksack[1];
        let r2 = &rucksack[2];

        let intermediate = &r0.whole & &r1.whole;
        let badge_letter = &intermediate & &r2.whole;
        for letter in badge_letter {
            score += get_priority(letter);
        }
    }

    score
}

#[cfg(test)]
mod test {
    use std::fs;

    #[test]
    fn test_example_part_a() {
        let input = fs::read_to_string("../../input/day_03_ex.txt").unwrap();
        let rlist = super::parse_rucksack_content(&input);
        let result = super::part_a(&rlist);
        let expected = 157;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_example_part_b() {
        let input = fs::read_to_string("../../input/day_03_ex.txt").unwrap();
        let rlist = super::parse_rucksack_content(&input);
        let result = super::part_b(&rlist);
        let expected = 70;

        assert_eq!(result, expected);
    }
}
