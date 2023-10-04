use std::str::FromStr;

#[allow(dead_code)]
static INPUT: &'static str = include_str!("./day02.txt");

#[derive(Debug, Clone, PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

impl Shape {
    #[allow(dead_code)]
    fn score_of_shape(&self) -> u64 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3
        }
    }

    #[allow(dead_code)]
    fn fight(&self, opp_choice: &Shape) -> FightResult {
        match (opp_choice, self) {
            (Self::Rock, Self::Scissors) => FightResult::Lose,
            (Self::Rock, Self::Paper) => FightResult::Win,
            (Self::Paper, Self::Rock) => FightResult::Lose,
            (Self::Paper, Self::Scissors) => FightResult::Win,
            (Self::Scissors, Self::Paper) => FightResult::Lose,
            (Self::Scissors, Self::Rock) => FightResult::Win,
            _ => FightResult::Draw
        }
    }
}

// to do .parse() you need FromStr trait
impl FromStr for Shape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "A" || s == "X" {
            Ok(Self::Rock)
        } else if s == "B" || s == "Y" {
            Ok(Self::Paper)
        } else if s == "C" || s == "Z" {
            Ok(Self::Scissors)
        } else {
            Err(())
        }
    }
}

#[derive(Debug)]
enum FightResult {
    Win,
    Lose,
    Draw
}

impl FromStr for FightResult {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "X" {
            Ok(FightResult::Lose)
        } else if s == "Y" {
            Ok(FightResult::Draw)
        } else if s == "Z" {
            Ok(FightResult::Win)
        } else {
            Err(())
        }
    }
}

impl FightResult {
    #[allow(dead_code)]
    fn score_of_result(&self) -> u64 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0
        }
    }

    #[allow(dead_code)]
    fn result_to_choice(&self, opponent_choice: &Shape) -> Shape {
        match (opponent_choice, self) {
            (Shape::Rock, Self::Lose) => Shape::Scissors,
            (Shape::Paper, Self::Lose) => Shape::Rock,
            (Shape::Rock, Self::Win) => Shape::Paper,
            (Shape::Paper, Self::Win) => Shape::Scissors,
            (Shape::Scissors, Self::Lose) => Shape::Paper,
            (Shape::Scissors, Self::Win) => Shape::Rock,
            _ => opponent_choice.clone()
        }
    }
}

#[allow(dead_code)]
fn parse_input_1(input: &str) -> Vec<(Shape, Shape)> {
    input.lines().filter_map(|chr| {
        let mut tokens = chr.split_whitespace();
        let opponent_token = tokens.next();
        let my_token = tokens.next();

        match (opponent_token, my_token) {
            (Some(opponent_token), Some(my_token)) => match (
                opponent_token.parse::<Shape>(),
                my_token.parse::<Shape>(),
            ) {
                (Ok(opponent_choice), Ok(my_choice)) => {
                    Some((opponent_choice, my_choice))
                }
                _ => None
            }
            _ => None
        }
    })
    .collect()
}

#[allow(dead_code)]
fn parse_input_2(input: &str) -> Vec<(Shape, FightResult)> {
    input.lines().filter_map(|chr| {
        let mut tokens = chr.split_whitespace();
        let opponent_token = tokens.next();
        let result_token = tokens.next();

        match (opponent_token, result_token) {
            (Some(opponent_token), Some(result_token)) => match (
                opponent_token.parse::<Shape>(),
                result_token.parse::<FightResult>()
            ) {
                (Ok(opponent_choice), Ok(result)) => {
                    Some((opponent_choice, result))
                }
                _ => None
            }
            _ => None
        }
    })
    .collect()
}

#[test]
fn part1() {
    let a = parse_input_1(INPUT);

    let b = a.iter()
        .map(|(opponent_choice, my_choice)| my_choice
            .fight(opponent_choice)
            .score_of_result() + my_choice.score_of_shape())
        .sum::<u64>();

    println!("{}", b);
}

#[test]
fn part2() {
    let a = parse_input_2(INPUT);

    let b = a.iter().map(|(opp_choice, outcome)| {
        outcome.result_to_choice(opp_choice)
            .fight(opp_choice)
            .score_of_result() + outcome.result_to_choice(opp_choice).score_of_shape()
    }).sum::<u64>();

    println!("{:?}", b);
}