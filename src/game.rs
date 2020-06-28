extern crate rand;
extern crate rand_chacha;

use rand::prelude::*;
use rand::{Rng, SeedableRng};

pub struct GameIndex {
    r: usize,
    c: usize
}

impl GameIndex {
    fn new(row: usize, col: usize) -> GameIndex {
        GameIndex {
            r: row,
            c: col
        }
    }
}

pub struct Game {
    board: [u32; 16],
    board_height:usize,
    board_width:usize,
    board_size:usize,
    score:u64,
    seed:u64,
    rng: rand_chacha::ChaCha8Rng,
}

impl Game {
    pub fn start() -> Game {
        let mut rng = rand::thread_rng();
        return Game::start_with_seed(rng.next_u64());
    }
    pub fn start_with_seed(game_seed:u64) -> Game {
        let mut result = Game {
            board: [0; 16],
            board_height:4, 
            board_width:4,
            board_size:16,
            score:0,
            seed:game_seed,
            rng: rand_chacha::ChaCha8Rng::seed_from_u64(game_seed)
        };

        result.new_cell();
        return result;
    }

    pub fn board(&self) -> std::slice::Iter<u32> {
        return self.board.iter();
    }

    pub fn score(&self) -> u64 {
        return self.score;
    }

    pub fn seed(&self) -> u64 {
        return self.seed;
    }

    pub fn new_cell(&mut self) {
        let cell_index:usize = self.rng.gen_range(0, 16);
        if self.rng.gen() {
            self.board[cell_index] = 4;
        } else {
            self.board[cell_index] = 2;
        }
    }

    pub fn up(&mut self) {
        for idx in (0..self.board_size).rev() {
            let val = self.board[idx];
            if (idx > (self.board_width - 1)) && (val != 0) {
                let up_idx = idx - self.board_width;
                if self.board[up_idx] == 0 {
                    self.board.swap(up_idx, idx);
                } else if self.board[up_idx] == val {
                    self.board[up_idx] = val * 2;
                    self.board[idx] = 0;
                }
            }
        }

        self.new_cell();
    }

    pub fn down(&mut self) {
        for idx in 0..self.board_size {
            let val = self.board[idx];
            if self.board[idx] != 0 {
                let down_idx = idx + self.board_width;
                if (down_idx < self.board_size) && (self.board[down_idx] == 0) {
                    self.board.swap(down_idx, idx);
                } else if self.board[down_idx] == val {
                    self.board[down_idx] = val * 2;
                    self.board[idx] = 0;
                }
            }
        }

        self.new_cell();
    }

    pub fn left(&mut self) {
        for idx in (0..self.board_size).rev() {
            let val = self.board[idx];
            if (idx % self.board_width != 0) && (self.board[idx] != 0) {
                let mut rc = self.index_to_row_col(idx);
                rc.c -= 1;
                let left_idx = self.row_col_to_idx(&rc);
                if self.board[left_idx] == 0 {
                    self.board.swap(left_idx, idx);
                } else if self.board[left_idx] == val {
                    self.board[left_idx] = val * 2;
                    self.board[idx] = 0;
                }
            }
        }

        self.new_cell();
    }

    pub fn right(&mut self) {
        for idx in (0..self.board_size).rev() {
            let val = self.board[idx];
            let mut rc = self.index_to_row_col(idx);
            if (rc.c < (self.board_width - 1)) && (self.board[idx] != 0) {
                rc.c += 1;
                let right_idx = self.row_col_to_idx(&rc);
                if self.board[right_idx] == 0 {
                    self.board.swap(right_idx, idx);
                }  else if self.board[right_idx] == val {
                    self.board[right_idx] = val * 2;
                    self.board[idx] = 0;
                }
            }
        }

        self.new_cell();
    }

    pub fn dump_board(&self) {
        for i in 0..self.board_height {
            for j in 0..self.board_width {
                let cell = (i * self.board_height) + j;
                print!("{} ", self.board[cell]);
            }

            println!();
        }
    }

    fn index_to_row_col(&self, idx:usize) -> GameIndex {
        let row = idx / self.board_height;
        let col = idx - (self.board_height * row);

        return GameIndex::new(row, col);
    }

