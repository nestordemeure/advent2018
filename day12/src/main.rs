#![feature(slice_patterns)]
#[macro_use]
extern crate scan_fmt;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

//-----------------------------------------------------------------------------
// TYPE

type Index = usize;
const EMPTY_POT: Index = 0;
const FULL_POT: Index = 1;

type Pattern = [Index; 5];
const EMPTY_PATTERN: Pattern = [EMPTY_POT; 5];

struct State
{
   shift: i128,
   vec: Vec<Index>
}

struct Rules
{
   rules: HashMap<Pattern, Index>,
   index_of_pattern: HashMap<Pattern, Index>, // compress patterns into indexes
   pattern_of_index: Vec<Pattern>             // decompress indexes into patterns
}

//-----------------------------------------------------------------------------
// INPUT

/// takes a char and returns a plant
fn plant_of_char(c: char) -> Index
{
   match c
   {
      '#' => FULL_POT,
      '.' => EMPTY_POT,
      _ => panic!("unable to match the plant")
   }
}

/// parses the initial state
fn parse_init(line: &str) -> State
{
   let plants = scan_fmt!(&line, "initial state: {}", String).unwrap().chars().map(plant_of_char).collect();
   State { shift: 0, vec: plants }
}

/// parses a pattern and its assocated result
fn parse_rule(line: &str) -> (Pattern, Index)
{
   let (rule, plant) = scan_fmt!(&line, "{} => {}", String, char);
   let plant = plant_of_char(plant.unwrap());
   let rule: Vec<Index> = rule.unwrap().chars().map(plant_of_char).collect();
   match rule[..]
   {
      [l1, l2, c, r1, r2] => ([l1, l2, c, r1, r2], plant),
      _ => panic!("rule of unpredicted size")
   }
}

/// parses a file and returns a state and rules
fn input_data(path: &str) -> (State, Rules)
{
   let file = File::open(path).expect("Failed to open input file.");
   let mut lines =
      BufReader::new(file).lines().map(|line_option| line_option.expect("Failed to load a line."));

   let initial_state = parse_init(&lines.next().unwrap());
   lines.next();
   let raw_rules: HashMap<Pattern, Index> = lines.map(|line| parse_rule(&line)).collect();

   let pattern_of_index = vec![EMPTY_PATTERN, EMPTY_PATTERN];
   let mut index_of_pattern = HashMap::new();
   index_of_pattern.insert(EMPTY_PATTERN, EMPTY_POT);
   let rules =
      Rules { rules: raw_rules, index_of_pattern: index_of_pattern, pattern_of_index: pattern_of_index };

   (initial_state, rules)
}

//-----------------------------------------------------------------------------
// TASK1

/// adds the given number of empty pots on both sides of a state
fn expand_state(state: &State, expansion: u128) -> State
{
   // new vec
   let mut vec = vec![EMPTY_POT; expansion as usize];
   let suffix = vec![EMPTY_POT; expansion as usize];
   vec.extend(&state.vec);
   vec.extend(suffix);
   // new shift
   let shift = state.shift - (expansion as i128);
   State { shift, vec }
}

/// removes empty pots on both side of a state
fn contracts_state(state: &State) -> State
{
   let mut shift = state.shift;
   let mut vec = &state.vec[..];

   loop
   {
      match vec
      {
         [EMPTY_POT, tail..] =>
         {
            vec = tail;
            shift += 1;
         }
         [tail.., EMPTY_POT] => vec = tail,
         _ => return State { shift: shift, vec: vec.to_vec() }
      }
   }
}

//-----------------------------------------------

/// gets the result of the pattern according to the rules
fn apply_rules(pattern: &Pattern, mut rules: &mut Rules) -> Index
{
   match rules.rules.get(pattern)
   {
      Some(&index) => index,
      None =>
      {
         let mut state = State { shift: 0, vec: pattern.to_vec() };

         // each step in compressed form is 5 steps in decompressed form
         state = decompress_state(&state, &rules);
         for _ in 0..5
         {
            state = next_state(&state, &mut rules);
         }
         state = compress_state(&state, &mut rules);

         // 5 patterns reduce to a single pattern after 5 steps
         let index = state.vec[0];
         rules.rules.insert(*pattern, index);
         index
      }
   }
}

