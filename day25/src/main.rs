#[macro_use]
extern crate scan_fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

//-----------------------------------------------------------------------------
// TYPE

type Point = (i32, i32, i32, i32);

type Constelation = Vec<Point>;

//-----------------------------------------------------------------------------
// INPUT

/// returns a point
fn parse_point(line: &str) -> Point
{
   let (x, y, z, t) = scan_fmt!(line, "{},{},{},{}", i32, i32, i32, i32);
   (x.unwrap(), y.unwrap(), z.unwrap(), t.unwrap())
}

/// turns a file into a vector of points
fn input_data(path: &str) -> Vec<Point>
{
   let file = File::open(path).expect("Failed to open input file.");
   BufReader::new(file).lines()
                       .map(|line_option| line_option.expect("Failed to load a line."))
                       .map(|line| parse_point(&line))
                       .collect()
}

//-----------------------------------------------------------------------------
// TASK1

/// computes the manhatan distance between two points
fn manhatant_distance(p1: &Point, p2: &Point) -> i32
{
   (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs() + (p1.2 - p2.2).abs() + (p1.3 - p2.3).abs()
}

/// is the point in the constelation
fn is_in_constelation(p: &Point, c: &[Point]) -> bool
{
   c.iter().any(|p2| manhatant_distance(p, p2) < 4)
}

/// put each point into a constelation
fn make_constelations(points: &[Point]) -> Vec<Constelation>
{
   let mut constelations: Vec<Constelation> = Vec::new();

   for &point in points
   {
      let (points_constelations, new_constelations): (Vec<Constelation>, Vec<Constelation>) =
         constelations.into_iter().partition(|c| is_in_constelation(&point, &c));
      // builds the constelation with the point
      let mut constelation: Constelation = points_constelations.into_iter().flat_map(|c| c).collect();
      constelation.push(point);
      // builds the group of constelations
      constelations = new_constelations;
      constelations.push(constelation);
   }

   constelations
}

//-----------------------------------------------------------------------------
// MAIN

fn main()
{
   let input_path = "./data/input.txt";
   let points = input_data(input_path);

   // task1
   let constelations = make_constelations(&points);
   let nb_constelations = constelations.len();
   println!("nb constelations : {}", nb_constelations);
}
