use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Game {
    boards: Vec<(Board, bool)>,
}

impl Game {
    fn mark_number(&mut self, number: i32) {
        for board in self.boards.iter_mut() {
            board.0.mark_number(number);
        }
    }

    fn has_bingo(&self) -> bool {
        self.boards.iter().any(|board| board.1 == false && board.0.has_bingo())
    }

    fn bingo_board(&self) -> &(Board, bool) {
        self.boards.iter().filter(|board| board.1 == false && board.0.has_bingo()).last().unwrap()
    }

    pub fn play(&mut self, input_sequence: Vec<i32>) -> i32 {
        for input_num in input_sequence {
            println!("Drawing {}", input_num);
            self.mark_number(input_num);
            if self.has_bingo() {
                let result = self.bingo_board().0.sum_unmarked() * input_num;
                println!("{}", result);
                return result;
            }
        }

        return 0;
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
                    .map(|bingo_val| (bingo_val, false)).collect();
                rows.push(row);
            } else {
                boards.push((Board { rows: rows.to_vec() }, false));
                rows.clear();
            }
        }

        return Self { boards };
    }
}

#[derive(PartialEq)]
struct Board {
    rows: Vec<Vec<(i32, bool)>>,
}

impl Board {
    fn mark_number(&mut self, number: i32) {
        for row in self.rows.iter_mut() {
            for val in row.iter_mut() {
                if val.0 == number {
                    val.1 = true;
                }
            }
        }
    }

    fn has_bingo(&self) -> bool {
        self.has_row_bingo() || self.has_column_bingo()
    }

    fn has_row_bingo(&self) -> bool {
        self.rows.iter().any(|row| row.iter().all(|val| val.1 == true))
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
        self.rows.iter().flatten().filter(|val| val.1 == false).map(|val| val.0).sum()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        let input_sequence = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3, 26, 1];
        let mut game = Game::new("src/part4_test_input.txt");
        assert_eq!(game.play(input_sequence), 4512);
    }
}