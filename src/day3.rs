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
        assert_eq!(
            trace_wire(vec![
                Pathlet::Right(2),
                Pathlet::Up(2),
                Pathlet::Left(2),
                Pathlet::Down(2),
            ]),
            vec![
                Line::new(Point::new(0, 0), Point::new(2, 0)),
                Line::new(Point::new(2, 0), Point::new(2, 2)),
                Line::new(Point::new(2, 2), Point::new(0, 2)),
                Line::new(Point::new(0, 2), Point::new(0, 0)),
            ]
        );
    }
    #[test]
    fn test_wire_intersections() {
        let red = trace_wire(string_to_path("R8,U5,L5,D3"));
        let green = trace_wire(string_to_path("U7,R6,D4,L4"));
        let intersections = wire_intersections(&red, &green);
        assert_eq!(
            wire_intersections(&red, &green),
            vec![Point::new(6, 5), Point::new(3, 3)]
        );
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
    pub fn origin() -> Point {
        Point { x: 0, y: 0 }
    }
    fn to_tuple(&self) -> (isize, isize) {
        (self.x, self.y)
    }
    pub fn distance(&self) -> usize {
        manhattan_distance(Point::origin(), *self)
    }
    fn x(&self) -> isize {
        self.x
    }
    fn y(&self) -> isize {
        self.y
    }
    fn x_plus_equals(&mut self, x: isize) -> Self {
        self.x += x;
        *self
    }
    fn y_plus_equals(&mut self, y: isize) -> Self {
        self.y += y;
        *self
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

/// The vertical or horizontal line between two points
#[derive(PartialEq)]
pub struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    pub fn new(p1: Point, p2: Point) -> Line {
        Line { p1, p2 }
    }
    fn p1(&self) -> Point {
        self.p1
    }
    fn p2(&self) -> Point {
        self.p2
    }
    fn x_min(&self) -> isize {
        std::cmp::min(self.p1.x(), self.p2.x())
    }
    fn x_max(&self) -> isize {
        std::cmp::max(self.p1.x(), self.p2.x())
    }
    fn y_min(&self) -> isize {
        std::cmp::min(self.p1.y(), self.p2.y())
    }
    fn y_max(&self) -> isize {
        std::cmp::max(self.p1.y(), self.p2.y())
    }
    pub fn find_intersection(&self, other: &Self) -> Option<Point> {
        // Test the reverse if the first didn't find a point, because
        // intersects only finds a if the first line is completely within the
        // bounding box of the second line.
        self.intersects(other).or(other.intersects(self))
    }
    fn intersects(&self, other: &Self) -> Option<Point> {
        if self.x_min() >= other.x_min()
            && other.x_max() >= self.x_min()
            && other.y_min() >= self.y_min()
            && self.y_max() >= other.y_min()
        {
            return Some(Point::new(self.x_min(), other.y_min()));
        }
        None
    }
    /// The length of the line
    pub fn length(&self) -> usize {
        // Since the line is either vertical or horizontal, Manhattan Distance is accurate
        manhattan_distance(self.p1, self.p2)
    }
    pub fn contains_point(&self, point: &Point) -> bool {
        // Case 1: Line is vertical so both x coords are equal. Contains point if point.x equals
        // line.x and line.min_y <= point.y <= line.max_y.
        //
        // Case 2: Line is horizontal, so both y coords are equal. Contains point according to the
        // above condition, swap x and y.
        (point.x() == self.p1.x && point.y() <= self.y_max() && point.y() >= self.y_min())
            || (point.y() == self.p1.y && point.x() <= self.x_max() && point.x() >= self.x_min())
    }
}

impl std::fmt::Debug for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Line {{({}, {}) -> ({}, {})}}",
            self.p1.x, self.p1.y, self.p2.x, self.p2.y
        )
    }
}

/// Calculate the manhattan distance between two points
pub fn manhattan_distance<T, U>(p: T, q: U) -> usize
where
    T: Into<(isize, isize)>,
    U: Into<(isize, isize)>,
{
    let p = p.into();
    let q = q.into();
    ((p.0 - q.0).abs() + (p.1 - q.1).abs()) as usize
}

/// Converts a vector of Pathlets to a BTreeSet of it's points, sorted by manhattan distance to
/// the origin
pub fn trace_wire(path: Vec<Pathlet>) -> Vec<Line> {
    let mut wire = vec![];
    let mut location = Point::origin();
    for pathlet in path {
        match pathlet {
            Pathlet::Right(distance) => wire.push(Line::new(
                location,
                location.x_plus_equals(distance as isize),
            )),
            Pathlet::Up(distance) => wire.push(Line::new(
                location,
                location.y_plus_equals(distance as isize),
            )),
            Pathlet::Left(distance) => wire.push(Line::new(
                location,
                location.x_plus_equals(-(distance as isize)),
            )),
            Pathlet::Down(distance) => wire.push(Line::new(
                location,
                location.y_plus_equals(-(distance as isize)),
            )),
        }
    }
    wire
}

pub fn wire_intersections(red: &Vec<Line>, green: &Vec<Line>) -> Vec<Point> {
    let mut intersections = vec![];
    for red_line in red {
        for green_line in green {
            red_line
                .find_intersection(&green_line)
                .and_then(|x| Some(intersections.push(x)));
        }
    }
    // The intersection at (0, 0) doesn't count
    intersections.split_off(1)
}

/// Distance from the origin to a point along a wire. Point must be in the wire.
pub fn wire_length_to(wire: &Vec<Line>, point: &Point) -> usize {
    let mut distance = 0;
    for line in wire {
        if line.contains_point(point) {
            // The line starts at p1, so the distance for this segment is the
            // distance between start of the line and the point.
            distance += manhattan_distance(line.p1(), *point);
            break;
        }
        distance += line.length();
    }
    distance
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
