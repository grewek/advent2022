use std::fs;
use std::io;

fn main() -> Result<(), io::Error> {
    let input = fs::read_to_string("../../input/day_06.txt")?;
    let result_a = part_a(input.as_bytes());
    let result_b = part_b(input.as_bytes());

    println!("The start-of-packet marker appears at position: {}", result_a);
    println!("The start-of-message marker appears at position: {}", result_b);
    Ok(())
}

fn part_a(data: &[u8]) -> usize {
    let marker_length = data.windows(4).map(|byte_window| {
        let mut possible_marker = std::collections::HashSet::new();
        for byte in byte_window {
            possible_marker.insert(byte);
        }

        println!("{:?}", possible_marker);
        possible_marker.len()
    });

    let mut stream_position = 4;
    for length in marker_length {
       if length == 4 {
           break;
       }

       stream_position += 1;
    }

    stream_position
}
fn part_b(data: &[u8]) -> usize {
    let marker_length = data.windows(14).map(|byte_window| {
        let mut possible_marker = std::collections::HashSet::new();
        for byte in byte_window {
            possible_marker.insert(byte);
        }

        println!("{:?}", possible_marker);
        possible_marker.len()
    });

    let mut stream_position = 14;
    for length in marker_length {
       if length == 14 {
           break;
       }

       stream_position += 1;
    }

    stream_position
}

#[cfg(test)]
mod test {
    #[test]
    fn test_part_a() {
        let input = std::fs::read_to_string("../../input/day_06_ex.txt").unwrap();
        let expected = 7;
        let result = super::part_a(input.as_bytes());

        assert_eq!(result, expected);
        
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let expected = 5;
        let result = super::part_a(input.as_bytes());

        assert_eq!(result, expected);

        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let expected = 10;
        let result = super::part_a(input.as_bytes());

        assert_eq!(result, expected);

        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let expected = 11;
        let result = super::part_a(input.as_bytes());

        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_b() {
        let input = std::fs::read_to_string("../../input/day_06_ex.txt").unwrap();
        let expected = 19;
        let result = super::part_b(input.as_bytes());

        assert_eq!(result, expected);
        
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let expected = 23;
        let result = super::part_b(input.as_bytes());

        assert_eq!(result, expected);

        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let expected = 29;
        let result = super::part_b(input.as_bytes());

        assert_eq!(result, expected);

        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let expected = 26;
        let result = super::part_b(input.as_bytes());

        assert_eq!(result, expected);
    }
}
