use rand::Rng;
use std::fs;

pub const GRID_SIZE: usize = 4;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
pub struct TileMove {
    pub from: (usize, usize),
    pub to: (usize, usize),
    pub value: u32,
    #[allow(dead_code)]
    pub is_merge: bool,
}

pub struct TurnResult {
    pub moves: Vec<TileMove>,
    #[allow(dead_code)]
    pub score_increase: u32,
    pub moved: bool,
    pub merged: bool,
    pub new_tile: Option<(usize, usize, u32)>,
}

pub struct Game {
    pub grid: [[u32; GRID_SIZE]; GRID_SIZE],
    pub score: u32,
    pub high_score: u32,
    pub game_over: bool,
    pub won: bool,
}

impl Game {
    pub fn new() -> Self {
        let high_score = Game::load_high_score();
        let mut game = Game {
            grid: [[0; GRID_SIZE]; GRID_SIZE],
            score: 0,
            high_score,
            game_over: false,
            won: false,
        };
        game.spawn_tile();
        game.spawn_tile();
        game
    }

    fn load_high_score() -> u32 {
        fs::read_to_string("highscore.txt")
            .ok()
            .and_then(|s| s.trim().parse().ok())
            .unwrap_or(0)
    }

    fn save_high_score(&self) {
        if let Err(e) = fs::write("highscore.txt", self.high_score.to_string()) {
            eprintln!("Failed to save high score: {}", e);
        }
    }

    pub fn spawn_tile(&mut self) -> Option<(usize, usize, u32)> {
        let mut empty_cells = Vec::new();
        for r in 0..GRID_SIZE {
            for c in 0..GRID_SIZE {
                if self.grid[r][c] == 0 {
                    empty_cells.push((r, c));
                }
            }
        }

        if let Some(&(r, c)) = empty_cells.get(rand::rng().random_range(0..empty_cells.len())) {
            let val = if rand::rng().random_bool(0.9) { 2 } else { 4 };
            self.grid[r][c] = val;
            Some((r, c, val))
        } else {
            None
        }
    }

