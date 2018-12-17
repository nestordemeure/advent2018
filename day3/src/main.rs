#[macro_use]
extern crate scan_fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

const TOTAL_WIDTH: usize = 1000;
type Canvas = [[i32; TOTAL_WIDTH]; TOTAL_WIDTH];

//-----------------------------------------------------------------------------
// INPUT

struct Rectangle
{
   id: usize,
   x: usize,
   y: usize,
   width: usize,
   height: usize
}

/// parses a line to produce a rectangle
fn make_rectangle(line: &str) -> Rectangle
{
   let (id, x, y, w, h) = scan_fmt!(line, "#{} @ {},{}: {}x{}", usize, usize, usize, usize, usize);
   Rectangle { id: id.unwrap(), x: x.unwrap(), y: y.unwrap(), width: w.unwrap(), height: h.unwrap() }
}

/// reads a file line by line and converts each line to a rectangle
fn read_inputs(path: &str) -> Vec<Rectangle>
{
   let file = File::open(path).expect("Failed to open input file.");
   BufReader::new(file).lines()
                       .map(|line_option| line_option.expect("Failed to load a line."))
                       .map(|line| make_rectangle(&line))
                       .collect()
}

//-----------------------------------------------------------------------------
// TASK1

/// add a single rectangle to a given canvas
fn add_rectangle_to_canvas(canvas: &mut Canvas, rectangle: &Rectangle)
{
   for i in rectangle.x..(rectangle.x + rectangle.width)
   {
      for j in rectangle.y..(rectangle.y + rectangle.height)
      {
         canvas[i][j] += 1;
      }
   }
}

/// fills the canvas
fn fill_canvas(rectangles: &[Rectangle]) -> Canvas
{
   let mut canvas = [[0; TOTAL_WIDTH]; TOTAL_WIDTH];

   for rectangle in rectangles
   {
      add_rectangle_to_canvas(&mut canvas, rectangle)
   }

   canvas
}

/// count the number of overlaps on the canvas
fn count_overlaps(canvas: &Canvas) -> usize
{
   // counts non empty cells
   let mut total = 0;

   for row in canvas.iter()
   {
      for &cell in row.iter()
      {
         if cell >= 2
         {
            total += 1;
         }
      }
   }

   total
}

//-----------------------------------------------------------------------------
// TASK2

/// returns true if a rectangle does not overlap with any other rectangle
/// using a canvas on which we printed each rectangle
fn non_overlapping(canvas: &Canvas, rectangle: &Rectangle) -> bool
{
   for i in rectangle.x..(rectangle.x + rectangle.width)
   {
      for j in rectangle.y..(rectangle.y + rectangle.height)
      {
         if canvas[i][j] > 1
         {
            return false;
         }
      }
   }

   true
}

/// finds a rectangle that overlaps with no other rectangle
fn task2(canvas: &Canvas, rectangles: &[Rectangle]) -> usize
{
   rectangles.iter()
             .find(|rect| non_overlapping(canvas, rect))
             .expect("Could not find a non overlapping rectangle.")
             .id
}

//-----------------------------------------------------------------------------

fn main()
{
   let input_path = "./data/input.txt";
   let rectangles = read_inputs(input_path);

   // task1 : computes the number of cells with overlapping rectangles
   let canvas = fill_canvas(&rectangles);
   let overlap = count_overlaps(&canvas);
   println!("overlap : {}", overlap);

   // task2 : find the ID of a non overlapping rectangle
   let id_nonoverlapping = task2(&canvas, &rectangles);
   println!("id of a no-noverlapping rectangle : {}", id_nonoverlapping);
}
