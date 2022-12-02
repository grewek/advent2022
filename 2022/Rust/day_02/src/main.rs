use std::fs;
use std::str::FromStr;

enum GameResult {
    Win,
    Draw,
    Loose,
}
#[derive(Debug)]
enum FiguresError {
    InvalidFigure,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum GameFigures {
    Rock,
    Paper,
    Scissor,
}

impl GameFigures {
    fn figure_score(&self) -> i32 {
        match self {
            GameFigures::Rock => 1,
            GameFigures::Paper => 2,
            GameFigures::Scissor => 3,
        }
    }

    fn figure_to(&self, desired_result: GameResult) -> Self {
        match (desired_result, self) {
            (GameResult::Win, GameFigures::Rock) => GameFigures::Paper,
            (GameResult::Win, GameFigures::Paper) => GameFigures::Scissor,
            (GameResult::Win, GameFigures::Scissor) => GameFigures::Rock,
            (GameResult::Loose, GameFigures::Rock) => GameFigures::Scissor,
            (GameResult::Loose, GameFigures::Paper) => GameFigures::Rock,
            (GameResult::Loose, GameFigures::Scissor) => GameFigures::Paper,
            (GameResult::Draw, _) => *self,
        }
    }
}

impl From<GameFigures> for GameResult {
    fn from(fig: GameFigures) -> Self {
        match fig {
            GameFigures::Rock => GameResult::Loose,
            GameFigures::Paper => GameResult::Draw,
            GameFigures::Scissor => GameResult::Win,
        }
    }
}
impl FromStr for GameFigures {
    type Err = FiguresError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.as_bytes() {
            b"A" | b"X" => Ok(GameFigures::Rock),
            b"B" | b"Y" => Ok(GameFigures::Paper),
            b"C" | b"Z" => Ok(GameFigures::Scissor),
            _   => Err(FiguresError::InvalidFigure),
        }
    }
}

fn score_game(opponent: GameFigures, player: GameFigures) -> i32 {
    match (opponent, player) {
        (GameFigures::Rock, GameFigures::Paper) => 6,
        (GameFigures::Paper, GameFigures::Scissor) => 6,
        (GameFigures::Scissor, GameFigures::Rock) => 6,
        (GameFigures::Rock, GameFigures::Rock) => 3,
        (GameFigures::Paper, GameFigures::Paper) => 3,
        (GameFigures::Scissor, GameFigures::Scissor) => 3,
        (_, _) => 0, //draw !
    }
}

fn parse_strategy_guide(input: &str) -> Vec<GameFigures> {
    input
        .split_terminator('\n')
        .map(|game|
             game
             .split_whitespace()
             .map(|game_figure|
                  game_figure
                  .parse::<GameFigures>()
                  .unwrap())
        )
        .flatten()
        .collect()
}

fn part_a(games: &[GameFigures]) -> i32 {
    let mut score = 0;
    for figures in games.chunks(2) {
        score += figures[1].figure_score() + score_game(figures[0], figures[1]);
    }

    score
}

fn part_b(games: &[GameFigures]) -> i32 {
    let mut score = 0;

    for figure in games.chunks(2) {
        let target_result = GameResult::from(figure[1]);
        let my_figure = figure[0].figure_to(target_result);

        score += my_figure.figure_score() + score_game(figure[0], my_figure);
    }

    score
}

fn main() -> Result<(), std::io::Error> {
    let input = fs::read_to_string("../../input/day_02.txt")?;
    let guide = &parse_strategy_guide(&input);

    let result_a = part_a(&guide);
    let result_b = part_b(&guide);

    println!("Your score after following the strategy guide: {}", result_a);
    println!("Your score after changing the strategy guide: {}", result_b);
    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn test_parsing_strategy_guide() {
        let input = std::fs::read_to_string("../../input/day_02_ex.txt").unwrap();
        let expected = vec![
            super::GameFigures::Rock, super::GameFigures::Paper,
            super::GameFigures::Paper, super::GameFigures::Rock,
            super::GameFigures::Scissor, super::GameFigures::Scissor
        ];
        let result = super::parse_strategy_guide(&input);

        for (fig_result, fig_expected) in result.iter().zip(expected.iter()) {
            assert_eq!(fig_result, fig_expected);
        }
    }

    #[test]
    fn test_part_a_example() {
        let input = std::fs::read_to_string("../../input/day_02_ex.txt").unwrap();
        let parsed_guide = super::parse_strategy_guide(&input);
        let result = super::part_a(&parsed_guide);

        assert_eq!(result, 15);
    }

    #[test]
    fn test_part_b_example() {
        let input = std::fs::read_to_string("../../input/day_02_ex.txt").unwrap();
        let parsed_guide = super::parse_strategy_guide(&input);
        let result = super::part_b(&parsed_guide);

        assert_eq!(result, 12);
    }
}
