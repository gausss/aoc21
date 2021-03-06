use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(coordinate_string: &str) -> Self {
        let coordinates: Vec<i32> = coordinate_string
            .split(",")
            .map(|coordinate| coordinate.parse().unwrap())
            .collect();

        Self {
            x: coordinates.get(0).unwrap().to_owned(),
            y: coordinates.get(1).unwrap().to_owned(),
        }
    }
}
#[derive(Debug)]
pub struct Vector {
    direction: (Point, Point),
}

impl Vector {
    pub fn span_points(&self) -> Vec<(i32, i32)> {
        let mut positions = Vec::new();
        if self.direction.0.y == self.direction.1.y {
            for x in min(self.direction.0.x, self.direction.1.x)
                ..max(self.direction.0.x, self.direction.1.x) + 1
            {
                positions.push((x, self.direction.0.y));
            }
        } else if self.direction.0.x == self.direction.1.x {
            for y in min(self.direction.0.y, self.direction.1.y)
                ..max(self.direction.0.y, self.direction.1.y) + 1
            {
                positions.push((self.direction.0.x, y));
            }
        }

        return positions;
    }

    pub fn from_points_file(file_path: &str) -> Vec<Vector> {
        let file = File::open(file_path).unwrap();
        let lines = BufReader::new(file).lines();

        let mut vectors: Vec<Vector> = Vec::new();
        for line in lines {
            let cleaned_line = line.unwrap();

            let direction: Vec<Point> = cleaned_line
                .split(" -> ")
                .into_iter()
                .map(|coordinates| Point::new(coordinates))
                .collect();

            let start = direction.get(0).unwrap();
            let end = direction.get(1).unwrap();
            vectors.push(Vector {
                direction: (
                    Point {
                        x: start.x,
                        y: start.y,
                    },
                    Point { x: end.x, y: end.y },
                ),
            });
        }

        return vectors;
    }
}

pub fn compute_overlapping_lines(file_path: &str) -> i32 {
    let mut coordinates_values: HashMap<(i32, i32), i32> = HashMap::new();

    let file_input = Vector::from_points_file(file_path);
    let lines: Vec<(i32, i32)> = file_input
        .iter()
        .flat_map(|line| line.span_points())
        .collect();

    for point in lines {
        if coordinates_values.contains_key(&point) {
            coordinates_values.insert(point, coordinates_values.get(&point).unwrap() + 1);
        } else {
            coordinates_values.insert(point, 1);
        }
    }

    return coordinates_values.iter().filter(|val| val.1 > &1).count() as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(compute_overlapping_lines("src/input/part5_test_input.txt"), 5);
    }

    #[test]
    fn solve() {
        // Part 5
        println!(
            "Result 5.A: {}",
            compute_overlapping_lines("src/input/part5_input.txt")
        );
    }
}
