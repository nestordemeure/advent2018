use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

//-----------------------------------------------------------------------------
// INPUT

/// reads a file line by line and converts each line to a number
fn read_inputs(path: &str) -> Vec<String>
{
   let file = File::open(path).expect("Failed to open input file.");
   BufReader::new(file).lines().map(|line_option| line_option.expect("Failed to load a line.")).collect()
}

//-----------------------------------------------------------------------------
// TASK1

/// return the number of occurence of each letter
fn count_letters(id: &str) -> Vec<i32>
{
   let mut count: HashMap<char, i32> = HashMap::new();

   for letter in id.chars()
   {
      *count.entry(letter).or_default() += 1;
   }

   count.values().cloned().collect()
}

/// returns true if at least a key is associated with n
fn contains_n(count_of_letter: &[i32], n: i32) -> bool
{
   count_of_letter.iter().any(|&v| v == n)
}

/// computes a checksum
fn task1(lines: &[String]) -> usize
{
   let count = lines.iter().map(|id| count_letters(id));

   let number_of_2 = count.clone().filter(|c| contains_n(&c, 2)).count();
   let number_of_3 = count.filter(|c| contains_n(&c, 3)).count();

   number_of_2 * number_of_3
}

//-----------------------------------------------------------------------------
// TASK2

/// returns the hamming distance between two strings
fn hamming(id1: &str, id2: &str) -> usize
{
   let chars1 = id1.chars();
   let chars2 = id2.chars();
   let different_chars = chars1.zip(chars2).filter(|(c1, c2)| c1 != c2);
   different_chars.count()
}

/// returns a string with the letter in common between two strings
fn common_letters(id1: &str, id2: &str) -> String
{
   let chars1 = id1.chars();
   let chars2 = id2.chars();
   chars1.zip(chars2).filter(|(c1, c2)| c1 == c2).map(|pair| pair.0).collect::<String>()
}

/// finds the two strings with only one letter of difference and returns the letter in common
fn task2(lines: &[String]) -> String
{
   for (i, id1) in lines.iter().enumerate()
   {
      for id2 in lines.iter().skip(i + 1)
      {
         if hamming(id1, id2) < 2
         {
            return common_letters(id1, id2);
         }
      }
   }

   panic!("Unable to find two string close enough.")
}

//-----------------------------------------------------------------------------

fn main()
{
   let input_path = "./data/input.txt";
   let lines = read_inputs(input_path);

   // compute checksum
   let signature = task1(&lines);
   println!("signature : {}", signature);

   // find two boxes that are one appart
   let common_letters = task2(&lines);
   println!("letters : {}", common_letters);
}
