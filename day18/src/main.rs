use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

//-----------------------------------------------------------------------------
// TYPE

/// describe the terrain
#[derive(Clone, Copy, PartialEq)]
enum Acre
{
   Tree,
   Ground,
   Lumberyard
}

/// describes the neibourhood of an acre
struct Neigbourhood
{
   nb_tree: usize,
   nb_ground: usize,
   nb_lumberyard: usize
}

type Lumber = [Vec<Acre>];

//-----------------------------------------------------------------------------
// INPUT

fn parse_char(c: char) -> Acre
{
   match c
   {
      '|' => Acre::Tree,
      '.' => Acre::Ground,
      '#' => Acre::Lumberyard,
      _ => panic!("Unrecognisezd char!")
   }
}

fn parse_line(line: &str) -> Vec<Acre>
{
   line.chars().map(parse_char).collect()
}

fn input_data(path: &str) -> Vec<Vec<Acre>>
{
   let file = File::open(path).expect("Failed to open input file.");
   BufReader::new(file).lines()
                       .map(|line_option| line_option.expect("Failed to load a line."))
                       .map(|line| parse_line(&line))
                       .collect()
}

//-----------------------------------------------------------------------------
// SIMULATE

/// returns the neibourhood of a cell
fn get_neibours(i0: usize, j0: usize, lumber: &Lumber) -> Neigbourhood
{
   let mut neibourhood = Neigbourhood { nb_tree: 0, nb_ground: 0, nb_lumberyard: 0 };

   let mini = if i0 == 0 { 0 } else { i0 - 1 };
   let maxi = std::cmp::min(i0 + 1, lumber.len() - 1);
   let minj = if j0 == 0 { 0 } else { j0 - 1 };
   let maxj = std::cmp::min(j0 + 1, lumber[0].len() - 1);

   for i in mini..=maxi
   {
      for j in minj..=maxj
      {
         if (i != i0) || (j != j0)
         {
            match lumber[i][j]
            {
               Acre::Ground => neibourhood.nb_ground += 1,
               Acre::Lumberyard => neibourhood.nb_lumberyard += 1,
               Acre::Tree => neibourhood.nb_tree += 1
            }
         }
      }
   }

   neibourhood
}

/// returns the future state of a cell
fn simulate_acre(i: usize, j: usize, lumber: &Lumber) -> Acre
{
   let n = get_neibours(i, j, &lumber);
   match lumber[i][j]
   {
      Acre::Ground if n.nb_tree >= 3 => Acre::Tree,
      Acre::Tree if n.nb_lumberyard >= 3 => Acre::Lumberyard,
      Acre::Lumberyard if n.nb_tree == 0 || n.nb_lumberyard == 0 => Acre::Ground,
      acre => acre
   }
}

/// returns the future state of a whole lumber
fn simulate_one_minute(lumber: &Lumber) -> Vec<Vec<Acre>>
{
   (0..lumber.len()).map(|i| (0..lumber[0].len()).map(|j| simulate_acre(i, j, &lumber)).collect()).collect()
}

/// simulate n minutes
fn simulate(lumber: &Lumber, nb_minutes: usize) -> Vec<Vec<Acre>>
{
   let mut lumber = lumber.to_vec();

   for _minute in 0..nb_minutes
   {
      lumber = simulate_one_minute(&lumber);
   }

   lumber
}

/// simulate n minutes but skip ahead once a period has been found in the scores
fn simulate_periodic(lumber: &Lumber, nb_minutes: usize) -> Vec<Vec<Acre>>
{
   let mut lumber = lumber.to_vec();
   let mut previous_scores = HashMap::new();
   let mut potential_period = 0;

   for minute in 0..nb_minutes
   {
      lumber = simulate_one_minute(&lumber);

      let score = evaluate(&lumber);
      if let Some(previous_minute) = previous_scores.get(&score)
      {
         // we found a period
         let period = minute - previous_minute;
         if period == potential_period
         {
            // the period has been confirmed
            println!("Found a period of {}.", period);
            let time_left = (nb_minutes - minute - 1) % period;
            return simulate(&lumber, time_left);
         }
         else
         {
            potential_period = period;
         }
      }
      previous_scores.insert(score, minute);
   }

   lumber
}

//-----------------------------------------------------------------------------
// MAIN

fn evaluate(lumber: &Lumber) -> usize
{
   let mut nb_tree = 0;
   let mut nb_lumberyard = 0;

   for row in lumber
   {
      for acre in row
      {
         match acre
         {
            Acre::Lumberyard => nb_lumberyard += 1,
            Acre::Tree => nb_tree += 1,
            Acre::Ground => ()
         }
      }
   }

   nb_tree * nb_lumberyard
}

fn main()
{
   let input_path = "./data/input.txt";
   let lumber = input_data(input_path);

   // task1
   let nb_minutes = 10;
   let new_lumber = simulate(&lumber, nb_minutes);
   let score = evaluate(&new_lumber);
   println!("Score after {} minutes : {}", nb_minutes, score);

   // task2
   let nb_big_minutes = 1_000_000_000;
   let new_lumber = simulate_periodic(&lumber, nb_big_minutes);
   let score = evaluate(&new_lumber);
   println!("Score after {} minutes : {}", nb_big_minutes, score);
}
