#[macro_use]
extern crate scan_fmt;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

//-----------------------------------------------------------------------------
// INSTRUCTION

#[derive(PartialEq, Eq, Hash, Clone)]
enum Operation
{
   Add,
   Mul,
   And,
   Or,
   Set,
   Greater,
   Equal
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Instruction
{
   name: String,
   a_is_register: bool,
   b_is_register: bool,
   op: Operation
}

struct Call
{
   instruction: Instruction,
   a: usize,
   b: usize,
   c: usize
}

//-----------------------------------------------------------------------------
// INPUT

fn parse_instruction_pointer(line: &str) -> usize
{
   let ip = scan_fmt!(&line, "#ip {}", usize);
   ip.unwrap()
}

fn parse_call(line: &str) -> Call
{
   let (instr, a, b, c) = scan_fmt!(&line, "{} {} {} {}", String, usize, usize, usize);
   let instr = instr.unwrap();
   let instr = match instr.as_ref()
   {
      "addr" => Instruction { name: instr, a_is_register: true, b_is_register: true, op: Operation::Add },
      "addi" => Instruction { name: instr, a_is_register: true, b_is_register: false, op: Operation::Add },
      "mulr" => Instruction { name: instr, a_is_register: true, b_is_register: true, op: Operation::Mul },
      "muli" => Instruction { name: instr, a_is_register: true, b_is_register: false, op: Operation::Mul },
      "banr" => Instruction { name: instr, a_is_register: true, b_is_register: true, op: Operation::And },
      "bani" => Instruction { name: instr, a_is_register: true, b_is_register: false, op: Operation::And },
      "borr" => Instruction { name: instr, a_is_register: true, b_is_register: true, op: Operation::Or },
      "bori" => Instruction { name: instr, a_is_register: true, b_is_register: false, op: Operation::Or },
      "setr" => Instruction { name: instr, a_is_register: true, b_is_register: true, op: Operation::Set },
      "seti" => Instruction { name: instr, a_is_register: false, b_is_register: false, op: Operation::Set },
      "gtir" =>
      {
         Instruction { name: instr, a_is_register: false, b_is_register: true, op: Operation::Greater }
      }
      "gtri" =>
      {
         Instruction { name: instr, a_is_register: true, b_is_register: false, op: Operation::Greater }
      }
      "gtrr" => Instruction { name: instr, a_is_register: true, b_is_register: true, op: Operation::Greater },
      "eqir" => Instruction { name: instr, a_is_register: false, b_is_register: true, op: Operation::Equal },
      "eqri" => Instruction { name: instr, a_is_register: true, b_is_register: false, op: Operation::Equal },
      "eqrr" => Instruction { name: instr, a_is_register: true, b_is_register: true, op: Operation::Equal },
      _ => panic!("unknown instruction")
   };
   Call { instruction: instr, a: a.unwrap(), b: b.unwrap(), c: c.unwrap() }
}

/// parses a file and returns (ip, calls)
fn input_data(path: &str) -> (usize, Vec<Call>)
{
   let file = File::open(path).expect("Failed to open input file.");
   let mut lines =
      BufReader::new(file).lines().map(|line_option| line_option.expect("Failed to load a line."));

   let ip = parse_instruction_pointer(&lines.next().unwrap());
   let calls = lines.map(|line| parse_call(&line)).collect();
   (ip, calls)
}

//-----------------------------------------------------------------------------
// DISPLAY

/// displays the pseudo-assembly and does some conversions to improve readability
/// this function is meant to do minimal preprocessing before a human analysis
/// we could improve the input in steps :
/// - translate all ip_register in arguments to the line number
/// - translate all ip_register in target to goto 1 + value
/// - compute all computation that do not use any variable
/// - use += and *= when possible
/// the next step would be to convert goto into tests and later loops
fn display(ip: usize, calls: &[Call])
{
   for (line, call) in calls.iter().enumerate()
   {
      print!("{}: ", line);
      match call.instruction.name.as_ref()
      {
         "addr" if call.c == ip =>
         {
            if call.a == ip
            {
               println!("goto ({} + x{})", line + 1, call.b)
            }
            else if call.b == ip
            {
               println!("goto ({} + x{})", line + 1, call.a)
            }
            else
            {
               println!("goto (x{} + x{})", call.a, call.b)
            }
         }
         "addr" => println!("x{} = x{} + x{}", call.c, call.a, call.b),
         "addi" if call.c == ip =>
         {
            if call.a == ip
            {
               println!("goto {}", line + 1 + call.b)
            }
            else
            {
               println!("goto (x{} + {})", call.a, call.b + 1)
            }
         }
         "addi" => println!("x{} = x{} + {}", call.c, call.a, call.b),
         "mulr" => println!("x{} = x{} * x{}", call.c, call.a, call.b),
         "muli" => println!("x{} = x{} * {}", call.c, call.a, call.b),
         "setr" => println!("x{} = x{}", call.c, call.a),
         "seti" if call.c == ip => println!("goto {}", 1 + call.a),
         "seti" => println!("x{} = {}", call.c, call.a),
         "gtir" => println!("x{} = if {} > x{} then 1 else 0", call.c, call.a, call.b),
         "gtri" => println!("x{} = if x{} > {} then 1 else 0", call.c, call.a, call.b),
         "gtrr" => println!("x{} = if x{} > x{} then 1 else 0", call.c, call.a, call.b),
         "eqir" => println!("x{} = if {} == x{} then 1 else 0", call.c, call.a, call.b),
         "eqri" => println!("x{} = if x{} == {} then 1 else 0", call.c, call.a, call.b),
         "eqrr" => println!("x{} = if x{} == x{} then 1 else 0", call.c, call.a, call.b),
         "banr" => println!("x{} = x{} & x{}", call.c, call.a, call.b),
         "bani" => println!("x{} = x{} & {}", call.c, call.a, call.b),
         "borr" => println!("x{} = x{} | x{}", call.c, call.a, call.b),
         "bori" => println!("x{} = x{} | {}", call.c, call.a, call.b),
         _ => panic!("unknown instruction")
      }
   }
}

//-----------------------------------------------------------------------------
// TASK

/// does one loop of the input program (gotten from a manual dissasembly)
/// returns the content or register3
/// wich happens to be the value register0 should have in order to stop at this particular iteration
fn one_loop(mut x3: u64) -> u64
{
   let mut x1 = x3 | 65536;
   x3 = 10373714;
   x3 += x1 & 255;
   x3 &= 16777215;
   x3 *= 65899;
   x3 &= 16777215;

   while 256 <= x1
   {
      let mut x4 = 256;
      let mut x5 = 0;
      while x4 <= x1
      {
         x5 += 1;
         x4 = x5 + 1;
         x4 *= 256;
      }

      x1 = x5;
      x5 = x1 & 255;
      x3 += x5;
      x3 &= 16777215;
      x3 *= 65899;
      x3 &= 16777215;
   }

   x3
}

/// returns the last x3 before going into a cycle
fn one_period(mut x3: u64) -> u64
{
   let mut previous_x3 = HashSet::new();

   loop
   {
      let new_x3 = one_loop(x3);
      let alreaddy_known = !previous_x3.insert(new_x3);
      if alreaddy_known
      {
         return x3;
      }
      else
      {
         x3 = new_x3;
      }
   }
}

//-----------------------------------------------------------------------------
// MAIN

fn main()
{
   let input_path = "./data/input.txt";
   let (ip, calls) = input_data(input_path);
   display(ip, &calls);

   // task1
   let x0_fewest_iterations = one_loop(0);
   println!("x0 that stops at the fewest number of iterations : {}", x0_fewest_iterations);

   // task1
   let x0_most_iterations = one_period(0);
   println!("x0 that stops at the biggest (non-infinite) number of iterations : {}", x0_most_iterations);
}
