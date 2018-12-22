#[macro_use]
extern crate scan_fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

//-----------------------------------------------------------------------------
// TYPE

#[derive(PartialEq, Clone, Copy)]
enum Region
{
   Rocky,
   Narrow,
   Wet
}

type Coordinate = (usize, usize);

type Map = [Vec<Region>];

#[derive(PartialEq, Clone, Copy)]
enum Gear
{
   Torch,
   ClimbingGear,
   Neither
}

//-----------------------------------------------------------------------------
// INPUT

/// parses a file and returns (depth, (targeti,targetj))
fn input_data(path: &str) -> (usize, Coordinate)
{
   let file = File::open(path).expect("Failed to open input file.");
   let mut lines =
      BufReader::new(file).lines().map(|line_option| line_option.expect("Failed to load a line."));

   let depth = scan_fmt!(&lines.next().unwrap(), "depth: {}", usize);
   let (x_target, y_target) = scan_fmt!(&lines.next().unwrap(), "target: {},{}", usize, usize);

   (depth.unwrap(), (y_target.unwrap(), x_target.unwrap()))
}

//-----------------------------------------------------------------------------
// TASK1

/// takes an erosion level and returns the associated region
fn region_of_erosion(erosion_level: &usize) -> Region
{
   match erosion_level % 3
   {
      0 => Region::Rocky,
      1 => Region::Wet,
      2 => Region::Narrow,
      _ => panic!("No other possible modulo!")
   }
}

/// formula that turn a geologic index into an erosion level
fn erosion_of_geologic_id(geologic_id: usize, depth: usize) -> usize
{
   (geologic_id + depth) % 20183
}

/// computes the geologic id of a coordinate
fn geologic_id_of_coordinate(i: usize, j: usize, target: Coordinate, erosion_map: &[Vec<usize>]) -> usize
{
   match (i, j)
   {
      (0, 0) => 0,
      coord if coord == target => 0,
      (0, x) => x * 16807,
      (y, 0) => y * 48271,
      (y, x) => erosion_map[y - 1][x] * erosion_map[y][x - 1]
   }
}

/// takes the description of a map and computes its composition
/// NOTE : the size of the map is not a fucntion of depth
fn make_map(depth: usize, target: Coordinate, buffer: usize) -> Vec<Vec<Region>>
{
   let height = target.0 + 1 + buffer;
   let width = target.1 + 1 + buffer;
   let mut erosion_map = vec![vec![0; width]; height];

   for i in 0..height
   {
      for j in 0..width
      {
         let geologic_id = geologic_id_of_coordinate(i, j, target, &erosion_map);
         let erosion = erosion_of_geologic_id(geologic_id, depth);
         erosion_map[i][j] = erosion;
      }
   }

   // builds the final map with the erosion levels
   erosion_map.into_iter().map(|row| row.iter().map(region_of_erosion).collect()).collect()
}

//-----------------------------------------------------------------------------
// TASK2

struct Distances
{
   torch: Vec<Vec<usize>>,
   climbing: Vec<Vec<usize>>,
   neither: Vec<Vec<usize>>
}

type Coordinate3D = (usize, usize, Gear);

fn make_distances(height: usize, width: usize) -> Distances
{
   let max_dist = std::usize::MAX;
   let distances_torch = vec![vec![max_dist; width]; height];
   let distances_climbing = vec![vec![max_dist; width]; height];
   let distances_neither = vec![vec![max_dist; width]; height];
   Distances { torch: distances_torch, climbing: distances_climbing, neither: distances_neither }
}

fn get_distance(distances: &Distances, (i, j, gear): Coordinate3D) -> usize
{
   match gear
   {
      Gear::Torch => distances.torch[i][j],
      Gear::Neither => distances.neither[i][j],
      Gear::ClimbingGear => distances.climbing[i][j]
   }
}

