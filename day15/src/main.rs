use std::fs::File;
use std::io::{BufRead, BufReader};

//-----------------------------------------------------------------------------
// TYPE

#[derive(Clone, Copy, PartialEq)]
enum Kind
{
   Elf,
   Gobelin
}

#[derive(Clone, Copy, PartialEq)]
struct Unit
{
   hp: i32,
   attack: i32,
   kind: Kind,
   turn: usize
}

#[derive(Clone, Copy, PartialEq)]
enum Cell
{
   Wall,
   Empty,
   Unit(Unit)
}

type Map = [Vec<Cell>];

//-----------------------------------------------------------------------------
// INPUT

fn parse_char(c: char) -> Cell
{
   match c
   {
      '#' => Cell::Wall,
      '.' => Cell::Empty,
      'E' => Cell::Unit(Unit { hp: 200, attack: 3, kind: Kind::Elf, turn: 0 }),
      'G' => Cell::Unit(Unit { hp: 200, attack: 3, kind: Kind::Gobelin, turn: 0 }),
      _ => panic!("Unrecognized char!")
   }
}

fn parse_line(line: &str) -> Vec<Cell>
{
   line.chars().map(parse_char).collect()
}

fn input_data(path: &str) -> Vec<Vec<Cell>>
{
   let file = File::open(path).expect("Failed to open input file.");
   BufReader::new(file).lines()
                       .map(|line_option| line_option.expect("Failed to load a line."))
                       .map(|line| parse_line(&line))
                       .collect()
}

//-----------------------------------------------------------------------------
// CELL MANIPULATION

#[derive(PartialEq)]
enum Round
{
   NormalRound,
   NoMoreTarget
}

/// returns a list of all units in the map and their coordinates in the form (row, col, unit)
fn list_units(map: &Map) -> Vec<(usize, usize, Unit)>
{
   let mut result = Vec::new();

   for row in 0..map.len()
   {
      for col in 0..map[0].len()
      {
         if let Cell::Unit(unit) = map[row][col]
         {
            result.push((row, col, unit));
         }
      }
   }

   result
}

/// returns the sum of the hp of all units in the map
fn sum_hp(map: &Map) -> i32
{
   list_units(&map).iter().map(|(_, _, unit)| unit.hp).sum()
}

/// returns all empty neigbouring cells
fn neigbour_cells(row: usize, col: usize, map: &Map) -> Vec<(usize, usize)>
{
   let mut result = Vec::new();

   for &(row, col) in [(row - 1, col), (row, col - 1), (row, col + 1), (row + 1, col)].iter()
   {
      if let Cell::Empty = map[row][col]
      {
         result.push((row, col));
      }
   }

   result
}

/// displays the current board
fn display(map: &Map, turn: i32)
{
   println!("\nTURN {}", turn);
   for row in map
   {
      let mut suffix = "".to_string();
      let mut line = "".to_string();
      for cell in row
      {
         match cell
         {
            Cell::Empty => line += ".",
            Cell::Wall => line += "#",
            Cell::Unit(unit) if unit.kind == Kind::Elf =>
            {
               line += "E";
               suffix += &format!("E({}) ", unit.hp);
            }
            Cell::Unit(unit) =>
            {
               line += "G";
               suffix += &format!("G({}) ", unit.hp);
            }
         }
      }
      println!("{}   {}", line, suffix);
   }
}

/// makes a deep copy of the map
fn clone_map(map: &Map) -> Vec<Vec<Cell>>
{
   map.iter().map(|row| row.iter().map(|&cell| cell).collect::<Vec<Cell>>()).collect()
}

//-----------------------------------------------------------------------------
// MOVE

/// finds a step that gets us to the given position by the shortest path
/// breaks ties according to reading order
/// returns (distance, row, col)
fn find_step(from_row: usize,
             from_col: usize,
             to_row: usize,
             to_col: usize,
             map: &Map)
             -> Option<(usize, usize, usize)>
{
   let max_dist = std::usize::MAX;

   // builds a distance-from-target map
   let mut dist_map = vec![vec![max_dist; map[0].len()]; map.len()];
   let mut active_square = vec![(to_row, to_col)];
   let mut current_dist = 0;
   while !active_square.is_empty()
   {
      let mut next_active_squares = Vec::new();
      for (row, col) in active_square
      {
         if (dist_map[row][col] > current_dist) && (map[row][col] == Cell::Empty)
         {
            dist_map[row][col] = current_dist;
            next_active_squares.extend(neigbour_cells(row, col, &map));
         }
      }
      active_square = next_active_squares;
      current_dist += 1;
   }

   // returns the neibours cell that gets us closest to our target (break ties by reading order)
   neigbour_cells(from_row, from_col, &map).iter()
                                           .map(|&(row, col)| (dist_map[row][col], row, col))
                                           .filter(|&(dist, _, _)| dist != max_dist)
                                           .min()
}

/// finds a step that gets us to one of the ennemies
/// break ties according to reading order
fn move_toward_ennemy(row: usize,
                      col: usize,
                      ennemies: Vec<(usize, usize)>,
                      map: &Map)
                      -> Option<(usize, usize)>
{
   ennemies.into_iter()
           .flat_map(|(row, col)| neigbour_cells(row, col, &map))
           .filter_map(|(to_row, to_col)| {
              find_step(row, col, to_row, to_col, &map).map(|(dist, fromr, fromc)| {
                                                          (dist, to_row, to_col, fromr, fromc)
                                                       })
           })
           .min()
           .map(|(_, _, _, row, col)| (row, col))
}

