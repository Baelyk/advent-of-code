// Day 3 2019

use std::cmp::Ordering;
use std::collections::BTreeSet;

#[cfg(test)]
mod tests {
    use crate::day3::*;
    #[test]
    fn test_string_to_path() {
        let path = vec![
            Pathlet::Right(8),
            Pathlet::Up(5),
            Pathlet::Left(5),
            Pathlet::Down(3),
        ];
        assert_eq!(string_to_path("R8,U5,L5,D3"), path);
    }
    #[test]
    fn test_trace_wire() {
        let mut wire: BTreeSet<Point> = BTreeSet::new();
        wire.insert(Point::new(0, 0));
        wire.insert(Point::new(1, 0));
        wire.insert(Point::new(2, 0));
        wire.insert(Point::new(2, 1));
        wire.insert(Point::new(2, 2));
        wire.insert(Point::new(1, 2));
        wire.insert(Point::new(0, 2));
        wire.insert(Point::new(0, 1));
        wire.insert(Point::new(0, 0)); // Won't be inserted, but the Pathlet::Down arm will try,
                                       // so we will to
        wire.insert(Point::new(1, 0));
        wire.insert(Point::new(2, 0));
        wire.insert(Point::new(3, 0));
        wire.insert(Point::new(4, 0));
        assert_eq!(
            trace_wire(vec![
                Pathlet::Right(2),
                Pathlet::Up(2),
                Pathlet::Left(2),
                Pathlet::Down(2),
                Pathlet::Right(4),
            ]),
            wire
        );
    }
    #[test]
    fn test_intersection() {
        let red = trace_wire(string_to_path("R8,U5,L5,D3,R30"));
        println!("{:?}", red);
        println!("{:?}", red.iter().clone().collect::<Vec<&Point>>());
        let green = trace_wire(string_to_path("U7,R6,D4,L4"));
        let intersection = red.intersection(&green);
        println!("{:?}", intersection.clone().collect::<Vec<&Point>>());
        assert_eq!(
            manhattan_distance(Point::Origin(), *red.intersection(&green).next().unwrap()),
            6
        );
        let red = trace_wire(string_to_path("R75,D30,R83,U83,L12,D49,R71,U7,L72"));
        let green = trace_wire(string_to_path("U62,R66,U55,R34,D71,R55,D58,R83"));
        let intersection = red.intersection(&green);
        println!("{:?}", intersection.clone().collect::<Vec<&Point>>());
        assert_eq!(
            manhattan_distance(Point::Origin(), *red.intersection(&green).next().unwrap()),
            159
        )
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Pathlet {
    Right(usize),
    Up(usize),
    Left(usize),
    Down(usize),
}

#[derive(Clone, Copy, Debug, Eq)]
pub struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }
    pub fn Origin() -> Point {
        Point { x: 0, y: 0 }
    }
    fn to_tuple(&self) -> (isize, isize) {
        (self.x, self.y)
    }
    fn distance(&self) -> isize {
        manhattan_distance(Point::Origin(), *self)
    }
    fn x(&self) -> isize {
        self.x
    }
    fn y(&self) -> isize {
        self.y
    }
    /// Increments x by 1
    fn xpp(&mut self) {
        self.x += 1;
    }
    /// Increments y by 1
    fn ypp(&mut self) {
        self.y += 1;
    }
    /// Decrements x by 1
    fn xmm(&mut self) {
        self.x -= 1;
    }
    /// Decrements y by 1
    fn ymm(&mut self) {
        self.y -= 1;
    }
}

impl Into<(isize, isize)> for Point {
    fn into(self) -> (isize, isize) {
        self.to_tuple()
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        // If the distance between the points is the same, but the points are different,
        // self is less than other.
        if self.distance() == other.distance() && self != other {
            if self.x != other.x {
                return self.x.cmp(&other.x);
            } else {
                return self.y.cmp(&other.y);
            }
        }
        self.distance().cmp(&other.distance())
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

/// Calculate the manhattan distance between two points
pub fn manhattan_distance<T, U>(p: T, q: U) -> isize
where
    T: Into<(isize, isize)>,
    U: Into<(isize, isize)>,
{
    let p = p.into();
    let q = q.into();
    let horizontal = p.0 - q.0;
    let vertical = p.1 - q.1;
    horizontal.abs() + vertical.abs()
}

/// Converts a vector of Pathlets to a BTreeSet of it's points, sorted by manhattan distance to
/// the origin
pub fn trace_wire(path: Vec<Pathlet>) -> BTreeSet<Point> {
    let mut wire = BTreeSet::new();
    let mut location = Point::Origin();
    for pathlet in path {
        match pathlet {
            Pathlet::Right(distance) => {
                for _ in 0..distance {
                    location.xpp();
                    wire.insert(location);
                }
            }
            Pathlet::Up(distance) => {
                for _ in 0..distance {
                    location.ypp();
                    wire.insert(location);
                }
            }
            Pathlet::Left(distance) => {
                for _ in 0..distance {
                    location.xmm();
                    wire.insert(location);
                }
            }
            Pathlet::Down(distance) => {
                for _ in 0..distance {
                    location.ymm();
                    wire.insert(location);
                }
            }
        }
    }
    wire
}

/// Converts a comma seperated list of Pathlet strs to a vector of Pathlets
pub fn string_to_path(string: &str) -> Vec<Pathlet> {
    use std::str::FromStr;
    string
        .split(",")
        .map(|x| {
            let (direction, distance) = x.split_at(1);
            let distance = usize::from_str(distance).unwrap();
            match direction {
                "R" => Pathlet::Right(distance),
                "U" => Pathlet::Up(distance),
                "L" => Pathlet::Left(distance),
                "D" => Pathlet::Down(distance),
                &_ => panic!("Unexpected pathlet direction {}", direction),
            }
        })
        .collect()
}
