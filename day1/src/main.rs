use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// reads a file line by line and converts each line to a number
fn read_inputs(path: &str) -> Vec<i32>
{
   let file = File::open(path).expect("Failed to open input file.");
   BufReader::new(file).lines()
                       .map(|line_option| line_option.expect("Failed to load a line."))
                       .map(|line| line.parse().expect("Failed to parse an integer."))
                       .collect::<Vec<i32>>()
}

/// computes the sum of all numbers in the given vector
fn task1(numbers: &[i32]) -> i32
{
   numbers.iter().sum()
}

/// detects the value on which the sum loops for the first time
fn task2(numbers: &[i32]) -> i32
{
   let mut result = 0;
   let mut previous_results = HashSet::new();

   for number in numbers.iter().cycle()
   {
      let is_known_result = !previous_results.insert(result);
      if is_known_result
      {
         break;
      }
      result += number;
   }

   result
}

fn main()
{
   let input_path = "./data/input.txt";
   let numbers = read_inputs(input_path);

   let result1 = task1(&numbers);
   println!("Sum of all numbers : {}", result1);

   let result2 = task2(&numbers);
   println!("First repetition : {}", result2);
}
