#[macro_use]
extern crate scan_fmt;
use std::fs::File;
use std::i32;
use std::io::{BufRead, BufReader};
extern crate priority_queue;
use priority_queue::PriorityQueue;

//-----------------------------------------------------------------------------
// TYPE

type Coordinate = i32;

type Position = (Coordinate, Coordinate, Coordinate);

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

/// cube whose dimenssions are a power of two
/// the max in each dimension is always excluded
#[derive(Clone, Hash, Eq, PartialEq)]
struct Cube
{
   xmin: i32,
   xmax: i32,
   ymin: i32,
   ymax: i32,
   zmin: i32,
   zmax: i32,
   bots_in_range: Vec<usize> // vector of indexes of bots such that at least a cell of the cube is in range
}

/// computes the shortest manhatan distance between a cube and a position
fn manhatan_distance_cube(cube: &Cube, (x, y, z): Position) -> i32
{
   /// distance between a segment and a value
   fn dist(kmin: i32, kmax: i32, k: i32) -> i32
   {
      if k < kmin
      {
         kmin - k
      }
      else if k >= kmax
      {
         k - kmax + 1 // because kmax is excluded
      }
      else
      {
         0 // k is in [kmin;kmax]
      }
   }
   let dx = dist(cube.xmin, cube.xmax, x);
   let dy = dist(cube.ymin, cube.ymax, y);
   let dz = dist(cube.zmin, cube.zmax, z);
   dx + dy + dz
}

/// is at least a cell of the cube in range of the bot
fn is_cube_in_range(cube: &Cube, bot: &Nanobot) -> bool
{
   manhatan_distance_cube(&cube, bot.position) <= bot.radius
}

/// returns true if the cube collapsed into a cell
fn is_cell(cube: &Cube) -> bool
{
   (cube.xmin == cube.xmax - 1) && (cube.ymin == cube.ymax - 1) && (cube.zmin == cube.zmax - 1)
}

/// builds a cube whose dimensions are power of two and that contains all the bots
fn make_cube(bots: &[Nanobot]) -> Cube
{
   // finds dimenssions of the box
   let width = bots.iter()
                   .flat_map(|b| vec![b.position.0, b.position.1, b.position.2])
                   .map(|k| k.abs())
                   .max()
                   .unwrap();
   let width = (width as usize).next_power_of_two() as i32;
   let kmin = -width;
   let kmax = width;
   // builds the cube
   let bots_in_range = (0..bots.len()).collect();
   Cube { xmin: kmin, xmax: kmax, ymin: kmin, ymax: kmax, zmin: kmin, zmax: kmax, bots_in_range }
}

/// takes a cube and splits it in 8 sub cubes
/// each cube gets the bots that are in its range
fn subdivide_cube(cube: &Cube, bots: &[Nanobot]) -> Vec<Cube>
{
   let xmid = (cube.xmax + cube.xmin) / 2;
   let ymid = (cube.ymax + cube.ymin) / 2;
   let zmid = (cube.zmax + cube.zmin) / 2;
   let mut c1 = Cube { xmin: cube.xmin,
                       xmax: xmid,
                       ymin: cube.ymin,
                       ymax: ymid,
                       zmin: cube.zmin,
                       zmax: zmid,
                       bots_in_range: Vec::new() };
   c1.bots_in_range =
      cube.bots_in_range.iter().filter(|&&i| is_cube_in_range(&c1, &bots[i])).cloned().collect();
   let mut c2 = Cube { xmin: cube.xmin,
                       xmax: xmid,
                       ymin: cube.ymin,
                       ymax: ymid,
                       zmin: zmid,
                       zmax: cube.zmax,
                       bots_in_range: Vec::new() };
   c2.bots_in_range =
      cube.bots_in_range.iter().filter(|&&i| is_cube_in_range(&c2, &bots[i])).cloned().collect();
   let mut c3 = Cube { xmin: cube.xmin,
                       xmax: xmid,
                       ymin: ymid,
                       ymax: cube.ymax,
                       zmin: cube.zmin,
                       zmax: zmid,
                       bots_in_range: Vec::new() };
   c3.bots_in_range =
      cube.bots_in_range.iter().filter(|&&i| is_cube_in_range(&c3, &bots[i])).cloned().collect();
   let mut c4 = Cube { xmin: cube.xmin,
                       xmax: xmid,
                       ymin: ymid,
                       ymax: cube.ymax,
                       zmin: zmid,
                       zmax: cube.zmax,
                       bots_in_range: Vec::new() };
   c4.bots_in_range =
      cube.bots_in_range.iter().filter(|&&i| is_cube_in_range(&c4, &bots[i])).cloned().collect();
   let mut c5 = Cube { xmin: xmid,
                       xmax: cube.xmax,
                       ymin: cube.ymin,
                       ymax: ymid,
                       zmin: cube.zmin,
                       zmax: zmid,
                       bots_in_range: Vec::new() };
   c5.bots_in_range =
      cube.bots_in_range.iter().filter(|&&i| is_cube_in_range(&c5, &bots[i])).cloned().collect();
   let mut c6 = Cube { xmin: xmid,
                       xmax: cube.xmax,
                       ymin: cube.ymin,
                       ymax: ymid,
                       zmin: zmid,
                       zmax: cube.zmax,
                       bots_in_range: Vec::new() };
   c6.bots_in_range =
      cube.bots_in_range.iter().filter(|&&i| is_cube_in_range(&c6, &bots[i])).cloned().collect();
   let mut c7 = Cube { xmin: xmid,
                       xmax: cube.xmax,
                       ymin: ymid,
                       ymax: cube.ymax,
                       zmin: cube.zmin,
                       zmax: zmid,
                       bots_in_range: Vec::new() };
   c7.bots_in_range =
      cube.bots_in_range.iter().filter(|&&i| is_cube_in_range(&c7, &bots[i])).cloned().collect();
   let mut c8 = Cube { xmin: xmid,
                       xmax: cube.xmax,
                       ymin: ymid,
                       ymax: cube.ymax,
                       zmin: zmid,
                       zmax: cube.zmax,
                       bots_in_range: Vec::new() };
   c8.bots_in_range =
      cube.bots_in_range.iter().filter(|&&i| is_cube_in_range(&c8, &bots[i])).cloned().collect();

   vec![c1, c2, c3, c4, c5, c6, c7, c8]
}

/// returns the distance to the origin of the cell that :
/// - is in range of the most bots
/// - is closer to the origin (among cells equaly in range)
fn task2(bots: &[Nanobot]) -> i32
{
   let mut queue = PriorityQueue::new();
   queue.push(make_cube(&bots), bots.len());

   let mut best_weight = 0;
   let mut best_dist = 0;
   loop
   {
      match queue.pop()
      {
         None => return best_dist,                                      // no more cubes
         Some((_, weight)) if weight < best_weight => return best_dist, // no cube can be better than our current cell
         Some((ref cube, weight)) if is_cell(cube) =>
         {
            // this cube is a cell, is it better than our current cell ?
            let dist = cube.xmin.abs() + cube.ymin.abs() + cube.zmin.abs();
            if (weight > best_weight) || ((weight == best_weight) && (dist < best_dist))
            {
               best_weight = weight;
               best_dist = dist;
            }
         }
         Some((cube, _)) =>
         {
            // subdivide the cube and keep searching
            for cube in subdivide_cube(&cube, &bots)
            {
               let weight = cube.bots_in_range.len();
               if weight > 0
               {
                  queue.push(cube, weight);
               }
            }
         }
      }
   }
}

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
