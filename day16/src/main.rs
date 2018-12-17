#[macro_use]
extern crate scan_fmt;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

//-----------------------------------------------------------------------------
// INSTRUCTION

type Register = [usize; 4];

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
   opcode: usize,
   a: usize,
   b: usize,
   c: usize
}

struct TestCase
{
   before: Register,
   after: Register,
   call: Call
}

//-----------------------------------------------

/// applies an operation to two inputs
fn apply_operation(op: &Operation, a: usize, b: usize) -> usize
{
   match op
   {
      Operation::Add => a + b,
      Operation::Mul => a * b,
      Operation::And => a & b,
      Operation::Or => a | b,
      Operation::Set => a,
      Operation::Greater if a > b => 1,
      Operation::Greater => 0,
      Operation::Equal if a == b => 1,
      Operation::Equal => 0
   }
}

/// applies an instruction to a register
fn apply_instruction(instr: &Instruction, a: usize, b: usize, c: usize, register: &Register) -> Register
{
   let mut result = *register;
   let a = if instr.a_is_register { register[a] } else { a };
   let b = if instr.b_is_register { register[b] } else { b };
   result[c] = apply_operation(&instr.op, a, b);
   result
}

//-----------------------------------------------

/// builds an instruction
fn make_instruction(name: &str, a_is_register: bool, b_is_register: bool, op: Operation) -> Instruction
{
   Instruction { name: name.to_string(), a_is_register, b_is_register, op }
}

/// produces a list of all instructions available
fn list_instructions() -> Vec<Instruction>
{
   let addr = make_instruction("addr", true, true, Operation::Add);
   let addi = make_instruction("addi", true, false, Operation::Add);
   let mulr = make_instruction("mulr", true, true, Operation::Mul);
   let muli = make_instruction("muli", true, false, Operation::Mul);
   let banr = make_instruction("banr", true, true, Operation::And);
   let bani = make_instruction("bani", true, false, Operation::And);
   let borr = make_instruction("borr", true, true, Operation::Or);
   let bori = make_instruction("bori", true, false, Operation::Or);
   let setr = make_instruction("setr", true, true, Operation::Set);
   let seti = make_instruction("seti", false, false, Operation::Set);
   let gtir = make_instruction("gtir", false, true, Operation::Greater);
   let gtri = make_instruction("gtri", true, false, Operation::Greater);
   let gtrr = make_instruction("gtrr", true, true, Operation::Greater);
   let eqir = make_instruction("eqir", false, true, Operation::Equal);
   let eqri = make_instruction("eqri", true, false, Operation::Equal);
   let eqrr = make_instruction("eqrr", true, true, Operation::Equal);
   vec![addr, addi, mulr, muli, banr, bani, borr, bori, setr, seti, gtir, gtri, gtrr, eqir, eqri, eqrr]
}

//-----------------------------------------------------------------------------
// INPUT

fn parse_register(line: &str) -> Register
{
   let (_, r0, r1, r2, r3) = scan_fmt!(&line, "{} [{}, {}, {}, {}]", String, usize, usize, usize, usize);
   [r0.unwrap(), r1.unwrap(), r2.unwrap(), r3.unwrap()]
}

fn parse_call(line: &str) -> Call
{
   let (code, a, b, c) = scan_fmt!(&line, "{} {} {} {}", usize, usize, usize, usize);
   Call { opcode: code.unwrap(), a: a.unwrap(), b: b.unwrap(), c: c.unwrap() }
}

