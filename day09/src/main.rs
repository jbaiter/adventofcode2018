use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;
use std::io::{self, BufRead};
use std::mem;
use std::str::FromStr;

type Result<T> = std::result::Result<T, Box<Error>>;

// Stolen from burntsushi's AOC day 3 solution
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<Error>::from(format!($($tt)*))) }
}


struct CircularList<T: Debug> {
    nodes: Vec<Node<T>>,
    cursor: Option<usize>,
}


#[derive(Debug)]
struct Node<T: Debug> {
    pub data: Option<T>,
    pub left: usize,
    pub right: usize,
}

impl<T: Debug> CircularList<T> {
    fn with_capacity(capacity: usize) -> CircularList<T> {
        CircularList {
            nodes: Vec::with_capacity(capacity),
            cursor: None,
        }
    }

    fn insert(&mut self, value: T) {
        let new_idx = self.nodes.len();
        let (left, right) = match self.cursor {
            Some(cur_idx) => {
                let right_idx = self.nodes[cur_idx].right;
                self.nodes[right_idx].left = new_idx;
                self.nodes[cur_idx].right = new_idx;
                (cur_idx, right_idx)
            },
            None      => (new_idx, new_idx)
        };
        self.nodes.push(Node { data: Some(value), left: left, right: right });
        self.cursor = Some(new_idx);
    }

    fn remove(&mut self) -> Option<T> {
        let cur_idx = self.cursor?;
        // We don't remove from the list of nodes to prevent expensive memory
        // operations, instead we use a node without a value
        let mut cleared = Node { data: None, left: 0, right: 0 };
        mem::swap(&mut cleared, &mut self.nodes[cur_idx]);

        if cleared.right == cur_idx {
            self.cursor = None;
        } else {
            self.nodes[cleared.left].right = cleared.right;
            self.nodes[cleared.right].left = cleared.left;
            self.cursor = Some(cleared.right);
        }
        cleared.data
    }

    fn rotate(&mut self, steps: isize) {
        (0..steps.abs()).for_each(|_| {
            if steps < 0 {
                self.rotate_left();
            } else {
                self.rotate_right();
            }
        });
    }

    fn rotate_right(&mut self) {
        self.cursor = match self.cursor {
            Some(idx) => Some(self.nodes[idx].left),
            None      => None
        }
    }

    fn rotate_left(&mut self) {
        self.cursor = match self.cursor {
            Some(idx) => Some(self.nodes[idx].right),
            None      => None
        }
    }
}


struct MarbleGame {
    marbles: CircularList<usize>,
    player_scores: HashMap<usize, usize>,
    players: usize,
    num_marbles: usize,
    next_marble: usize,
}

impl FromStr for MarbleGame {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<MarbleGame> {
        let parts: Vec<String> = s.split_whitespace()
            .map(|s| s.to_string()).collect();
        let num_players: usize = (parts[0]).parse()?;
        let highest_marble: usize = (parts[6]).parse()?;
        Ok(MarbleGame::new(num_players, highest_marble))
    }
}

impl MarbleGame {
    fn new(players: usize, num_marbles: usize) -> MarbleGame {
        let mut marbles = CircularList::with_capacity(num_marbles);
        marbles.insert(0);
        MarbleGame {
            players: players,
            num_marbles: num_marbles,
            marbles: marbles,
            player_scores: HashMap::new(),
            next_marble: 1,
        }
    }

    fn play_round(&mut self) -> Option<usize> {
        let player = self.next_marble % self.players;
        if self.next_marble % 23 == 0 {
            let player_score = self.player_scores.entry(player).or_insert(0);
            *player_score += self.next_marble;
            self.marbles.rotate(7);
            *player_score += self.marbles.remove().unwrap();
        } else {
            self.marbles.rotate(-1);
            self.marbles.insert(self.next_marble);
        }
        if self.next_marble == self.num_marbles {
            None
        } else {
            self.next_marble += 1;
            Some(self.next_marble)
        }
    }

    fn play_game(&mut self) -> (usize, usize) {
        loop {
            match self.play_round() {
                Some(_) => (),
                None    => break
            };
            let prct_done: f32 = self.next_marble as f32 / self.num_marbles as f32;
            print!("{:.2}%\r", prct_done * 100.0);
        }
        print!("\n");
        self.player_scores.iter()
            .max_by_key(|(_, &score)| score)
            .map(|(x, y)| (x.clone(), y.clone()))
            .unwrap()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! marble_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                let mut game: MarbleGame = input.parse().unwrap();
                assert_eq!(expected, game.play_game());
            }
        )*
        }
    }

    marble_tests! {
        game_00: ("9 players; last marble is worth 25 points", (5, 32)),
        game_01: ("10 players; last marble is worth 1618 points", (0, 8317)),
        game_02: ("13 players; last marble is worth 7999 points",  (12, 146373)),
        game_03: ("17 players; last marble is worth 1104 points", (16, 2764)),
        game_04: ("21 players; last marble is worth 6111 points", (5, 54718)),
        game_05: ("30 players; last marble is worth 5807 points", (20, 37305)),
    }
}


fn main() -> Result<()> {
    let stdin = io::stdin();
    let mut line_iter = stdin.lock().lines();
    let mut game: MarbleGame = match line_iter.next() {
        Some(l) => Ok(l?.parse()?),
        None    => err!("No tree specification passed.")
    }?;
    let (winning_player, highscore) = game.play_game();
    println!("{} players; last marble is worth {} points; high score is {}; winner is {}",
             game.players, game.num_marbles, highscore, winning_player);
    game = MarbleGame::new(game.players, game.num_marbles * 100);
    let (winning_player, highscore) = game.play_game();
    println!("{} players; last marble is worth {} points; high score is {}; winner is {}",
             game.players, game.num_marbles, highscore, winning_player);
    Ok(())
}
