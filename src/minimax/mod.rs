use board::{Board, Square};

use std::cmp;
use std::cmp::Ordering;
use std::i32;
use std::cmp::PartialEq;

#[derive(PartialEq, Eq, PartialOrd)]
pub struct Decision
{
    score: i32,
    pub pos: Option<(usize, usize)>
}

impl Ord for Decision
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        let lhs = self.score;
        let rhs = other.score;
        lhs.cmp(&rhs)
    }
}

pub fn minimax(board: &Board,
               depth: usize,
               alpha: i32,
               beta: i32,
               maximizing_player: bool,
               prev_play: Option<(usize, usize)>,
               player: &Square,
               mut killer_moves: Vec<Vec<(usize, usize)>>)
    -> Decision
{
    let current_color = match maximizing_player { true => player.clone(), false => player.opposite() };
    if depth == 0 || board.check_full_board()
        || (prev_play.is_some() &&
            board.check_aligned(prev_play.unwrap(), &current_color))
        || board.b_capture >= 10
        || board.w_capture >= 10 {
        let score = board.evaluation(&player);
        return Decision {
            score: score,
            pos: prev_play
        };
    }
    let mut plays: Vec<(usize, usize)> = Vec::new();
    if !killer_moves[depth].is_empty() { plays.append(&mut killer_moves[depth]); }
    plays.append(&mut board.get_plays(&current_color));
    if maximizing_player {
        let mut v = Decision { score: alpha, pos: None };
        for pos in plays {
            let child = board.play_at(Some(pos), &current_color);
            if child.is_some() {
                let score = v.score;
                v = cmp::max(v, minimax(&child.unwrap(), depth - 1, score, beta, false, Some(pos), player, killer_moves.clone()));
                if beta <= v.score {
                    killer_moves[depth].push(v.pos.unwrap());
                    println!("beta cutoff\n{:?}", board.play_at(v.pos, &current_color));
                    break ; // beta cut-off
                }
            }
        }
        return Decision {
            score: v.score,
            pos: if prev_play.is_none() { v.pos } else { prev_play },
        };
    }
    else {
        let mut v = Decision { score: beta, pos: None };
        for pos in plays {
            let child = board.play_at(Some(pos), &current_color);
            if child.is_some() {
                let score = v.score;
                v = cmp::min(v, minimax(&child.unwrap(), depth - 1, alpha, score, true, Some(pos), player, killer_moves.clone()));
                if v.score <= alpha {
                    killer_moves[depth].push(v.pos.unwrap());
                    println!("alpha cutoff\n{:?}", board.play_at(v.pos, &current_color));
                    break ; // alpha cut-off
                }
            }
        }
        return Decision {
            score: v.score,
            pos: if prev_play.is_none() { v.pos } else { prev_play },
        };
    }
}
