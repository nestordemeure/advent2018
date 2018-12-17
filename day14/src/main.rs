#![feature(slice_patterns)]
use std::char;

//-----------------------------------------------------------------------------
// TASK1

/// takes a recipe and returns the number associated
/// uses a string to preserve leading zeroes
fn number_of_recipes(recipes: &[usize]) -> String
{
   recipes.iter().map(|&digit| char::from_digit(digit as u32, 10).unwrap()).collect()
}

/// what is the ten digit score produced after the first recipes_number recipies
fn task1(recipes_number: usize) -> String
{
   let mut recipes = vec![3, 7];
   let mut elf1 = 0;
   let mut elf2 = 1;

   while recipes.len() < recipes_number + 10
   {
      let rec1 = recipes[elf1];
      let rec2 = recipes[elf2];

      // add new recipes
      let total = rec1 + rec2;
      let new_rec1 = total / 10;
      let new_rec2 = total % 10;
      if new_rec1 != 0
      {
         recipes.push(new_rec1);
      }
      recipes.push(new_rec2);

      // moves the elfs
      elf1 = (elf1 + 1 + rec1) % recipes.len();
      elf2 = (elf2 + 1 + rec2) % recipes.len();
      //println!("[{:02}] ({:02}) {:?}", elf1, elf2, recipes);
   }

   number_of_recipes(&recipes[recipes_number..(recipes_number + 10)])
}

//-----------------------------------------------------------------------------
// TASK2

/// checks wether a vector ends with a given suffix
fn endswith(vec: &[usize], suffix: &[usize]) -> bool
{
   match (vec, suffix)
   {
      ([], []) => true,
      ([], _) => false,
      (_, []) => true,
      ([tail.., head], [tail_suf.., head_suf]) => (head == head_suf) && endswith(tail, tail_suf)
   }
}

/// turns a number into a recipes
fn recipes_of_number(mut recipes_number: usize) -> Vec<usize>
{
   let mut result = Vec::new();

   while recipes_number != 0
   {
      result.push(recipes_number % 10);
      recipes_number /= 10;
   }
   result.reverse();

   result
}

/// how many recipes to the left when we see our score firt appear
fn task2(recipes_number: usize) -> usize
{
   let target_recipe = recipes_of_number(recipes_number);
   let mut recipes = vec![3, 7];
   let mut elf1 = 0;
   let mut elf2 = 1;

   loop
   {
      let rec1 = recipes[elf1];
      let rec2 = recipes[elf2];
      let total = rec1 + rec2;

      // add first recipes
      let new_rec1 = total / 10;
      if new_rec1 != 0
      {
         recipes.push(new_rec1);
         if endswith(&recipes, &target_recipe)
         {
            return recipes.len() - target_recipe.len();
         }
      }

      // add second recipes
      let new_rec2 = total % 10;
      recipes.push(new_rec2);
      if endswith(&recipes, &target_recipe)
      {
         return recipes.len() - target_recipe.len();
      }

      // moves the elfs
      elf1 = (elf1 + 1 + rec1) % recipes.len();
      elf2 = (elf2 + 1 + rec2) % recipes.len();
   }
}

//-----------------------------------------------------------------------------
// MAIN

fn main()
{
   for &recipes_number in [9, 5, 18, 2018, 409551].iter()
   {
      println!("recipes number : {}", recipes_number);

      let score = task1(recipes_number);
      println!("   score : {}", score);

      let score2 = task2(recipes_number);
      println!("   score2 : {}", score2);
   }
}
