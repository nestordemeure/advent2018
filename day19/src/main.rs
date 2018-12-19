#[macro_use]
extern crate scan_fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

//-----------------------------------------------------------------------------
// INSTRUCTION

type Register = [usize; 6];

#[derive(PartialEq, Eq, Hash, Clone)]
enum Operation
{
   Add,
   Mul,
   //And,
   //Or,
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
      //"banr" => Instruction { name: instr, a_is_register: true, b_is_register: true, op: Operation::And },
      //"bani" => Instruction { name: instr, a_is_register: true, b_is_register: false, op: Operation::And },
      //"borr" => Instruction { name: instr, a_is_register: true, b_is_register: true, op: Operation::Or },
      //"bori" => Instruction { name: instr, a_is_register: true, b_is_register: false, op: Operation::Or },
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
// EXECUTION

/// applies an operation to two inputs
fn apply_operation(op: &Operation, a: usize, b: usize) -> usize
{
   match op
   {
      Operation::Add => a + b,
      Operation::Mul => a * b,
      //Operation::And => a & b,
      //Operation::Or => a | b,
      Operation::Set => a,
      Operation::Greater if a > b => 1,
      Operation::Greater => 0,
      Operation::Equal if a == b => 1,
      Operation::Equal => 0
   }
}

/// applies a call to a register
fn apply_call(call: &Call, register: &Register) -> Register
{
   let mut result = *register;
   let a = if call.instruction.a_is_register { register[call.a] } else { call.a };
   let b = if call.instruction.b_is_register { register[call.b] } else { call.b };
   result[call.c] = apply_operation(&call.instruction.op, a, b);
   result
}

/// execute a serie of instruction
fn execute(mut register: Register, instruction_pointer_register: usize, calls: &[Call]) -> Register
{
   let mut instruction_pointer = 0;

   while instruction_pointer < calls.len()
   {
      register[instruction_pointer_register] = instruction_pointer;
      register = apply_call(&calls[instruction_pointer], &register);
      instruction_pointer = register[instruction_pointer_register] + 1;
      //println!("target:{} ip:{} register:{:?}", register[0], instruction_pointer, register);
   }

   register
}

//-----------------------------------------------------------------------------
// TASK2

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
         _ => panic!("unknown instruction")
      }
   }
}

/// after a manual decompilation, my target program is the sum of divisors of 10_551_355
fn sum_of_divisors(x5: usize) -> usize
{
   let mut x0 = 0;

   for x1 in 1..=x5
   {
      if x5 % x1 == 0
      {
         x0 += x1;
      }
   }

   x0
}

//-----------------------------------------------------------------------------
// MAIN

fn main()
{
   let input_path = "./data/input.txt";
   let (ip, calls) = input_data(input_path);
   display(ip, &calls);

   // task1
   let initial_register = [0; 6];
   let register = execute(initial_register, ip, &calls);
   println!("register 0 : {}", register[0]);

   // task2
   /*let initial_register = [1, 0, 0, 0, 0, 0];
   let register = execute(initial_register, ip, &calls);
   println!("register 0 : {}", register[0]);*/
   let register5 = 10_551_355;
   let register0 = sum_of_divisors(register5); // found via manual decompilation of the pseudo-assembly
   println!("register 0 : {}", register0);
}