/// goes to the next state
/// WARNING: requires some buffering as it will truncate the state
fn next_state(state: &State, mut rules: &mut Rules) -> State
{
   let vec = &state.vec;
   let mut new_vec = Vec::new();

   for i in 2..(vec.len() - 2)
   {
      let pattern = [vec[i - 2], vec[i - 1], vec[i], vec[i + 1], vec[i + 2]];
      let index = apply_rules(&pattern, &mut rules);
      new_vec.push(index);
   }

   State { shift: state.shift + 2, vec: new_vec }
}

/// goes forward by n steps
fn next_n_state(state: &State, mut rules: &mut Rules, n: u128) -> State
{
   let mut state = expand_state(&state, n * 4);

   for _ in 0..n
   {
      state = next_state(&state, &mut rules);
   }

   contracts_state(&state)
}

//-----------------------------------------------------------------------------
// TASK2

/// takes a pattern and produces an index
fn compress_pattern(pattern: &Pattern, rules: &mut Rules) -> Index
{
   match rules.index_of_pattern.get(pattern)
   {
      Some(index) => *index,
      None =>
      {
         let index = rules.pattern_of_index.len();
         rules.pattern_of_index.push(*pattern);
         rules.index_of_pattern.insert(*pattern, index);
         index
      }
   }
}

/// express each block of 5 indexes as a single index
fn compress_state(state: &State, mut rules: &mut Rules) -> State
{
   // insures that the shift starts with a multiple of 5
   if state.shift % 5 != 0
   {
      let buffer_size = (state.shift % 5) as u128;
      let state = expand_state(&state, buffer_size);
      return compress_state(&state, &mut rules);
   }

   // get the indexes by block of 5 (patterns) and stores the corresponding indexes
   let mut vec = Vec::new();
   let mut old_vec = &state.vec[..];
   while !old_vec.is_empty()
   {
      match old_vec
      {
         [i0, i1, i2, i3, i4, tail..] =>
         {
            let pattern = [*i0, *i1, *i2, *i3, *i4];
            let index = compress_pattern(&pattern, &mut rules);
            vec.push(index);
            old_vec = tail;
         }
         _ =>
         {
            let mut it = old_vec.iter();
            let i0 = it.next().unwrap_or(&EMPTY_POT);
            let i1 = it.next().unwrap_or(&EMPTY_POT);
            let i2 = it.next().unwrap_or(&EMPTY_POT);
            let i3 = it.next().unwrap_or(&EMPTY_POT);
            let i4 = it.next().unwrap_or(&EMPTY_POT);
            let pattern = [*i0, *i1, *i2, *i3, *i4];
            let index = compress_pattern(&pattern, &mut rules);
            vec.push(index);
            old_vec = &[];
         }
      }
   }

   let shift = state.shift / 5;
   State { shift, vec }
}

/// express each index as a block of 5 index
fn decompress_state(state: &State, rules: &Rules) -> State
{
   let vec: Vec<Index> =
      state.vec.iter().flat_map(|&index| rules.pattern_of_index[index].iter()).cloned().collect();
   let shift = state.shift * 5;
   State { shift, vec }
}

//-----------------------------------------------

/// goes forward by n steps
fn hash_next_n_state(state: &State, mut rules: &mut Rules, n: u128) -> State
{
   let mut state = next_n_state(&state, &mut rules, n % 5);

   if n / 5 == 0
   {
      state
   }
   else
   {
      state = compress_state(&state, &mut rules);
      state = hash_next_n_state(&state, &mut rules, n / 5);
      state = decompress_state(&state, &rules);
      contracts_state(&state)
   }
}

//-----------------------------------------------------------------------------
// MAIN

/// returns the value of a state (sum of, potentially negativ, indexes of non empty pots)
fn evaluate_state(state: &State) -> i128
{
   let mut sum = 0;

   for (i, &index) in state.vec.iter().enumerate()
   {
      match index
      {
         FULL_POT => sum += (i as i128) + state.shift,
         EMPTY_POT => (),
         _ => panic!("A compressed index survived into the evaluation phase!")
      }
   }

   sum
}

fn main()
{
   let input_path = "./data/input.txt";
   let (initial_state, mut rules) = input_data(input_path);

   // short steps
   let short_time = 20;
   let state_after_short = next_n_state(&initial_state, &mut rules, short_time);
   let score_after_short = evaluate_state(&state_after_short);
   println!("score after short time : {}", score_after_short);

   // long steps
   let long_time = 1_000_000_000_000_000_000_000_000_000_000_000_000; // 50_000_000_000
   let state_after_long = hash_next_n_state(&initial_state, &mut rules, long_time);
   let score_after_long = evaluate_state(&state_after_long);
   println!("score after long time : {}", score_after_long);
}
