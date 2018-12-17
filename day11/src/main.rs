const GRID_SIZE: usize = 300;

//-----------------------------------------------------------------------------
// TASK1

/// takes a number and returns the digit of the hundreds
fn keep_hundreds(n: i32) -> i32
{
   (n % 1000) / 100
}

/// computes the power level of a cell with given coordinates
fn power_level(x: i32, y: i32, grid_serial_number: i32) -> i32
{
   let rack_id = x + 10;
   let mut power = rack_id * y;
   power += grid_serial_number;
   power *= rack_id;
   power = keep_hundreds(power);
   power -= 5;
   power
}

/// fills a grids with power levels
fn make_grid(grid_serial_number: i32) -> [[i32; GRID_SIZE]; GRID_SIZE]
{
   let mut grid = [[0; GRID_SIZE]; GRID_SIZE];

   for i in 0..GRID_SIZE
   {
      for j in 0..GRID_SIZE
      {
         let x = (j + 1) as i32;
         let y = (i + 1) as i32;
         grid[i][j] = power_level(x, y, grid_serial_number);
      }
   }

   grid
}

/// computes the power of a size*size square
fn power_of_square(i: usize, j: usize, size: usize, grid: &[[i32; GRID_SIZE]; GRID_SIZE]) -> i32
{
   let mut result = 0;

   for row in &grid[i..(i + size)]
   {
      for cell in &row[j..(j + size)]
      {
         result += cell;
      }
   }

   result
}

/// computes the cell at the begining of the 3x3 square fo maximum power
fn max_3square(grid: &[[i32; GRID_SIZE]; GRID_SIZE]) -> (usize, usize, i32)
{
   let mut best_i = 0;
   let mut best_j = 0;
   let mut best_power = power_of_square(best_i, best_j, 3, &grid);

   for i in 0..(GRID_SIZE - 3)
   {
      for j in 0..(GRID_SIZE - 3)
      {
         let power = power_of_square(i, j, 3, &grid);
         if power > best_power
         {
            best_i = i;
            best_j = j;
            best_power = power;
         }
      }
   }

   (best_j + 1, best_i + 1, best_power)
}

//-----------------------------------------------------------------------------
// TASK2

/// produces a grid in which integral[i][j] = sum grid[k<=i][t<=j]
/// alows the computation of the sum of a square in o(1)
fn integrate_grid(grid: &[[i32; GRID_SIZE]; GRID_SIZE]) -> [[i32; GRID_SIZE]; GRID_SIZE]
{
   let mut integral = *grid;

   // integrate on j
   for i in 0..(GRID_SIZE - 1)
   {
      for j in 0..(GRID_SIZE - 1)
      {
         integral[i][j + 1] += integral[i][j];
      }
   }

   // integrate on i
   for j in 0..(GRID_SIZE - 1)
   {
      for i in 0..(GRID_SIZE - 1)
      {
         integral[i + 1][j] += integral[i][j];
      }
   }

   integral
}

/// computes the power of a size*size square using an integral grid to make it o(1)
fn fast_power_of_square(i: usize, j: usize, size: usize, integral: &[[i32; GRID_SIZE]; GRID_SIZE]) -> i32
{
   let imax = (i + size) - 1;
   let jmax = (j + size) - 1;

   match (i, j)
   {
      (0, 0) => integral[imax][jmax],
      (0, _) => integral[imax][jmax] - integral[imax][j - 1],
      (_, 0) => integral[imax][jmax] - integral[i - 1][jmax],
      _ => integral[imax][jmax] - integral[imax][j - 1] - integral[i - 1][jmax] + integral[i - 1][j - 1]
   }
}

/// computes the subsquare with the maximum total power
/// uses an integral grid to make the computation of the sum of a square a o(1) operation
fn max_square(grid: &[[i32; GRID_SIZE]; GRID_SIZE]) -> (usize, usize, usize, i32)
{
   let integral = integrate_grid(grid);
   let mut best_i = 0;
   let mut best_j = 0;
   let mut best_size = 1;
   let mut best_power = fast_power_of_square(best_i, best_j, best_size, &integral);

   for size in 1..=GRID_SIZE
   {
      for i in 0..(GRID_SIZE - size)
      {
         for j in 0..(GRID_SIZE - size)
         {
            let power = fast_power_of_square(i, j, size, &integral);
            if power > best_power
            {
               best_i = i;
               best_j = j;
               best_size = size;
               best_power = power;
            }
         }
      }
   }

   (best_j + 1, best_i + 1, best_size, best_power)
}

//-----------------------------------------------------------------------------
// MAIN

fn main()
{
   let grid_serial_number = 5034;

   // task1
   let grid = make_grid(grid_serial_number);
   let (x, y, power) = max_3square(&grid);
   println!("best 3 square : ({},{}) of power {}", x, y, power);

   // task2
   let (x, y, size, power) = max_square(&grid);
   println!("best square : ({},{}) of size {} and power {}", x, y, size, power);
}
