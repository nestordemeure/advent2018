#![feature(vec_remove_item)]
#[macro_use]
extern crate scan_fmt;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

//-----------------------------------------------------------------------------
// INPUT

type Task = char;

/// parses a line to produce a dependency
fn parse_instruction(line: &str) -> (Task, Task)
{
   let (step1, step2) = scan_fmt!(line, "Step {} must be finished before step {} can begin.", Task, Task);
   (step1.unwrap(), step2.unwrap())
}

/// reads a file line by line and converts each line to a dependency
fn read_inputs(path: &str) -> Vec<(Task, Task)>
{
   let file = File::open(path).expect("Failed to open input file.");
   BufReader::new(file).lines()
                       .map(|line_option| line_option.expect("Failed to load a line."))
                       .map(|line| parse_instruction(&line))
                       .collect()
}

//-----------------------------------------------------------------------------
// TASK1

/// takes an array of dependencies and builds a graph of (task, dependencies)
fn build_dependency_graph(dependencies: &[(Task, Task)]) -> HashMap<Task, Vec<Task>>
{
   let mut result: HashMap<Task, Vec<Task>> = HashMap::new();

   for &(step1, step2) in dependencies
   {
      result.entry(step2).or_default().push(step1);
      result.entry(step1).or_default();
   }

   result
}

/// finds the first task, in alphabetical order, with no dependencies
fn next_task(dependencies: &HashMap<Task, Vec<Task>>) -> Option<Task>
{
   dependencies.iter().filter(|(_, dep)| dep.is_empty()).map(|(&t, _)| t).min()
}

/// removes a task from all dependencies
fn remove_task_from_dep(dependencies: &mut HashMap<Task, Vec<Task>>, task: Task)
{
   for dep in dependencies.values_mut()
   {
      dep.remove_item(&task);
   }
}

/// finds an order in which to execute the tasks
fn task1(mut dependencies: HashMap<Task, Vec<Task>>) -> String
{
   let mut result = Vec::new();

   while !dependencies.is_empty()
   {
      let task = next_task(&dependencies).unwrap();
      result.push(task);

      dependencies.remove(&task);
      remove_task_from_dep(&mut dependencies, task);
   }

   result.into_iter().collect()
}

//-----------------------------------------------------------------------------
// TASK2

/// returns 60 + numerotation of task
fn time_of_task(task: Task) -> usize
{
   (task as usize) - ('A' as usize) + 60
}

/// take a worker and consume one second of work
/// returns None and update the number of free workers if he is finished
fn work_one_sec(worker: (Task, usize),
                mut dependencies: &mut HashMap<Task, Vec<Task>>,
                free_workers_number: &mut usize)
                -> Option<(Task, usize)>
{
   match worker
   {
      (task, 0) =>
      {
         remove_task_from_dep(&mut dependencies, task);
         *free_workers_number += 1;
         None
      }
      (task, time) => Some((task, time - 1))
   }
}

/// if possible, assign a worker to a task
fn assign_worker(dependencies: &mut HashMap<Task, Vec<Task>>,
                 free_workers_number: &mut usize)
                 -> Option<(Task, usize)>
{
   match next_task(&dependencies)
   {
      None => None,
      Some(task) =>
      {
         dependencies.remove(&task);
         *free_workers_number -= 1;
         Some((task, time_of_task(task)))
      }
   }
}

fn task2(mut dependencies: HashMap<Task, Vec<Task>>, worker_number: usize) -> usize
{
   let mut second = 0;
   let mut free_workers_number = worker_number;
   let mut workers: Vec<(Task, usize)> = Vec::new();

   while !(dependencies.is_empty() && workers.is_empty())
   {
      // consume one second on each worker
      workers = workers.into_iter()
                       .filter_map(|worker| work_one_sec(worker, &mut dependencies, &mut free_workers_number))
                       .collect();

      // put free workers on available tasks
      let new_workers =
         (1..=free_workers_number).filter_map(|_| assign_worker(&mut dependencies, &mut free_workers_number));
      workers.extend(new_workers);

      // advance time
      second += 1;
   }

   second - 1
}

//-----------------------------------------------------------------------------
// MAIN

fn main()
{
   let input_path = "./data/input.txt";
   let dependencies_vec = read_inputs(input_path);
   let dependencies_graph = build_dependency_graph(&dependencies_vec);

   /*println!("Dependencies : ");
   for (t,dep) in dependencies_graph.iter()
   {
      println!("{} depends on {:?}", t, dep);
   }*/

   // task1
   let order = task1(dependencies_graph.clone());
   println!("order : {}", order);

   // task2
   let worker_number = 5;
   let time = task2(dependencies_graph, worker_number);
   println!("time : {}", time);
}
