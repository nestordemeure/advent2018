#![feature(slice_patterns)]

//-----------------------------------------------------------------------------
// INPUT

fn input_data(path: &str) -> Vec<usize>
{
   std::fs::read_to_string(path).unwrap().trim_end().split(' ').map(|line| line.parse().unwrap()).collect()
}

//-----------------------------------------------------------------------------
// PARSE

struct Tree
{
   childrens: Vec<Tree>,
   metadata: Vec<usize>
}

/// extracts the description from a tree
fn parse_tree_description(data: &[usize]) -> (usize, usize, &[usize])
{
   match data
   {
      [nb_childrens, nb_metadata, tail..] => (*nb_childrens, *nb_metadata, tail),
      _ => panic!("unable to parse tree description")
   }
}

/// parses a tree of format [nb_children nb_metadata [childrens] [metadata]]
fn parse_tree(data: &[usize]) -> (Tree, &[usize])
{
   // parse description
   let (nb_childrens, nb_metadata, leftover_data) = parse_tree_description(data);
   let mut data = leftover_data;

   // parse childrens
   let mut childrens = Vec::new();
   for _ in 0..nb_childrens
   {
      let (child, leftover_data) = parse_tree(data);
      childrens.push(child);
      data = leftover_data;
   }

   // parse metadata
   let metadata = data[..nb_metadata].to_vec();
   data = &data[nb_metadata..];

   let tree = Tree { childrens: childrens, metadata: metadata };
   (tree, data)
}

//-----------------------------------------------------------------------------
// TASKS

/// sums all the metadata in the tree and its childrens
fn sum_metadata(tree: &Tree) -> usize
{
   let sum_root: usize = tree.metadata.iter().sum();
   let sum_childrens: usize = tree.childrens.iter().map(sum_metadata).sum();
   sum_root + sum_childrens
}

/// computes the value of a tree
/// the sum of its metadata if it is empty or the sum of the value of the node corresponding to a metadata
fn sum_value(tree: &Tree) -> usize
{
   if tree.childrens.is_empty()
   {
      tree.metadata.iter().sum()
   }
   else
   {
      tree.metadata.iter().filter_map(|&i| tree.childrens.get(i - 1)).map(sum_value).sum()
   }
}

//-----------------------------------------------------------------------------
// MAIN

fn main()
{
   let input_path = "./data/input.txt";
   let raw_data = input_data(input_path);
   let (tree, _) = parse_tree(&raw_data);

   // task1
   let sum = sum_metadata(&tree);
   println!("sum : {}", sum);

   // task2
   let val = sum_value(&tree);
   println!("value : {}", val);
}
