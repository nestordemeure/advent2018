#[macro_use]
extern crate scan_fmt;
use std::collections::LinkedList;

//-----------------------------------------------------------------------------
// INPUT

/// parses a file and returns (number of players, number of marbles)
fn input_data(path: &str) -> (usize, usize)
{
   let line = std::fs::read_to_string(path).unwrap();
   let (nb_players, nb_marbles) =
      scan_fmt!(&line, "{} players; last marble is worth {} points", usize, usize);
   (nb_players.unwrap(), nb_marbles.unwrap())
}

//-----------------------------------------------------------------------------
// TASK1

/// move the index clockwise by the given shift modulo the size of the circle
fn clockwise(current_index: usize, circle_size: usize, shift: usize) -> usize
{
   (current_index + shift) % circle_size
}

/// move the index counter-clockwise by the given shift modulo the size of the circle
fn counter_clockwise(current_index: usize, circle_size: usize, shift: usize) -> usize
{
   ((current_index + circle_size) - shift) % circle_size
}

/// simulates a game of the given size and returns the highest score
/// WARNING: uses a vector to represent the circle, insertion and deletion are o(n) operations
fn game(nb_players: usize, nb_marbles: usize) -> usize
{
   let mut scores = vec![0; nb_players];
   let mut circle = vec![0];
   let mut current_index = 0;

   for marble in 1..=nb_marbles
   {
      if marble % 23 == 0
      {
         current_index = counter_clockwise(current_index, circle.len(), 7);
         let removed_value = circle.remove(current_index);
         let player = marble % nb_players;
         scores[player] += removed_value + marble;
      }
      else
      {
         current_index = clockwise(current_index, circle.len(), 2);
         circle.insert(current_index, marble);
      }
   }

   scores.iter().max().cloned().unwrap()
}

//-----------------------------------------------------------------------------
// TASK2

/// moves the current position in the circle a given number of places clockwise
fn move_clockwise(circle: &mut LinkedList<usize>, shift: usize)
{
   for _ in 0..shift
   {
      let head = circle.pop_front().unwrap();
      circle.push_back(head);
   }
}

/// moves the current position in the circle a given number of places counter-clockwise
fn move_counter_clockwise(circle: &mut LinkedList<usize>, shift: usize)
{
   for _ in 0..shift
   {
      let head = circle.pop_back().unwrap();
      circle.push_front(head);
   }
}

/// simulates a game of the given size and returns the highest score
/// uses a double ended linked list to represent the circle, the position of the current marble is its front
fn game_list(nb_players: usize, nb_marbles: usize) -> usize
{
   let mut scores = vec![0; nb_players];
   let mut circle = LinkedList::new();
   circle.push_back(0);

   for marble in 1..=nb_marbles
   {
      if marble % 23 == 0
      {
         move_counter_clockwise(&mut circle, 7);
         let removed_value = circle.pop_front().unwrap();
         let player = marble % nb_players;
         scores[player] += removed_value + marble;
      }
      else
      {
         move_clockwise(&mut circle, 2);
         circle.push_front(marble);
      }
   }

   scores.iter().max().cloned().unwrap()
}

//-----------------------------------------------------------------------------
// MAIN

fn main()
{
   let input_path = "./data/input.txt";
   let (nb_players, nb_marbles) = input_data(input_path);

   // task1
   let highscore = game(nb_players, nb_marbles);
   println!("high score : {}", highscore);

   // task2
   let highscore = game_list(nb_players, 100 * nb_marbles);
   println!("high score (time 100) : {}", highscore);
}
