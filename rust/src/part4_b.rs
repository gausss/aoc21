use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Game {
    boards: Vec<Board>,
}

impl Game {
    fn mark_number(&mut self, number: i32) -> i32 {
        let mut winning_index: i32 = -1;
        for board in self.boards.iter_mut().enumerate() {
            if board.1.mark_number(number) {
                winning_index = board.0 as i32;
            }
        }

        return winning_index;
    }

    pub fn play(&mut self, input_sequence: Vec<i32>) -> i32 {
        let mut last_result = 0;
        for input_num in input_sequence {
            let winning_index = self.mark_number(input_num);
            if winning_index != -1 {
                last_result = self
                    .boards
                    .get(winning_index as usize)
                    .unwrap()
                    .sum_unmarked()
                    * input_num;
            }
        }

        return last_result;
    }

    pub fn new(file_path: &str) -> Self {
        let file = File::open(file_path).unwrap();
        let lines = BufReader::new(file).lines();
        let mut boards = Vec::new();

        let mut rows: Vec<Vec<(i32, bool)>> = Vec::new();
        for line in lines {
            let cleaned_line = line.unwrap();
            if !cleaned_line.is_empty() {
                let row: Vec<(i32, bool)> = cleaned_line
                    .trim()
                    .split_whitespace()
                    .map(|bingo_val| bingo_val.parse().unwrap())
                    .map(|bingo_val| (bingo_val, false))
                    .collect();
                rows.push(row);
            } else {
                boards.push(Board {
                    had_bingo: false,
                    rows: rows.to_vec(),
                });
                rows.clear();
            }
        }

        return Self { boards };
    }
}

struct Board {
    had_bingo: bool,
    rows: Vec<Vec<(i32, bool)>>,
}

impl Board {
    fn mark_number(&mut self, number: i32) -> bool {
        for row in self.rows.iter_mut() {
            for val in row.iter_mut() {
                if val.0 == number {
                    val.1 = true;
                }
            }
        }

        let has_bingo = self.has_bingo();
        if has_bingo {
            self.had_bingo = true;
        }

        return has_bingo;
    }

    fn has_bingo(&self) -> bool {
        !self.had_bingo && (self.has_row_bingo() || self.has_column_bingo())
    }

    fn has_row_bingo(&self) -> bool {
        self.rows
            .iter()
            .any(|row| row.iter().all(|val| val.1 == true))
    }

    fn has_column_bingo(&self) -> bool {
        for position in 0..4 {
            let mut bingo = true;
            for row in &self.rows {
                bingo = row.get(position).unwrap().1 && bingo;
            }

            if bingo == true {
                return true;
            }
        }

        return false;
    }

    fn sum_unmarked(&self) -> i32 {
        self.rows
            .iter()
            .flatten()
            .filter(|val| val.1 == false)
            .map(|val| val.0)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input_sequence = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        let mut game = Game::new("src/input/part4_test_input.txt");
        assert_eq!(game.play(input_sequence), 1924);
    }
    #[test]
    fn solve() {
        let mut game = Game::new("src/input/part4_input.txt");
        let input = vec![
            0, 56, 39, 4, 52, 7, 73, 57, 65, 13, 3, 72, 69, 96, 18, 9, 49, 83, 24, 31, 12, 64, 29,
            21, 80, 71, 66, 95, 2, 62, 68, 46, 11, 33, 74, 88, 17, 15, 5, 6, 98, 30, 51, 78, 76,
            75, 28, 53, 87, 48, 20, 22, 55, 86, 82, 90, 47, 19, 25, 1, 27, 60, 94, 38, 97, 58, 70,
            10, 43, 40, 89, 26, 34, 32, 23, 45, 50, 91, 61, 44, 35, 85, 63, 16, 99, 92, 8, 36, 81,
            84, 79, 37, 93, 67, 59, 54, 41, 77, 42, 14,
        ];
        println!("Result 4.B: {}", game.play(input));
    }
}
