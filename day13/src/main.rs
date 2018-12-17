#![feature(slice_patterns)]
use std::fs::File;
use std::io::{BufRead, BufReader};

//-----------------------------------------------------------------------------
// TYPE

/// describe the terrain
#[derive(Clone, Copy, PartialEq)]
enum Road
{
   None,
   Vertical,
   Horizontal,
   RightCorner,
   LeftCorner,
   Intersection
}

/// describe the direction in which a cart is going
#[derive(Clone, Copy, PartialEq)]
enum Direction
{
   Up,
   Down,
   Left,
   Right
}

/// describe the next turn that the cart will do on an intersection
#[derive(Clone, Copy, PartialEq)]
enum Turn
{
   Left,
   Straight,
   Right
}

#[derive(Clone, Copy, PartialEq)]
struct Cart
{
   direction: Direction,
   turn: Turn,
   last_tick: usize
}

type Terrain = [Vec<Road>];
type Vehicules = [Vec<Option<Cart>>];

//-----------------------------------------------------------------------------
// INPUT

/// makes a new cart with the default direction
fn new_cart(dir: Direction) -> Cart
{
   Cart { direction: dir, turn: Turn::Left, last_tick: 0 }
}

/// identifies the road and cart displayed by a char
fn parse_char(c: char) -> (Road, Option<Cart>)
{
   match c
   {
      // terrain
      ' ' => (Road::None, None),
      '|' => (Road::Vertical, None),
      '-' => (Road::Horizontal, None),
      '+' => (Road::Intersection, None),
      '/' => (Road::LeftCorner, None),
      '\\' => (Road::RightCorner, None),
      // cart
      '>' => (Road::Horizontal, Some(new_cart(Direction::Right))),
      '<' => (Road::Horizontal, Some(new_cart(Direction::Left))),
      '^' => (Road::Vertical, Some(new_cart(Direction::Up))),
      'v' => (Road::Vertical, Some(new_cart(Direction::Down))),
      //'X' => (Road::NoRoad, Cart::Collision),
      // error
      _ => panic!("Unrecognisezd char!")
   }
}

/// parses a line into a road and carts
fn parse_line(line: &str) -> (Vec<Road>, Vec<Option<Cart>>)
{
   line.chars().map(parse_char).unzip()
}

/// parses a file and returns (points, speeds)
fn input_data(path: &str) -> (Vec<Vec<Road>>, Vec<Vec<Option<Cart>>>)
{
   let file = File::open(path).expect("Failed to open input file.");
   BufReader::new(file).lines()
                       .map(|line_option| line_option.expect("Failed to load a line."))
                       .map(|line| parse_line(&line))
                       .unzip()
}

//-----------------------------------------------------------------------------
// TASKS

/// returns the position of the first cart it finds
fn find_a_cart(carts: &Vehicules) -> (usize, usize)
{
   for row in 0..carts.len()
   {
      for col in 0..carts[0].len()
      {
         if carts[row][col].is_some()
         {
            return (row, col);
         }
      }
   }

   panic!("No cart left after the last collision");
}

/// returns new coordinate produced by a direction
fn apply_direction(direction: Direction, row: usize, col: usize) -> (usize, usize)
{
   match direction
   {
      Direction::Right => (row, col + 1),
      Direction::Left => (row, col - 1),
      Direction::Up => (row - 1, col),
      Direction::Down => (row + 1, col)
   }
}

/// returns the direction after a right turn
fn turn_right(direction: Direction) -> Direction
{
   match direction
   {
      Direction::Down => Direction::Left,
      Direction::Left => Direction::Up,
      Direction::Right => Direction::Down,
      Direction::Up => Direction::Right
   }
}

/// returns the direction after a left turn
fn turn_left(direction: Direction) -> Direction
{
   match direction
   {
      Direction::Down => Direction::Right,
      Direction::Left => Direction::Down,
      Direction::Right => Direction::Up,
      Direction::Up => Direction::Left
   }
}

