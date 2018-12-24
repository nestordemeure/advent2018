#[macro_use]
extern crate scan_fmt;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

//-----------------------------------------------------------------------------
// TYPE

type Coordinate = i32;

type Position = (Coordinate, Coordinate, Coordinate);
const ORIGIN: Position = (0, 0, 0);

#[derive(Copy, Clone)]
struct Nanobot
{
   position: Position,
   radius: Coordinate
}

//-----------------------------------------------------------------------------
// INPUT

fn parse_nanobot(line: &str) -> Nanobot
{
   let (x, y, z, radius) = scan_fmt!(line, "pos=<{},{},{}>, r={}", i32, i32, i32, i32);
   let position = (x.unwrap(), y.unwrap(), z.unwrap());
   let radius = radius.unwrap();
   Nanobot { position, radius }
}

/// parses a file and returns a vector of nanobots
fn input_data(path: &str) -> Vec<Nanobot>
{
   let file = File::open(path).expect("Failed to open input file.");
   BufReader::new(file).lines()
                       .map(|line_option| line_option.expect("Failed to load a line."))
                       .map(|line| parse_nanobot(&line))
                       .collect()
}

//-----------------------------------------------------------------------------
// TASK1

fn manhatan_distance((x1, y1, z1): Position, (x2, y2, z2): Position) -> i32
{
   (x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs()
}

fn is_in_range(bot: &Nanobot, position_target: Position) -> bool
{
   manhatan_distance(bot.position, position_target) <= bot.radius
}

fn task1(nanobots: &[Nanobot]) -> usize
{
   let stronger_bot = nanobots.iter().max_by_key(|bot| bot.radius).unwrap();
   nanobots.iter().filter(|bot| is_in_range(stronger_bot, bot.position)).count()
}

//-----------------------------------------------------------------------------
// TASK2

/*
cut space into rectangles
take the rectngle in range of the most bots and subdivide it
when a rectangle is too small to be subdivided, computes its best cell
stop when n o rectangle can contain better than the best known cell
*/

//-----------------------------------------------------------------------------
// MAIN

fn main()
{
   let input_path = "./data/input.txt";
   let mut nanobots = input_data(input_path);

   // task1
   let bots_in_range = task1(&nanobots);
   println!("bots in range of the stronger bot : {}", bots_in_range);

   // task2
   nanobots.sort_unstable_by_key(|bot| bot.radius);
   let min_dist = task2(&nanobots);
   println!("closest bot to origin : {}", min_dist);
}
