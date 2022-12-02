use itertools::{FoldWhile, Itertools};
use std::str::FromStr;

type Round = BaseRound<OutcomeDirection>;
type MisinterpretedRound = BaseRound<MisinterpretedPlayerMove>;

#[derive(Debug, Copy, Clone)]
enum GameMove {
    Rock,
    Paper,
    Scissor,
}

enum RoundOutcome {
    PlayerWins(PlayerMove),
    OpponentWins(PlayerMove),
    Draw(PlayerMove),
}

#[derive(Debug, Copy, Clone)]
struct OpponentMove(GameMove);

#[derive(Debug, Copy, Clone)]
struct MisinterpretedPlayerMove(GameMove);
struct PlayerMove(GameMove);

#[derive(Debug, Copy, Clone)]
enum OutcomeDirection {
    PlayerWins,
    OpponentWins,
    Draw,
}

#[derive(Debug)]
enum GameMoveParsingError {
    UnknownMove {
        supplied_play: String,
        is_player: bool,
    },
    UnknowOutcomeDirection {
        supplied_direction: String,
    },
}

impl GameMoveParsingError {
    fn unknown_move(s: &str, is_player: bool) -> GameMoveParsingError {
        GameMoveParsingError::UnknownMove {
            supplied_play: s.to_string(),
            is_player,
        }
    }

    fn unknown_direction(s: &str) -> GameMoveParsingError {
        GameMoveParsingError::UnknowOutcomeDirection {
            supplied_direction: s.to_string(),
        }
    }
}

#[derive(Debug)]
enum RoundParsingError {
    LessMovesThanPlayers { supplied_moves: Vec<String> },
    MoreMovesThanPlayers { supplied_moves: Vec<String> },
    UnknownPlayerMove(GameMoveParsingError),
    UnknownOpponentMove(GameMoveParsingError),
    UnknownMoves(GameMoveParsingError, GameMoveParsingError),
}

struct BaseRound<T>(OpponentMove, T);

impl<T> FromStr for BaseRound<T>
where
    T: FromStr<Err = GameMoveParsingError>,
{
    type Err = RoundParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let space_split: &str = " ";

        let result: Vec<String> = s
            .trim()
            .split(space_split)
            .map(|s| s.to_string())
            .collect_vec();

        if result.len() < 2 {
            return Err(RoundParsingError::LessMovesThanPlayers {
                supplied_moves: result,
            });
        }

        if result.len() > 2 {
            return Err(RoundParsingError::MoreMovesThanPlayers {
                supplied_moves: result,
            });
        }

        let opponent_move: Result<OpponentMove, GameMoveParsingError> = result
            .get(0)
            .expect("we checked indexes above")
            .parse::<OpponentMove>();
        let player_move: Result<T, GameMoveParsingError> = result
            .get(1)
            .expect("we checked indexes above")
            .parse::<T>();

        match (opponent_move, player_move) {
            (Ok(opponent_move), Ok(player_move)) => Ok(BaseRound(opponent_move, player_move)),
            (Err(opp_err), Ok(_)) => Result::Err(RoundParsingError::UnknownOpponentMove(opp_err)),
            (Ok(_), Err(player_err)) => {
                Result::Err(RoundParsingError::UnknownPlayerMove(player_err))
            }
            (Err(opp_err), Err(player_err)) => {
                Result::Err(RoundParsingError::UnknownMoves(opp_err, player_err))
            }
        }
    }
}

impl FromStr for OpponentMove {
    type Err = GameMoveParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "A" => Ok(OpponentMove(GameMove::Rock)),
            "B" => Ok(OpponentMove(GameMove::Paper)),
            "C" => Ok(OpponentMove(GameMove::Scissor)),
            other => Err(GameMoveParsingError::unknown_move(other, true)),
        }
    }
}

