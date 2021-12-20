#[derive(Debug)]
struct ValuePos (usize, usize, usize);

#[derive(Debug)]
pub struct Bingo {
    index: [Vec<ValuePos>; 100],
    boards: Vec<BoardMap>
}

pub const BOARD_SIZE: usize = 5;

impl Bingo {
    pub fn new() -> Self {
        Self {
            index: [(); 100].map(|_| Vec::new()),
            boards: Vec::new()
        }
    }
    
    pub fn add_board(&mut self, nums: BoardNumbers) {
        let mut board: BoardMap = Default::default();
        let board_idx = self.boards.len();
        for y in 0..BOARD_SIZE {
            let mut sum: isize = 0;
            for x in 0..BOARD_SIZE {
                let v = nums[y][x];
                sum += v as isize;
                self.index[v].push(ValuePos(board_idx, x, y))
            }
            board.totals[y] = sum;
        }
        self.boards.push(board);        
    }

    pub fn draw(&mut self, value: usize, last_winner_required: bool) -> Option<isize> {
        assert!(value < 100);
        let mut found_winner = false;
        let mut last_sum: isize = 0;
        for v in self.index[value].iter() {
            let board = &mut self.boards[v.0];
            if matches!(board.set_number(v.1, v.2, value), MarkNumberResult::BoardWinner) {
                found_winner = true;
                last_sum = board.get_unmarked_sum();
                if !last_winner_required {
                    break; 
                }
            } 
        }
        if found_winner {
            Some(last_sum * value as isize)
        } else {
            None
        }
    }
}


#[derive(Debug, Default)]
struct BoardMap {
    rows: [i8; BOARD_SIZE],
    columns: [i8; BOARD_SIZE],
    totals: [isize; BOARD_SIZE],
    completed: bool
}

enum MarkNumberResult {
    NoWinner,
    AlreadyCompleted,
    BoardWinner
}

impl BoardMap {
    fn set_number(&mut self, x:usize, y:usize, value: usize) -> MarkNumberResult {
        assert!(x < BOARD_SIZE && y < BOARD_SIZE);
        if self.completed {
            return MarkNumberResult::AlreadyCompleted
        }

        let bitx = 1 << (BOARD_SIZE - x - 1);
        let m = self.rows[y] | bitx;
        self.rows[y] = m;

        let bity = 1 << (BOARD_SIZE - y - 1);
        let n = self.columns[x] | bity;
        self.columns[x] = n;
        
        self.totals[y] -= value as isize;
        assert!(self.totals[y] >= 0);

        if m == 0x1f || n == 0x1f {
            self.completed = true;
            MarkNumberResult::BoardWinner
        } else {
            MarkNumberResult::NoWinner
        }
    }

    fn get_unmarked_sum(&self) -> isize {
        let total = self.totals.into_iter().fold(0, |acc, x| acc + x);
        total
    }
}


pub type BoardNumbers = Box<[[usize; BOARD_SIZE]; BOARD_SIZE]>;