    fn row_col_to_idx(&self, rc:&GameIndex) -> usize {
        return (rc.r * self.board_height) + rc.c;
    }

    fn set_board(&mut self, board:[u32; 16]) {
        self.board = board;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SEED:u64 = 0xCAFEBABE;

    #[test]
    fn game_new_cell() {
        let g = Game::start_with_seed(SEED);
        let board = g.board().as_slice();
        g.dump_board();

        for i in 0..board.len() {
            let val = board[i];
            if i == 14 {
                assert_eq!(2, val);
            } else {
                assert_eq!(0, val);
            }
        }
    }

    #[test]
    fn game_up() {
        let mut g = Game::start_with_seed(SEED);
        g.up();
        let board = g.board().as_slice();
        g.dump_board();

        for i in 0..board.len() {
            let val = board[i];
            if i == 2 {
                assert_eq!(2, val);
            } else {
                assert_eq!(0, val);
            }
        }
    }

    #[test]
    fn game_down() {
        let mut g = Game::start_with_seed(SEED);
        g.up();
        g.down();
        let board = g.board().as_slice();
        g.dump_board();

        for i in 0..board.len() {
            let val = board[i];
            if i == 14 {
                assert_eq!(2, val);
            } else {
                assert_eq!(0, val);
            }
        }
    }

    #[test]
    fn game_left() {
        let mut g = Game::start_with_seed(SEED);
        g.left();
        let board = g.board().as_slice();
        g.dump_board();

        for i in 0..board.len() {
            let val = board[i];
            if i == 12 {
                assert_eq!(2, val);
            } else {
                assert_eq!(0, val);
            }
        }
    }

    #[test]
    fn game_right() {
        let mut g = Game::start_with_seed(SEED);
        g.right();
        let board = g.board().as_slice();
        g.dump_board();

        for i in 0..board.len() {
            let val = board[i];
            if i == 15 {
                assert_eq!(2, val);
            } else {
                assert_eq!(0, val);
            }
        }
    }

    #[test]
    fn game_upleft() {
        let mut g = Game::start_with_seed(SEED);
        g.left();
        g.up();
        let board = g.board().as_slice();
        g.dump_board();

        for i in 0..board.len() {
            let val = board[i];
            if i == 0 {
                assert_eq!(2, val);
            } else {
                assert_eq!(0, val);
            }
        }
    }

    #[test]
    fn game_upright() {
        let mut g = Game::start_with_seed(SEED);
        g.right();
        g.up();
        let board = g.board().as_slice();
        g.dump_board();

        for i in 0..board.len() {
            let val = board[i];
            if i == 3 {
                assert_eq!(2, val);
            } else {
                assert_eq!(0, val);
            }
        }
    }

    #[test]
    fn game_downleft() {
        let mut g = Game::start_with_seed(SEED);
        g.left();
        g.down();
        let board = g.board().as_slice();
        g.dump_board();

        for i in 0..board.len() {
            let val = board[i];
            if i == 12 {
                assert_eq!(2, val);
            } else {
                assert_eq!(0, val);
            }
        }
    }

    #[test]
    fn idx_conversions() {
        let g = Game::start_with_seed(SEED);

        println!("0 = (0, 0)");
        let rc = g.index_to_row_col(0);
        assert_eq!(0, rc.r);
        assert_eq!(0, rc.c);

        println!("15 = (3, 3)");
        let rc = g.index_to_row_col(15);
        assert_eq!(3, rc.r);
        assert_eq!(3, rc.c);

        println!("8 = (2, 0)");
        let rc = g.index_to_row_col(8);
        assert_eq!(2, rc.r);
        assert_eq!(0, rc.c);
    }

    #[test]
    fn rc_conversions() {
        let g = Game::start_with_seed(SEED);

        println!("0 = (0, 0)");
        let idx = g.row_col_to_idx(&GameIndex::new(0, 0));
        assert_eq!(0, idx);

        println!("15 = (3, 3)");
        let idx = g.row_col_to_idx(&GameIndex::new(3, 3));
        assert_eq!(15, idx);

        println!("8 = (2, 0)");
        let idx = g.row_col_to_idx(&GameIndex::new(2, 0));
        assert_eq!(8, idx);
    }
}