impl ToString for GameMoveParsingError {
    fn to_string(&self) -> String {
        match self {
            Self::UnknownMove {
                supplied_play,
                is_player,
            } => {
                let player_or_opponent_str = if !*is_player { "player" } else { "opponent" };
                let expected: &str = if !*is_player { "X, Y, Z" } else { "A, B, C" };
                format!(
                    "supplied game move [{}] for [{}] expected one in [{}] as [Rock, Paper, Scissor]",
                    supplied_play, player_or_opponent_str, expected
                )
            }
            Self::UnknowOutcomeDirection { supplied_direction } => {
                let expected: &str = "X, Y, Z";
                format!(
                    "supplied direction [{}] expected one in [{}] as [OpponentWins, Draw, PlayerWins]",
                    supplied_direction, expected
                )
            }
        }
    }
}

impl ToString for RoundParsingError {
    fn to_string(&self) -> String {
        match self {
            RoundParsingError::LessMovesThanPlayers { supplied_moves } => {
                format!(
                    "Less moves than players have been supplied\n\tsupplied: {}",
                    &supplied_moves.iter().join(",")
                )
            }
            RoundParsingError::MoreMovesThanPlayers { supplied_moves } => {
                format!(
                    "More moves than players have been supplied\n\tsupplied: {}",
                    &supplied_moves.iter().join(",")
                )
            }
            RoundParsingError::UnknownPlayerMove(e) => {
                format!(
                    "An invalid move or direction has been supplied for:\n\tPlayer:{}",
                    e.to_string()
                )
            }
            RoundParsingError::UnknownOpponentMove(e) => {
                format!(
                    "An invalid move or direction has been supplied for:\n\tOpponent:{}",
                    e.to_string()
                )
            }

            RoundParsingError::UnknownMoves(opponent_err, player_err) => {
                format!(
                    "Invalid moves or directions supplied for all the players:\n\tOpponent:{}\n\tPlayer:{}",
                    opponent_err.to_string(),
                    player_err.to_string()
                )
            }
        }
    }
}

impl FromStr for MisinterpretedPlayerMove {
    type Err = GameMoveParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "X" => Ok(MisinterpretedPlayerMove(GameMove::Rock)),
            "Y" => Ok(MisinterpretedPlayerMove(GameMove::Paper)),
            "Z" => Ok(MisinterpretedPlayerMove(GameMove::Scissor)),
            other => Err(GameMoveParsingError::unknown_move(other, false)),
        }
    }
}

trait Solvable {
    fn solve(&self) -> RoundOutcome;
}

impl Solvable for Round {
    fn solve(&self) -> RoundOutcome {
        let player_move = plan_move(self);
        solve_round_outcome(self.0, player_move)
    }
}

impl Solvable for MisinterpretedRound {
    fn solve(&self) -> RoundOutcome {
        solve_round_outcome(self.0, PlayerMove(self.1 .0))
    }
}

impl FromStr for OutcomeDirection {
    type Err = GameMoveParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "X" => Ok(OutcomeDirection::OpponentWins),
            "Y" => Ok(OutcomeDirection::Draw),
            "Z" => Ok(OutcomeDirection::PlayerWins),
            other => Err(GameMoveParsingError::unknown_direction(other)),
        }
    }
}

fn solve_match(player: &GameMove, opponent: &GameMove) -> bool {
    matches!(
        (player, opponent),
        (GameMove::Rock, GameMove::Scissor)
            | (GameMove::Paper, GameMove::Rock)
            | (GameMove::Scissor, GameMove::Paper)
    )
}

fn solve_round_outcome(opponent_move: OpponentMove, player_move: PlayerMove) -> RoundOutcome {
    let player_wins = solve_match(&player_move.0, &opponent_move.0);
    let opponent_wins = solve_match(&opponent_move.0, &player_move.0);

    match (player_wins, opponent_wins) {
        (true, false) => RoundOutcome::PlayerWins(player_move),
        (false, true) => RoundOutcome::OpponentWins(player_move),
        _ => RoundOutcome::Draw(player_move),
    }
}

