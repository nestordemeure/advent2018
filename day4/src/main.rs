#[macro_use]
extern crate scan_fmt;
use itertools::Itertools; // group_by
use std::fs::File;
use std::io::{BufRead, BufReader};

//-----------------------------------------------------------------------------
// INPUT

/// stores a time
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
struct Time
{
   year: usize,
   month: usize,
   day: usize,
   hour: usize,
   minute: usize
}

/// stores a command
enum State
{
   BeginShift(usize),
   Sleep,
   WakeUp
}

/// parses a line to produce a (Time,State)
fn parse_line(line: &str) -> (Time, State)
{
   // parses the time
   let (y, mo, d, h, mi) = scan_fmt!(&line[..18], "[{}-{}-{} {}:{}]", usize, usize, usize, usize, usize);
   let time = Time { year: y.unwrap(), month: mo.unwrap(), day: d.unwrap(), hour: h.unwrap(), minute: mi.unwrap() };

   // parses the state
   match &line[19..]
   {
      "falls asleep" => (time, State::Sleep),
      "wakes up" => (time, State::WakeUp),
      cmd_str =>
      {
         let id = scan_fmt!(cmd_str, "Guard #{} begins shift", usize).unwrap();
         (time, State::BeginShift(id))
      }
   }
}

/// reads a file line by line and converts each line to a rectangle
fn read_inputs(path: &str) -> Vec<(Time, State)>
{
   let file = File::open(path).expect("Failed to open input file.");
   BufReader::new(file).lines()
                       .map(|line_option| line_option.expect("Failed to load a line."))
                       .map(|line| parse_line(&line))
                       .collect::<Vec<(Time, State)>>()
}

//-----------------------------------------------------------------------------
// NAPS

type Minute = usize;

/// stores a nap that id made from start to end
#[derive(Clone, Copy)]
struct Nap
{
   id: usize,
   start: Minute,
   end: Minute
}

/// takes two times and an id to produce a nap
fn make_nap(id: usize, t_start: &Time, t_end: &Time) -> Nap
{
   if t_start.hour != 0 || t_end.hour != 0
   {
      panic!("A nap happened out of the midnight hour!");
   }
   else
   {
      Nap { id: id, start: t_start.minute, end: t_end.minute }
   }
}

/// takes an array of (time,state) and outputs an array of Nap
/// TODO we suppose that the input is well formed
fn collect_nap(records: &mut [(Time, State)]) -> Vec<Nap>
{
   // insures the records are in chronological order
   records.sort_unstable_by_key(|ts| ts.0);

   // groups the state by nap
   let mut result = Vec::new();
   let mut current_id = 0;
   let mut start = Time { year: 0, month: 0, day: 0, hour: 0, minute: 0 };
   for (time, state) in records.iter()
   {
      match state
      {
         State::BeginShift(id) => current_id = *id,
         State::Sleep => start = *time,
         State::WakeUp => result.push(make_nap(current_id, &start, &time))
      }
   }
   result
}

/// groups the nap by id
fn group_by_id(naps: &mut [Nap]) -> Vec<(usize, Vec<Nap>)>
{
   naps.sort_unstable_by_key(|n| n.id);
   naps.iter()
       .group_by(|n| n.id)
       .into_iter()
       .map(|(id, group)| (id, group.cloned().collect::<Vec<Nap>>()))
       .collect()
}

//-----------------------------------------------------------------------------
// TASK1

/// returns the total sleep time associated with an array of naps
fn total_sleep_time(naps: &[Nap]) -> usize
{
   naps.iter().map(|n| n.end - n.start).sum()
}

/// returns the minute during which this person sleeped the most and the number of time associated with that minute
fn most_sleeped_minute(naps: &[Nap]) -> (usize, usize)
{
   // computes the total number of nap that happened in each minute of an hour
   let mut hour = [0_usize; 60];
   for nap in naps
   {
      for minute in nap.start..nap.end
      {
         hour[minute] += 1;
      }
   }

   hour.iter().enumerate().max_by_key(|&(_minute, num)| num).map(|(minute, &num)| (minute, num)).unwrap()
}

/// find the id with the most time in naps and the minutes in which the most naps happened
fn task1(grouped_naps: &[(usize, Vec<Nap>)]) -> usize
{
   let (best_id, associated_naps) = grouped_naps.iter().max_by_key(|(_id, group)| total_sleep_time(group)).unwrap();
   let (best_minute, _nap_number) = most_sleeped_minute(&associated_naps);
   best_id * best_minute
}

//-----------------------------------------------------------------------------
// TASK2

/// find the id with that napped the most at a given minute
fn task2(grouped_naps: &[(usize, Vec<Nap>)]) -> usize
{
   let (best_id, best_minute) = grouped_naps.iter()
                                            .map(|(id, group)| (id, most_sleeped_minute(&group)))
                                            .max_by_key(|&(_, (_, nap_number))| nap_number)
                                            .map(|(id, (minute, _))| (id, minute))
                                            .unwrap();

   best_id * best_minute
}

//-----------------------------------------------------------------------------

fn main()
{
   let input_path = "./data/input.txt";
   let grouped_naps = group_by_id(&mut collect_nap(&mut read_inputs(input_path)));

   let result1 = task1(&grouped_naps);
   println!("guard that sleeps the most * most sleept minute {}", result1);

   let result2 = task2(&grouped_naps);
   println!("guard that sleeps regularly * most sleept minute {}", result2);
}