fn set_distance(distances: &mut Distances, (i, j, gear): Coordinate3D, value: usize)
{
   match gear
   {
      Gear::Torch => distances.torch[i][j] = value,
      Gear::Neither => distances.neither[i][j] = value,
      Gear::ClimbingGear => distances.climbing[i][j] = value
   }
}

/// returns true if a region can be traversed with the current equipement
fn can_be_traversed((i, j, gear): Coordinate3D, map: &Map) -> bool
{
   match map[i][j]
   {
      Region::Wet if gear != Gear::Torch => true,
      Region::Rocky if gear != Gear::Neither => true,
      Region::Narrow if gear != Gear::ClimbingGear => true,
      _ => false
   }
}

//-----------------------------------------------

/// returns a vector of (3D-coordinate,time)
fn add_neigbours((i, j, gear): Coordinate3D,
                 time: usize,
                 height: usize,
                 width: usize,
                 neigbours: &mut Vec<(Coordinate3D, usize)>)
{
   // add neigbours with switched gear
   let gear_time = time + 7;
   match gear
   {
      Gear::Torch =>
      {
         neigbours.push(((i, j, Gear::ClimbingGear), gear_time));
         neigbours.push(((i, j, Gear::Neither), gear_time))
      }
      Gear::Neither =>
      {
         neigbours.push(((i, j, Gear::ClimbingGear), gear_time));
         neigbours.push(((i, j, Gear::Torch), gear_time))
      }
      Gear::ClimbingGear =>
      {
         neigbours.push(((i, j, Gear::Torch), gear_time));
         neigbours.push(((i, j, Gear::Neither), gear_time))
      }
   };

   let move_time = time + 1;
   if i != 0
   {
      neigbours.push(((i - 1, j, gear), move_time));
   }
   if i + 1 < height
   {
      neigbours.push(((i + 1, j, gear), move_time));
   }
   if j != 0
   {
      neigbours.push(((i, j - 1, gear), move_time));
   }
   if j + 1 < width
   {
      neigbours.push(((i, j + 1, gear), move_time));
   }
}

//-----------------------------------------------

/// computes the shortest time needed to reach the target
fn shortest_distance((itarget, jtarget): Coordinate, map: &Map) -> usize
{
   let height = map.len();
   let width = map[0].len();
   let mut distances = make_distances(height, width);

   let initial_coordinate = (0, 0, Gear::Torch);
   let mut current_position = vec![(initial_coordinate, 0)];
   while !current_position.is_empty()
   {
      let mut new_positions = Vec::new();
      for (coordinate, time) in current_position
      {
         if can_be_traversed(coordinate, &map) && get_distance(&distances, coordinate) > time
         {
            set_distance(&mut distances, coordinate, time);
            add_neigbours(coordinate, time, height, width, &mut new_positions);
         }
      }
      current_position = new_positions;
   }

   let final_coordinate = (itarget, jtarget, Gear::Torch);
   get_distance(&distances, final_coordinate)
}

//-----------------------------------------------------------------------------
// MAIN

fn display_map(map: &Map)
{
   fn char_of_region(region: &Region) -> char
   {
      match region
      {
         Region::Narrow => '|',
         Region::Rocky => '.',
         Region::Wet => '='
      }
   }

   for row in map
   {
      let line: String = row.iter().map(char_of_region).collect();
      println!("{}", line);
   }
}

fn risk_level(map: &Map, (targeti, targetj): Coordinate) -> usize
{
   let mut level = 0;

   for i in 0..=targeti
   {
      for j in 0..=targetj
      {
         match map[i][j]
         {
            Region::Rocky => level += 0,
            Region::Wet => level += 1,
            Region::Narrow => level += 2
         }
      }
   }

   level
}

fn main()
{
   let input_path = "./data/input.txt";
   let (depth, target) = input_data(input_path);
   let buffer = 100;

   // task1
   let map = make_map(depth, target, buffer);
   //display_map(&map);
   let risk = risk_level(&map, target);
   println!("the risk level is {}", risk);

   // task2
   let distance = shortest_distance(target, &map);
   println!("The shortest distance to the target is {}", distance);
}
