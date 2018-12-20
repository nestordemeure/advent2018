#![feature(slice_patterns)]
use std::collections::HashSet;

//-----------------------------------------------------------------------------
// TYPES

enum Direction
{
   North,
   East,
   West,
   South,
   Crossing(Vec<Route>)
}

type Route = Vec<Direction>;

type Position = (i32, i32);

#[derive(Clone, Copy, PartialEq)]
enum Base
{
   Wall,
   Door,
   Room,
   InitialPosition
}

type Map = [Vec<Base>];

//-----------------------------------------------------------------------------
// ROUTE PARSING

/// reads a string and returns a couple (route, non-consummed chars)
fn make_route(mut regexp: &[char]) -> (Route, &[char])
{
   let mut route = Vec::new();

   while !regexp.is_empty()
   {
      match regexp[0]
      {
         '^' | '$' => (),
         '|' | ')' => return (route, regexp),
         'N' => route.push(Direction::North),
         'S' => route.push(Direction::South),
         'E' => route.push(Direction::East),
         'W' => route.push(Direction::West),
         '(' =>
         {
            let mut crossing = Vec::new();
            while regexp[0] != ')'
            {
               let (subroute, regexp_letf) = make_route(&regexp[1..]);
               crossing.push(subroute);
               regexp = regexp_letf;
            }
            route.push(Direction::Crossing(crossing))
         }
         _ => panic!("Unknown direction!")
      }
      regexp = &regexp[1..];
   }

   (route, regexp)
}

/// applies a direction to a position and update the doors
fn apply_direction((i, j): Position, direction: &Direction, doors: &mut HashSet<Position>) -> Position
{
   match direction
   {
      Direction::East =>
      {
         doors.insert((i, j + 1));
         (i, j + 2)
      }
      Direction::North =>
      {
         doors.insert((i + 1, j));
         (i + 2, j)
      }
      Direction::South =>
      {
         doors.insert((i - 1, j));
         (i - 2, j)
      }
      Direction::West =>
      {
         doors.insert((i, j - 1));
         (i, j - 2)
      }
      Direction::Crossing(_) => panic!("This direction cannot be applied")
   }
}

/// takes current positions, a route and doors seen so far
/// returns new positions obtained after taking the route while updating the list of door seen
fn apply_route(mut current_position: HashSet<Position>,
               route: &Route,
               mut doors: &mut HashSet<Position>)
               -> HashSet<Position>
{
   for direction in route
   {
      match direction
      {
         Direction::Crossing(routes) =>
         {
            current_position =
               routes.iter()
                     .flat_map(|route| apply_route(current_position.clone(), &route, &mut doors))
                     .collect();
         }
         _ =>
         {
            current_position =
               current_position.iter()
                               .map(|&position| apply_direction(position, direction, &mut doors))
                               .collect();
         }
      }
   }

   current_position
}

/// returns the position of the player and a map
fn make_map(doors: &HashSet<Position>) -> ((usize, usize), Vec<Vec<Base>>)
{
   let imin = doors.iter().map(|(i, _)| i).min().unwrap();
   let imax = doors.iter().map(|(i, _)| i).max().unwrap();
   let jmin = doors.iter().map(|(_, j)| j).min().unwrap();
   let jmax = doors.iter().map(|(_, j)| j).max().unwrap();

   // find appropriate size
   let mut height = (3 + imax - imin) as usize;
   if height % 2 == 0
   {
      height += 1;
   }
   let mut width = (3 + jmax - jmin) as usize;
   if width % 2 == 0
   {
      width += 1;
   }
   let mut map = vec![vec![Base::Wall; width]; height];
   // place rooms
   for i in 0..height
   {
      for j in 0..width
      {
         if (i % 2 == 1) && (j % 2 == 1)
         {
            map[i][j] = Base::Room;
         }
      }
   }
   // place doors
   for (i, j) in doors.iter().map(|(i, j)| ((1 + i - imin) as usize, (1 + j - jmin) as usize))
   {
      map[i][j] = Base::Door;
   }
   // place initial position
   let initial_position = ((1 - imin) as usize, (1 - jmin) as usize);
   map[initial_position.0][initial_position.1] = Base::InitialPosition;

   (initial_position, map)
}

/// displays the map
fn display_map(map: &Map)
{
   fn char_of_base(b: &Base) -> char
   {
      match b
      {
         Base::Door => ' ',
         Base::InitialPosition => 'X',
         Base::Room => '.',
         Base::Wall => '#'
      }
   }

   for row in map.iter().rev()
   {
      let line: String = row.iter().map(char_of_base).collect();
      println!("{}", line)
   }
}

//-----------------------------------------------------------------------------
// PATH

/// makes a map of the distance from all points to (i,j)
/// WARNING the distance is in number of square and needs to be divided by two to be converted in doors
fn make_distance_map(initial_position: (usize, usize), map: &Map) -> Vec<Vec<usize>>
{
   let height = map.len();
   let width = map[0].len();
   let max_dist = std::usize::MAX;
   let mut distances = vec![vec![max_dist; width]; height];
   let mut current_position = vec![initial_position];
   let mut current_distance = 0;

   while !current_position.is_empty()
   {
      let mut new_positions = Vec::new();
      for (i, j) in current_position
      {
         if (map[i][j] != Base::Wall) && (distances[i][j] > current_distance)
         {
            distances[i][j] = current_distance;
            new_positions.push((i + 1, j));
            new_positions.push((i - 1, j));
            new_positions.push((i, j + 1));
            new_positions.push((i, j - 1));
         }
      }
      current_position = new_positions;
      current_distance += 1;
   }

   distances
}

fn farthest_room(initial_position: (usize, usize), map: &Map) -> usize
{
   let distances = make_distance_map(initial_position, &map);
   distances.iter()
            .flat_map(|row| row)
            .filter(|&&dist| dist != std::usize::MAX)
            .max()
            .map(|dist| dist / 2)
            .unwrap()
}

fn count_far_rooms(initial_position: (usize, usize), map: &Map, max_distance: usize) -> usize
{
   let height = map.len();
   let width = map[0].len();
   let illegal_dist = std::usize::MAX;
   let distances = make_distance_map(initial_position, &map);

   let mut result = 0;
   for i in 0..height
   {
      for j in 0..width
      {
         if map[i][j] == Base::Room
         {
            let dist = distances[i][j];
            if (dist != illegal_dist) && (dist / 2 >= max_distance)
            {
               result += 1;
            }
         }
      }
   }

   result
}

//-----------------------------------------------------------------------------
// MAIN

fn main()
{
   let input_path = "./data/input.txt";
   let regexp: Vec<char> = std::fs::read_to_string(input_path).unwrap().trim_end().chars().collect();

   // building the map
   let (route, _) = make_route(&regexp);
   let mut doors = HashSet::new();
   let mut current_position = HashSet::new();
   current_position.insert((0, 0));
   apply_route(current_position, &route, &mut doors);
   let (initial_position, map) = make_map(&doors);
   display_map(&map);

   // task1
   let distance_to_room = farthest_room(initial_position, &map);
   println!("the farthest room is at {} doors", distance_to_room);

   // task2
   let max_dist = 1000;
   let nb_rooms = count_far_rooms(initial_position, &map, max_dist);
   println!("number of rooms further than {} doors : {}", max_dist, nb_rooms);
}