fn plan_move(round: &BaseRound<OutcomeDirection>) -> PlayerMove {
    PlayerMove(match round {
        BaseRound(OpponentMove(GameMove::Rock), OutcomeDirection::PlayerWins) => GameMove::Paper,
        BaseRound(OpponentMove(GameMove::Paper), OutcomeDirection::PlayerWins) => GameMove::Scissor,
        BaseRound(OpponentMove(GameMove::Scissor), OutcomeDirection::PlayerWins) => GameMove::Rock,
        BaseRound(OpponentMove(GameMove::Rock), OutcomeDirection::OpponentWins) => {
            GameMove::Scissor
        }
        BaseRound(OpponentMove(GameMove::Paper), OutcomeDirection::OpponentWins) => GameMove::Rock,
        BaseRound(OpponentMove(GameMove::Scissor), OutcomeDirection::OpponentWins) => {
            GameMove::Paper
        }
        BaseRound(OpponentMove(opponent_move), OutcomeDirection::Draw) => *opponent_move,
    })
}

fn evaluate_move(m: GameMove) -> u32 {
    match m {
        GameMove::Rock => 1,
        GameMove::Paper => 2,
        GameMove::Scissor => 3,
    }
}

fn evaluate_outcome(outcome: RoundOutcome) -> u32 {
    match outcome {
        RoundOutcome::PlayerWins(m) => evaluate_move(m.0) + 6,
        RoundOutcome::OpponentWins(m) => evaluate_move(m.0),
        RoundOutcome::Draw(m) => evaluate_move(m.0) + 3,
    }
}
fn solve<T>(input: &str) -> Result<u32, String>
where
    T: Solvable + FromStr<Err = RoundParsingError>,
{
    input
        .lines()
        .map(|s| s.parse::<T>())
        .fold_while(Ok(0), |acc, elem| match elem {
            Ok(round) => {
                let outcome = Solvable::solve(&round);
                let score = evaluate_outcome(outcome);

                FoldWhile::Continue(acc.map(|current_score| current_score + score))
            }
            Err(error) => FoldWhile::Done(Err(error)),
        })
        .into_inner()
        .map_err(|e| e.to_string())
}

pub fn part_one(input: &str) -> Result<u32, String> {
    solve::<MisinterpretedRound>(input)
}

pub fn part_two(input: &str) -> Result<u32, String> {
    solve::<Round>(input)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Ok(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Ok(12));
    }

    #[test]
    fn test_no_moves() {
        let input = "";

        assert_eq!(part_one(&input), Ok(0));
        assert_eq!(part_two(&input), Ok(0));
    }

    #[test]
    fn test_less_moves() {
        let input = "A X\nB ";

        assert_eq!(
            part_one(input),
            Err("Less moves than players have been supplied\n\tsupplied: B".to_string())
        );
        assert_eq!(
            part_two(input),
            Err("Less moves than players have been supplied\n\tsupplied: B".to_string())
        );
    }

    #[test]
    fn test_more_moves() {
        let input = "A X\nB X A ";

        assert_eq!(
            part_one(input),
            Err("More moves than players have been supplied\n\tsupplied: B,X,A".to_string())
        );
        assert_eq!(
            part_two(input),
            Err("More moves than players have been supplied\n\tsupplied: B,X,A".to_string())
        );
    }

    #[test]
    fn test_wrong_moves_opponent() {
        let input = "A X\nZ X";

        assert_eq!(part_one(input), Err("An invalid move or direction has been supplied for:\n\tOpponent:supplied game move [Z] for [opponent] expected one in [A, B, C] as [Rock, Paper, Scissor]".to_string()));
        assert_eq!(part_two(input), Err("An invalid move or direction has been supplied for:\n\tOpponent:supplied game move [Z] for [opponent] expected one in [A, B, C] as [Rock, Paper, Scissor]".to_string()));
    }
    #[test]
    fn test_wrong_moves_player() {
        let input = "A X\nB A";

        assert_eq!(part_one(input), Err("An invalid move or direction has been supplied for:\n\tPlayer:supplied game move [A] for [player] expected one in [X, Y, Z] as [Rock, Paper, Scissor]".to_string()));
        assert_eq!(part_two(input), Err("An invalid move or direction has been supplied for:\n\tPlayer:supplied direction [A] expected one in [X, Y, Z] as [OpponentWins, Draw, PlayerWins]".to_string()));
    }
}
