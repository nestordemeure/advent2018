#[macro_use]
extern crate scan_fmt;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

//-----------------------------------------------------------------------------
// INPUT

struct Point
{
   x: i32,
   y: i32
}

/// parses a line to produce a point
fn parse_point(line: &str) -> Point
{
   let (x, y) = scan_fmt!(line, "{}, {}", i32, i32);
   Point { x: x.unwrap(), y: y.unwrap() }
}

/// reads a file line by line and converts each line to a point
fn read_inputs(path: &str) -> Vec<Point>
{
   let file = File::open(path).expect("Failed to open input file.");
   BufReader::new(file).lines()
                       .map(|line_option| line_option.expect("Failed to load a line."))
                       .map(|line| parse_point(&line))
                       .collect()
}

//-----------------------------------------------------------------------------
// TASK1

/// returns the manhattan distance between two points
fn manhattan_distance(p1: &Point, p2: &Point) -> i32
{
   (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

/// returns the index of the closest point in the absence of a tie
fn closest_point(coordinate: &Point, points: &[Point]) -> Option<usize>
{
   let mut is_tie = true;
   let mut best_point = 0;
   let mut best_dist = std::i32::MAX;

   for (i, p) in points.iter().enumerate()
   {
      match manhattan_distance(coordinate, p)
      {
         d if d == best_dist => is_tie = true,
         d if d < best_dist =>
         {
            is_tie = false;
            best_point = i;
            best_dist = d;
         }
         _ => ()
      }
   }

   if is_tie
   {
      None
   }
   else
   {
      Some(best_point)
   }
}

/// returns the area of the largest, non infinite, voronoy polygon
fn task1(points: &[Point]) -> usize
{
   let (xmin, xmax) = points.iter().map(|p| p.x).minmax().into_option().unwrap();
   let (ymin, ymax) = points.iter().map(|p| p.y).minmax().into_option().unwrap();

   // find area of all polygones whithin a rectangle that contains all points
   let mut areas = vec![0; points.len()];
   let mut finite = vec![true; points.len()];
   for x in xmin..=xmax
   {
      for y in ymin..=ymax
      {
         match closest_point(&Point { x, y }, points)
         {
            Some(i) if x == xmin || x == xmax || y == ymin || y == ymax => finite[i] = false,
            Some(i) => areas[i] += 1,
            None => ()
         }
      }
   }

   // gets largest, non infinite, area
   areas.iter().zip(finite).filter(|&(_, is_finite)| is_finite).map(|(&s, _)| s).max().unwrap()
}

//-----------------------------------------------------------------------------
// TASK2

/// returns the sum of distance to all points
fn total_distance(coordinate: &Point, points: &[Point]) -> i32
{
   points.iter().map(|p| manhattan_distance(coordinate, p)).sum()
}

/// returns the area of the section were is point is within max_dist of total distance to all points
fn task2(points: &[Point], max_dist: i32) -> usize
{
   let (xmin, xmax) = points.iter().map(|p| p.x).minmax().into_option().unwrap();
   let (ymin, ymax) = points.iter().map(|p| p.y).minmax().into_option().unwrap();

   (xmin..=xmax).cartesian_product(ymin..=ymax)
                .map(|(x, y)| Point { x, y })
                .filter(|p| total_distance(p, points) < max_dist)
                .count()
}

//-----------------------------------------------------------------------------
// MAIN

fn main()
{
   let input_path = "./data/input.txt";
   let points = read_inputs(input_path);

   // task1
   let max_area = task1(&points);
   println!("max area : {}", max_area);

   // task2
   let max_dist = 10_000;
   let area = task2(&points, max_dist);
   println!("safest area : {}", area);
}