//-----------------------------------------------------------------------------
// ATTACK

/// returns the first neigbouring cell that is an ennemy
fn find_ennemy(my_kind: Kind, row: usize, col: usize, map: &Map) -> Option<(usize, usize)>
{
   /// is the content of the cell an ennemy of my kind ?
   fn is_ennemy(my_kind: Kind, cell: &Cell) -> Option<&Unit>
   {
      match cell
      {
         Cell::Unit(unit) if unit.kind != my_kind => Some(unit),
         _ => None
      }
   }

   [(row - 1, col), (row, col - 1), (row, col + 1), (row + 1, col)].iter()
      .filter_map(|&(row,col)| is_ennemy(my_kind, &map[row][col]).map(|unit| (unit.hp, row, col)) )
      .min()
      .map(|(_,row,col)| (row, col))
}

//-----------------------------------------------------------------------------
// SIMULATION

/// plays a turn for the unit at the given position
fn turn(unit: Unit, mut row: usize, mut col: usize, map: &mut Map, elf_attack: i32) -> Round
{
   // if there is no ennemy we finish
   let ennemies: Vec<_> = list_units(&map).into_iter()
                                          .filter(|(_, _, ennemy)| ennemy.kind != unit.kind)
                                          .map(|(row, col, _)| (row, col))
                                          .collect();
   if ennemies.is_empty()
   {
      return Round::NoMoreTarget;
   }

   // if we are not next to an ennemy, we try to get closer
   let mut ennemy = find_ennemy(unit.kind, row, col, &map);
   if ennemy == None
   {
      if let Some((new_row, new_col)) = move_toward_ennemy(row, col, ennemies, &map)
      {
         map[row][col] = Cell::Empty;
         map[new_row][new_col] = Cell::Unit(Unit { turn: unit.turn + 1, ..unit });
         row = new_row;
         col = new_col;
         ennemy = find_ennemy(unit.kind, row, col, &map);
      }
   }

   // if we are, now, next to an ennemy, we attack
   if let Some((row_en, col_en)) = ennemy
   {
      if let Cell::Unit(ref mut ennemy) = map[row_en][col_en]
      {
         if unit.kind == Kind::Elf
         {
            ennemy.hp -= elf_attack;
         }
         else
         {
            ennemy.hp -= unit.attack;
         }

         if ennemy.hp <= 0
         {
            map[row_en][col_en] = Cell::Empty;
         }
      }
   }

   Round::NormalRound
}

/// plays a turn for all units
/// returns NoMoreTarget if a unit found no target
fn round(mut map: &mut Map, elf_attack: i32) -> Round
{
   for (row, col, previous_unit) in list_units(&map)
   {
      // if there still is a unit there despite the previous turns
      if let Cell::Unit(unit) = map[row][col]
      {
         if previous_unit.turn == unit.turn
         {
            if turn(unit, row, col, &mut map, elf_attack) == Round::NoMoreTarget
            {
               return Round::NoMoreTarget;
            }
         }
      }
   }

   Round::NormalRound
}

/// plays the game until a side win and returns the number of turn played
fn simulate(mut map: &mut Map, elf_attack: i32) -> i32
{
   let mut turn = 0;

   loop
   {
      //display(&map, turn);
      if round(&mut map, elf_attack) == Round::NoMoreTarget
      {
         return turn;
      }
      else
      {
         turn += 1
      }
   }
}

//-----------------------------------------------------------------------------
// TASK2

/// counts the number of elfs in the map
fn count_elfs(map: &Map) -> usize
{
   let mut result = 0;

   for row in map
   {
      for cell in row
      {
         if let Cell::Unit(unit) = cell
         {
            if unit.kind == Kind::Elf
            {
               result += 1;
            }
         }
      }
   }

   result
}

/// test wether an attack manages to keep the number of elfs constant
fn test_attack(map: &Map, attack: i32, elf_count: usize) -> bool
{
   let mut map = clone_map(&map);
   simulate(&mut map, attack);
   let new_elf_count = count_elfs(&map);
   let result = new_elf_count == elf_count;
   println!("attack:{} nb_before:{} nb_after:{} => {}", attack, elf_count, new_elf_count, result);
   result
}

/// finds the minimal attack needed to make sure that all elfs survive
/// NOTE: dichotomy might not work since stonger elfs mights adopt another strategy that get some of them killed
fn find_optimal_attack(map: &Map, mut base_attack: i32) -> i32
{
   let elf_number = count_elfs(&map);

   loop
   {
      if test_attack(&map, base_attack, elf_number)
      {
         return base_attack;
      }
      else
      {
         base_attack += 1;
      }
   }
}

//-----------------------------------------------------------------------------
// MAIN

fn main()
{
   let input_path = "./data/input.txt";
   let mut map = input_data(input_path);
   let elf_attack = 3;

   // task1
   {
      let mut map = clone_map(&map);
      let turn_number = simulate(&mut map, elf_attack);
      let total_hp = sum_hp(&map);
      let score = turn_number * total_hp;
      //display(&map, turn_number);
      println!("score final : {} (total_hp:{} turn_number:{})", score, total_hp, turn_number);
   }

   // task2
   {
      let mut map = clone_map(&map);
      let optimal_attack = find_optimal_attack(&map, elf_attack);
      println!("optimal attack is {}", optimal_attack);

      let turn_number = simulate(&mut map, optimal_attack);
      let total_hp = sum_hp(&map);
      let score = turn_number * total_hp;
      println!("score optimal : {} (total_hp:{} turn_number:{})", score, total_hp, turn_number);
   }
}
