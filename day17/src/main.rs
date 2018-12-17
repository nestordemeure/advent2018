#[macro_use]
extern crate scan_fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

//-----------------------------------------------------------------------------
// TYPE

struct Interval
{
   xmin: usize,
   xmax: usize,
   ymin: usize,
   ymax: usize
}

#[derive(Clone, Copy, PartialEq)]
enum Material
{
   Source,
   FallingWater,
   FlowingWater, // water supported by a solid base and currently flowing to the side
   Water,
   Sand,
   Clay
}

type Map = [Vec<Material>];

//-----------------------------------------------------------------------------
// INPUT

/// reads a line and outputs an interval
fn parse_coordinates(line: &str) -> Interval
{
   let (c1, v1, _, v2min, v2max) = scan_fmt!(&line, "{}={}, {}={}..{}", char, usize, char, usize, usize);
   let v1 = v1.unwrap();
   let v2min = v2min.unwrap();
   let v2max = v2max.unwrap();
   if c1.unwrap() == 'x'
   {
      Interval { xmin: v1, xmax: v1, ymin: v2min, ymax: v2max }
   }
   else
   {
      Interval { xmin: v2min, xmax: v2max, ymin: v1, ymax: v1 }
   }
}

/// parses a file and returns a vector of intervals
fn input_data(path: &str) -> Vec<Interval>
{
   let file = File::open(path).expect("Failed to open input file.");
   BufReader::new(file).lines()
                       .map(|line_option| line_option.expect("Failed to load a line."))
                       .map(|line| parse_coordinates(&line))
                       .collect()
}

//-----------------------------------------------------------------------------
// SIDE FUNCTIONS

/// takes interval and the position of a source to build a map
fn fill_map(xsource: usize, ysource: usize, intervals: &[Interval]) -> (usize, usize, Vec<Vec<Material>>)
{
   let xmin = std::cmp::min(xsource, intervals.iter().map(|i| i.xmin).min().unwrap()) - 1;
   let xmax = intervals.iter().map(|i| i.xmax).max().unwrap() - xmin + 1;

   let ymin = intervals.iter().map(|i| i.ymin).min().unwrap();
   let ymax = intervals.iter().map(|i| i.ymax).max().unwrap() - ymin;
   let mut map = vec![vec![Material::Sand; xmax + 1]; ymax + 1];

   for interval in intervals
   {
      for y_unscaled in interval.ymin..=interval.ymax
      {
         let y = y_unscaled - ymin;
         for x_unscaled in interval.xmin..=interval.xmax
         {
            let x = x_unscaled - xmin;
            map[y][x] = Material::Clay;
         }
      }
   }

   (xsource - xmin, 0, map)
}

/// takes a material and outputs the corresponding char
fn char_of_material(m: Material) -> char
{
   match m
   {
      Material::Clay => '#',
      Material::Sand => '.',
      Material::Source => '+',
      Material::Water => '~',
      Material::FallingWater => '|',
      Material::FlowingWater => '-'
   }
}

/// displays the current map
fn display(map: &Map)
{
   println!("");
   for row in map
   {
      let line: String = row.iter().map(|&m| char_of_material(m)).collect();
      println!("{}", line);
   }
}

//-----------------------------------------------------------------------------
// task1

/// can we work on that square
fn is_free(x: usize, y: usize, map: &Map) -> bool
{
   if (y >= map.len()) || (x >= map[0].len())
   {
      false
   }
   else
   {
      let material = map[y][x];
      material == Material::Sand || material == Material::Source
   }
}

/// can we go throu that material
fn is_solid(x: usize, y: usize, map: &Map) -> bool
{
   if (y >= map.len()) || (x >= map[0].len())
   {
      false
   }
   else
   {
      let material = map[y][x];
      material == Material::Clay || material == Material::Water
   }
}

/// does the cell contains flowing water ?
fn is_flowing(x: usize, y: usize, map: &Map) -> bool
{
   map[y][x] == Material::FlowingWater
}

/// if we are in a container, fills the container with water
fn fill_container(x: usize, y: usize, map: &mut Map)
{
   /// was is the index of the last flowing cell before any solid cell
   fn container_max(x: usize, y: usize, map: &Map) -> Option<usize>
   {
      if is_flowing(x, y, &map)
      {
         if is_solid(x + 1, y, &map)
         {
            Some(x)
         }
         else
         {
            container_max(x + 1, y, &map)
         }
      }
      else
      {
         None
      }
   }
   let xmax = container_max(x, y, &map);

   /// was is the index of the first flowing cell, after a solid cell
   fn container_min(x: usize, y: usize, map: &Map) -> Option<usize>
   {
      if is_flowing(x, y, &map)
      {
         if is_solid(x - 1, y, &map)
         {
            Some(x)
         }
         else
         {
            container_min(x - 1, y, &map)
         }
      }
      else
      {
         None
      }
   }
   let xmin = container_min(x, y, &map);

   // if we are in a container, fills the container with water
   if let (Some(xmin), Some(xmax)) = (xmin, xmax)
   {
      for x in xmin..=xmax
      {
         map[y][x] = Material::Water;
      }
   }
}

/// simulates the flow of water from our position
fn simulate(x: usize, y: usize, mut map: &mut Map)
{
   // is the current square workable
   if is_free(x, y, &map)
   {
      // put falling water in our current position
      map[y][x] = Material::FallingWater;
      //display(&map);

      // waters flows under us if possible
      simulate(x, y + 1, &mut map);

      // are we on solid ground ?
      if is_solid(x, y + 1, &map)
      {
         map[y][x] = Material::FlowingWater;

         // water flows on the side
         simulate(x + 1, y, &mut map);
         simulate(x - 1, y, &mut map);

         // fill our current container (if is is a container)
         fill_container(x, y, &mut map);
      }
   }
}

//-----------------------------------------------------------------------------
// MAIN

/// counts the number of wet squares
fn evaluate(map: &Map) -> usize
{
   map.iter().flat_map(|row| row).filter(|&&mat| mat != Material::Sand && mat != Material::Clay).count()
}

/// counts the number of square with stable water
fn evaluate_stable_water(map: &Map) -> usize
{
   map.iter().flat_map(|row| row).filter(|&&mat| mat == Material::Water).count()
}

fn main()
{
   let (xsource, ysource) = (500, 0);
   let input_path = "./data/test.txt";
   let intervals = input_data(input_path);
   let (xsource, ysource, mut map) = fill_map(xsource, ysource, &intervals);

   // task1
   simulate(xsource, ysource, &mut map);
   map[ysource][xsource] = Material::Source;
   display(&map);
   let nb_water = evaluate(&map);
   println!("number of wet squares : {}", nb_water);

   // task2
   let nb_water_left = evaluate_stable_water(&map);
   println!("number of stable squares : {}", nb_water_left);
}
