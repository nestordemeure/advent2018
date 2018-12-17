#[macro_use]
extern crate scan_fmt;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

//-----------------------------------------------------------------------------
// POINT

struct Point
{
   x: i32,
   y: i32
}

/*
use std::ops::AddAssign;
impl AddAssign for &Point
{
   fn add_assign(&mut self, other: &Point)
   {
      self = Point { x: self.x + other.x, y: self.y + other.y };;
   }
}
*/

use std::ops::Add;
impl Add for &Point
{
   type Output = Point;

   fn add(self, other: &Point) -> Point
   {
      Point { x: self.x + other.x, y: self.y + other.y }
   }
}

//-----------------------------------------------------------------------------
// INPUT

/// parses a point and its speed
fn parse_particle(line: &str) -> (Point, Point)
{
   let (x, y, vx, vy) = scan_fmt!(&line, "position=<{}, {}> velocity=<{}, {}>", i32, i32, i32, i32);
   let point = Point { x: x.unwrap(), y: y.unwrap() };
   let speed = Point { x: vx.unwrap(), y: vy.unwrap() };
   (point, speed)
}

/// parses a file and returns (points, speeds)
fn input_data(path: &str) -> (Vec<Point>, Vec<Point>)
{
   let file = File::open(path).expect("Failed to open input file.");
   BufReader::new(file).lines()
                       .map(|line_option| line_option.expect("Failed to load a line."))
                       .map(|line| parse_particle(&line))
                       .unzip()
}

//-----------------------------------------------------------------------------
// TASK

/// move each point by its speed
fn move_points(points: &[Point], speeds: &[Point]) -> Vec<Point>
{
   points.iter().zip(speeds.iter()).map(|(p, s)| p + s /*Point { x: p.x + s.x, y: p.y + s.y }*/).collect()
}

/// returns the dimensions of the cloud
fn dimensions(points: &[Point]) -> (usize, usize)
{
   let (xmin, xmax) = points.iter().map(|p| p.x).minmax().into_option().unwrap();
   let (ymin, ymax) = points.iter().map(|p| p.y).minmax().into_option().unwrap();
   let width = (xmax - xmin) as usize;
   let height = (ymax - ymin) as usize;
   (width, height)
}

/// displays the current state on screen
fn display_points(points: &[Point])
{
   let (xmin, xmax) = points.iter().map(|p| p.x).minmax().into_option().unwrap();
   let (ymin, ymax) = points.iter().map(|p| p.y).minmax().into_option().unwrap();
   let width = (xmax - xmin) as usize;
   let height = (ymax - ymin) as usize;

   let mut screen = vec![vec![' '; width + 1]; height + 1];
   for Point { x, y } in points
   {
      let x = (x - xmin) as usize;
      let y = (y - ymin) as usize;
      screen[y][x] = '█';
   }

   screen.iter().map(|row| row.iter().collect::<String>()).for_each(|row| println!("{}", row))
}

/// finds the moment of alignement
fn find_alignement(mut points: Vec<Point>, speeds: &[Point])
{
   let (mut width, mut height) = dimensions(&points);
   let mut second = 0;

   loop
   {
      let new_points = move_points(&points, &speeds);
      let (new_width, new_height) = dimensions(&new_points);

      if new_width > width && new_height > height
      {
         // the cloud was at its most compact
         break;
      }
      else
      {
         // the cloud is getting more compact
         points = new_points;
         width = new_width;
         height = new_height;
         second += 1;
      }
   }

   println!("SECOND {}:", second);
   display_points(&points);
}

//-----------------------------------------------------------------------------
// MAIN

fn main()
{
   let input_path = "./data/input.txt";
   let (points, speeds) = input_data(input_path);
   find_alignement(points, &speeds);
}