    pub fn move_tiles(&mut self, dir: Direction) -> TurnResult {
        if self.game_over {
            return TurnResult { moves: vec![], score_increase: 0, moved: false, merged: false, new_tile: None };
        }

        let mut moves = Vec::new();
        let mut moved = false;
        let mut merged_flag = false;
        let mut score_inc = 0;
        let mut new_grid = [[0; GRID_SIZE]; GRID_SIZE];
        let mut merged = [[false; GRID_SIZE]; GRID_SIZE];

        match dir {
            Direction::Left => {
                for r in 0..GRID_SIZE {
                    let mut c_idx = 0;
                    for c in 0..GRID_SIZE {
                        if self.grid[r][c] != 0 {
                            if c_idx > 0 && new_grid[r][c_idx - 1] == self.grid[r][c] && !merged[r][c_idx - 1] {
                                let val = new_grid[r][c_idx - 1] * 2;
                                new_grid[r][c_idx - 1] = val;
                                score_inc += val;
                                merged[r][c_idx - 1] = true;
                                moved = true;
                                merged_flag = true;
                                moves.push(TileMove { from: (r, c), to: (r, c_idx - 1), value: self.grid[r][c], is_merge: true });
                            } else {
                                new_grid[r][c_idx] = self.grid[r][c];
                                if c != c_idx {
                                    moved = true;
                                    moves.push(TileMove { from: (r, c), to: (r, c_idx), value: self.grid[r][c], is_merge: false });
                                } else {
                                     // No move, but still need to represent it if we want full state animations, 
                                     // but here we only care about moving tiles.
                                }
                                c_idx += 1;
                            }
                        }
                    }
                }
            }
            Direction::Right => {
                for r in 0..GRID_SIZE {
                    let mut c_idx = GRID_SIZE - 1;
                    for c in (0..GRID_SIZE).rev() {
                        if self.grid[r][c] != 0 {
                            if c_idx < GRID_SIZE - 1 && new_grid[r][c_idx + 1] == self.grid[r][c] && !merged[r][c_idx + 1] {
                                let val = new_grid[r][c_idx + 1] * 2;
                                new_grid[r][c_idx + 1] = val;
                                score_inc += val;
                                merged[r][c_idx + 1] = true;
                                moved = true;
                                merged_flag = true;
                                moves.push(TileMove { from: (r, c), to: (r, c_idx + 1), value: self.grid[r][c], is_merge: true });
                            } else {
                                new_grid[r][c_idx] = self.grid[r][c];
                                if c != c_idx {
                                    moved = true;
                                    moves.push(TileMove { from: (r, c), to: (r, c_idx), value: self.grid[r][c], is_merge: false });
                                }
                                if c_idx > 0 { c_idx -= 1; }
                            }
                        }
                    }
                }
            }
            Direction::Up => {
                for c in 0..GRID_SIZE {
                    let mut r_idx = 0;
                    for r in 0..GRID_SIZE {
                        if self.grid[r][c] != 0 {
                            if r_idx > 0 && new_grid[r_idx - 1][c] == self.grid[r][c] && !merged[r_idx - 1][c] {
                                let val = new_grid[r_idx - 1][c] * 2;
                                new_grid[r_idx - 1][c] = val;
                                score_inc += val;
                                merged[r_idx - 1][c] = true;
                                moved = true;
                                merged_flag = true;
                                moves.push(TileMove { from: (r, c), to: (r_idx - 1, c), value: self.grid[r][c], is_merge: true });
                            } else {
                                new_grid[r_idx][c] = self.grid[r][c];
                                if r != r_idx {
                                    moved = true;
                                    moves.push(TileMove { from: (r, c), to: (r_idx, c), value: self.grid[r][c], is_merge: false });
                                }
                                r_idx += 1;
                            }
                        }
                    }
                }
            }
            Direction::Down => {
                for c in 0..GRID_SIZE {
                    let mut r_idx = GRID_SIZE - 1;
                    for r in (0..GRID_SIZE).rev() {
                        if self.grid[r][c] != 0 {
                            if r_idx < GRID_SIZE - 1 && new_grid[r_idx + 1][c] == self.grid[r][c] && !merged[r_idx + 1][c] {
                                let val = new_grid[r_idx + 1][c] * 2;
                                new_grid[r_idx + 1][c] = val;
                                score_inc += val;
                                merged[r_idx + 1][c] = true;
                                moved = true;
                                merged_flag = true;
                                moves.push(TileMove { from: (r, c), to: (r_idx + 1, c), value: self.grid[r][c], is_merge: true });
                            } else {
                                new_grid[r_idx][c] = self.grid[r][c];
                                if r != r_idx {
                                    moved = true;
                                    moves.push(TileMove { from: (r, c), to: (r_idx, c), value: self.grid[r][c], is_merge: false });
                                }
                                if r_idx > 0 { r_idx -= 1; }
                            }
                        }
                    }
                }
            }
        }

        let mut new_tile = None;
        if moved {
            self.score += score_inc;
            if self.score > self.high_score {
                self.high_score = self.score;
                self.save_high_score();
            }
            self.grid = new_grid;
            new_tile = self.spawn_tile();
            self.check_game_over();
        }

        TurnResult {
            moves,
            score_increase: score_inc,
            moved,
            merged: merged_flag,
            new_tile,
        }
    }

    fn check_game_over(&mut self) {
        // Check for 2048
        for r in 0..GRID_SIZE {
            for c in 0..GRID_SIZE {
                if self.grid[r][c] == 2048 {
                    self.won = true;
                }
            }
        }

        // Check if full
        for r in 0..GRID_SIZE {
            for c in 0..GRID_SIZE {
                if self.grid[r][c] == 0 {
                    return;
                }
            }
        }

        // Check if merges possible
        for r in 0..GRID_SIZE {
            for c in 0..GRID_SIZE {
                let val = self.grid[r][c];
                if r + 1 < GRID_SIZE && self.grid[r+1][c] == val { return; }
                if c + 1 < GRID_SIZE && self.grid[r][c+1] == val { return; }
            }
        }

        self.game_over = true;
    }
    
    pub fn reset(&mut self) {
        *self = Game::new();
    }
}