/// parses a file and returns (testcases, program)
fn input_data(path: &str) -> (Vec<TestCase>, Register, Vec<Call>)
{
   let file = File::open(path).expect("Failed to open input file.");
   let mut lines =
      BufReader::new(file).lines().map(|line_option| line_option.expect("Failed to load a line."));

   // parses test cases
   let mut line = lines.next().unwrap();
   let mut testcases = Vec::new();
   while line.starts_with("Before")
   {
      let before = parse_register(&line);
      let call = parse_call(&lines.next().unwrap());
      let after = parse_register(&lines.next().unwrap());
      testcases.push(TestCase { before, after, call });
      lines.next();
      line = lines.next().unwrap();
   }

   // parses program
   lines.next();
   lines.next();
   let (r0, r1, r2, r3) = scan_fmt!(&lines.next().unwrap(), "{} {} {} {}", usize, usize, usize, usize);
   let initial_register = [r0.unwrap(), r1.unwrap(), r2.unwrap(), r3.unwrap()];
   let program: Vec<Call> = lines.map(|line| parse_call(&line)).collect();

   (testcases, initial_register, program)
}

//-----------------------------------------------------------------------------
// TASK1

/// returns true if the instruction matches the behaviours of the code in this testcase
fn match_instruction(TestCase { before, after, call }: &TestCase, instr: &Instruction) -> bool
{
   let result = &apply_instruction(&instr, call.a, call.b, call.c, before);
   result == after
}

/// given a test case and an array of instructions, returns the instructions that match the test case
fn possible_instructions<'a>(test: &TestCase, instructions: &'a [Instruction]) -> HashSet<&'a Instruction>
{
   instructions.iter().filter(|instr| match_instruction(test, instr)).collect()
}

/// returns the number of test cases that match three or more instructions
fn task1(test_cases: &[TestCase], instructions: &[Instruction]) -> usize
{
   test_cases.iter()
             .map(|test| possible_instructions(test, instructions))
             .filter(|instr| instr.len() >= 3)
             //.inspect(|instr| instr.iter().for_each(|i| println!("{}", i.name)))
             .count()
}

//-----------------------------------------------------------------------------
// TASK2

/// gets the sintrcution from a singleton
fn get_single_instruction<'a>(set: &HashSet<&'a Instruction>) -> Option<&'a Instruction>
{
   if set.len() == 1
   {
      let instruction = *set.iter().next().unwrap();
      Some(instruction)
   }
   else
   {
      None
   }
}

fn deduce_codes(test_cases: &[TestCase], instructions: &[Instruction]) -> Vec<Instruction>
{
   let instructions_number = instructions.len();

   let mut result = vec![HashSet::from_iter(instructions); instructions_number];
   for test in test_cases
   {
      let code = test.call.opcode;
      let candidates = possible_instructions(test, instructions);
      result[code] = result[code].intersection(&candidates).cloned().collect();
   }

   let mut unsolved: Vec<usize> = (0..instructions_number).collect();
   while !unsolved.is_empty()
   {
      let mut new_unsolved = Vec::new();
      for &code in &unsolved
      {
         if let Some(instruction) = get_single_instruction(&result[code])
         {
            println!("{}: {:02}", instruction.name, code);
            // singleton, we can safely remove the instruction from every other unsolved
            for &code2 in &unsolved
            {
               if code2 != code
               {
                  result[code2].remove(instruction);
               }
            }
         }
         else
         {
            // several possible value, we need to wait
            new_unsolved.push(code);
         }
      }
      unsolved = new_unsolved;
   }

   result.iter()
         .map(|instructions| get_single_instruction(&instructions).expect("Several possibilities"))
         .cloned()
         .collect()
}

/// execute a serie of instruction
fn execute(program: &[Call], initial_register: &Register, instructions: &[Instruction]) -> Register
{
   let mut register = *initial_register;

   for call in program
   {
      register = apply_instruction(&instructions[call.opcode], call.a, call.b, call.c, &register)
   }

   register
}

//-----------------------------------------------------------------------------
// MAIN

fn main()
{
   let input_path = "./data/input.txt";
   let (testcases, initial_register, program) = input_data(input_path);
   let instructions = list_instructions();

   // task1
   let nb_3more = task1(&testcases, &instructions);
   println!("number of >=3 tests : {}", nb_3more);

   // task2
   let instructions = deduce_codes(&testcases, &instructions);
   let final_register = execute(&program, &initial_register, &instructions);
   println!("register 0 : {}", final_register[0]);
}
