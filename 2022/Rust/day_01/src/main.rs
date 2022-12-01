use std::fs;
use std::io;

fn parse_calorie_list(data: &str) -> Vec<i32> {
   let mut c_list: Vec<i32> = data 
    .split("\n\n")
    .map(|elf_inventory| 
         elf_inventory
         .split_terminator('\n')
         .map(|raw_value| 
              raw_value
              .parse::<i32>()
              .unwrap())
         .sum())
    .collect();
   c_list.sort_unstable();

   return c_list;
}

fn part_a(c_list: &[i32]) -> i32 {
    *c_list.last().unwrap()
}

fn part_b(c_list: &[i32]) -> i32 {
    let end = c_list.len();
    let start = end - 3;
    c_list[start..end].iter().sum()
}

fn main() -> Result<(), io::Error> {
    let input = fs::read_to_string("../../input/day_01.txt")?;
    let calorie_list: Vec<i32> = parse_calorie_list(&input); 
    println!("Part A -- The elf with the most calories has: {}", part_a(&calorie_list));


    println!("Part B -- The amount of calories the top three elfs are carrying: {}", part_b(&calorie_list));
    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn test_part_a() {
        let input = std::fs::read_to_string("../../input/day_01_ex.txt").unwrap();
        let result = super::part_a(&mut super::parse_calorie_list(&input));
        assert_eq!(result, 24000);

    }

    #[test]
    fn test_part_b() {
        let input = std::fs::read_to_string("../../input/day_01_ex.txt").unwrap();
        let result = super::part_b(&mut super::parse_calorie_list(&input));
        assert_eq!(result, 45000);
    }
}
