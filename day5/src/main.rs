#![feature(slice_patterns)]

//-----------------------------------------------------------------------------
// TASK1

/// two different chars with identical lowercase representation
fn is_complement(c1: char, c2: char) -> bool
{
   (c1 != c2) && (c1.to_ascii_lowercase() == c2.to_ascii_lowercase())
}

/// takes a polymer and simplifies its chars
/// TODO could reduce memory consumption by taking an iterator as input
fn simplify(polymer: &[char]) -> Vec<char>
{
   let mut result = Vec::new();

   for c1 in polymer
   {
      match result.last()
      {
         Some(c2) if is_complement(*c1, *c2) =>
         {
            result.pop();
         }
         _ => result.push(*c1)
      }
   }

   result
}

/// simplifies a polymer and returns its length
fn task1(polymer: &[char]) -> usize
{
   simplify(&polymer).len()
}

//-----------------------------------------------------------------------------
// TASK2

/// removes a char from a polymer and returns the length of its simplification
fn test_char(polymer: &[char], c_lowercase: char) -> usize
{
   let c_uppercase = c_lowercase.to_ascii_uppercase();
   let simplified_polymer: Vec<char> =
      polymer.iter().filter(|&&u| u != c_lowercase && u != c_uppercase).cloned().collect();
   task1(&simplified_polymer)
}

/// returns the minimal length of the polymer after simplification by a char
fn task2(polymer: &[char]) -> usize
{
   "abcdefghijklmnopqrstuvwxyz".chars().map(|c| test_char(&polymer, c)).min().unwrap()
}

//-----------------------------------------------------------------------------
// MAIN

fn main()
{
   let input_path = "./data/input.txt";
   let polymer: Vec<char> = std::fs::read_to_string(input_path).unwrap().trim_end().chars().collect();

   // task1
   let unit_number = task1(&polymer);
   println!("length of simplified polymer : {}", unit_number);

   // task2
   let optimal_unit_number = task2(&polymer);
   println!("length of optimal simplified polymer : {}", optimal_unit_number);
}