/// returns a cart after aplying any terrain related effect
fn apply_terrain(direction: Direction, turn: Turn, road: Road) -> (Direction, Turn)
{
   match road
   {
      Road::None => panic!("No car should be out of the road"),
      Road::Horizontal => match direction
      {
         Direction::Left | Direction::Right => (direction, turn),
         _ => panic!("The car has an illegal direction on this horizontal road")
      },
      Road::Vertical => match direction
      {
         Direction::Up | Direction::Down => (direction, turn),
         _ => panic!("The car has an illegal direction on this vertical road")
      },
      Road::LeftCorner => match direction
      {
         Direction::Down => (Direction::Left, turn),
         Direction::Up => (Direction::Right, turn),
         Direction::Right => (Direction::Up, turn),
         Direction::Left => (Direction::Down, turn)
      },
      Road::RightCorner => match direction
      {
         Direction::Down => (Direction::Right, turn),
         Direction::Up => (Direction::Left, turn),
         Direction::Right => (Direction::Down, turn),
         Direction::Left => (Direction::Up, turn)
      },
      Road::Intersection => match turn
      {
         Turn::Straight => (direction, Turn::Right),
         Turn::Right => (turn_right(direction), Turn::Left),
         Turn::Left => (turn_left(direction), Turn::Straight)
      }
   }
}

/// plays one tick and returns a vector of all collisions that happened during this tick in order
fn one_tick(tick_number: usize, terrain: &Terrain, carts: &mut Vehicules) -> Vec<(usize, usize)>
{
   let mut collisions = Vec::new();

   for row in 0..carts.len()
   {
      for col in 0..carts[0].len()
      {
         match carts[row][col]
         {
            Some(cart) if cart.last_tick == tick_number =>
            {
               let (new_row, new_col) = apply_direction(cart.direction, row, col);
               if carts[new_row][new_col].is_none()
               {
                  let (direction, turn) = apply_terrain(cart.direction, cart.turn, terrain[new_row][new_col]);
                  let new_cart = Cart { direction: direction, turn: turn, last_tick: tick_number + 1 };
                  carts[new_row][new_col] = Some(new_cart);
               }
               else
               {
                  collisions.push((new_row, new_col));
                  carts[new_row][new_col] = None;
               }
               carts[row][col] = None;
            }
            _ => ()
         }
      }
   }

   collisions
}

/// returns the location of the first collision
fn first_collision(terrain: &Terrain, carts: &Vehicules) -> (usize, usize)
{
   let mut tick_number = 0;
   let mut carts = carts.to_vec();
   let mut collisions = one_tick(tick_number, &terrain, &mut carts);

   while collisions.is_empty()
   {
      tick_number += 1;
      collisions = one_tick(tick_number, &terrain, &mut carts);
   }

   collisions[0]
}

/// returns the position of the last cart standing after all the other collisions
fn last_cart(terrain: &Terrain, carts: &Vehicules) -> (usize, usize)
{
   let mut cart_number = carts.iter().flat_map(|row| row).filter(|cart| cart.is_some()).count();
   let mut carts = carts.to_vec();
   let mut tick_number = 0;

   while cart_number > 1
   {
      let collisions = one_tick(tick_number, &terrain, &mut carts);
      cart_number -= 2 * collisions.len();
      tick_number += 1;
   }

   find_a_cart(&carts)
}

//-----------------------------------------------------------------------------
// MAIN

fn main()
{
   let input_path = "./data/input.txt";
   let (terrain, carts) = input_data(input_path);

   // task1
   let (yfirst, xfirst) = first_collision(&terrain, &carts);
   println!("first collision ({},{})", xfirst, yfirst);

   // task2
   let (ylast, xlast) = last_cart(&terrain, &carts);
   println!("last cart ({},{})", xlast, ylast);
}
