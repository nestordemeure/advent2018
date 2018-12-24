#[macro_use]
extern crate scan_fmt;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

//-----------------------------------------------------------------------------
// TYPE

#[derive(Copy, Clone, PartialEq, Debug)]
enum Affiliation
{
   System,
   Infection
}

#[derive(Clone)]
struct Group
{
   id: usize,
   nb_units: usize,
   hit_points: usize,
   attack_damage: usize,
   attack_type: String,
   initiative: usize,
   weaknesses: HashSet<String>,
   immunities: HashSet<String>,
   affiliation: Affiliation
}

//-----------------------------------------------------------------------------
// INPUT

/// parses a comma separated line
fn parse_csv_line(line: &str) -> HashSet<String>
{
   line.split(',').map(|s| s.trim().to_string()).collect()
}

/// returns (weaknesses, immunities)
fn parse_particularities(line: &str) -> (HashSet<String>, HashSet<String>)
{
   if line.contains(';')
   {
      let (type1, list1, _, list2) =
         scan_fmt!(line, "{} to {/[a-z, ]+/}; {} to {/[a-z, ]+/}", String, String, String, String);
      let list1 = parse_csv_line(&list1.unwrap());
      let list2 = parse_csv_line(&list2.unwrap());
      if type1.unwrap() == "weak"
      {
         (list1, list2)
      }
      else
      {
         (list2, list1)
      }
   }
   else
   {
      let (type1, list1) = scan_fmt!(line, "{} to {/[a-z, ]+/}", String, String);
      let list1 = parse_csv_line(&list1.unwrap());
      if type1.unwrap() == "weak"
      {
         (list1, HashSet::new())
      }
      else
      {
         (HashSet::new(), list1)
      }
   }
}

/// parses a line
fn parse_group(line: &str, affiliation: Affiliation, id: &mut usize) -> Group
{
   *id += 1;
   if line.contains('(')
   {
      let (nb_units, hit_points, particularities, attack_damage, attack_type, initiative) = scan_fmt!(line,
                "{} units each with {} hit points ({/[a-z,; ]+/}) with an attack that does {} {} damage at initiative {}",
                usize,
                usize,
                String,
                usize,
                String,
                usize);
      let (weaknesses, immunities) = parse_particularities(&particularities.unwrap().trim());
      Group { id: *id,
              nb_units: nb_units.unwrap(),
              hit_points: hit_points.unwrap(),
              attack_damage: attack_damage.unwrap(),
              attack_type: attack_type.unwrap(),
              initiative: initiative.unwrap(),
              weaknesses: weaknesses,
              immunities: immunities,
              affiliation: affiliation }
   }
   else
   {
      let (nb_units, hit_points, attack_damage, attack_type, initiative) = scan_fmt!(line,
                "{} units each with {} hit points with an attack that does {} {} damage at initiative {}",
                usize,
                usize,
                usize,
                String,
                usize);
      Group { id: *id,
              nb_units: nb_units.unwrap(),
              hit_points: hit_points.unwrap(),
              attack_damage: attack_damage.unwrap(),
              attack_type: attack_type.unwrap(),
              initiative: initiative.unwrap(),
              weaknesses: HashSet::new(),
              immunities: HashSet::new(),
              affiliation: affiliation }
   }
}

/// turns a file into two groups : (immune_system, infection)
fn input_data(path: &str) -> Vec<Group>
{
   let file = File::open(path).expect("Failed to open input file.");
   let mut lines =
      BufReader::new(file).lines().map(|line_option| line_option.expect("Failed to load a line."));
   // immune system
   let mut id = 0;
   lines.next();
   let mut system: Vec<Group> = lines.by_ref()
                                     .take_while(|line| !line.is_empty())
                                     .map(|line| parse_group(&line, Affiliation::System, &mut id))
                                     .collect();
   // infection
   id = 0;
   lines.next();
   let infection: Vec<Group> =
      lines.map(|line| parse_group(&line, Affiliation::Infection, &mut id)).collect();
   system.extend(infection);
   system
}

//-----------------------------------------------------------------------------
// SIMULATION

/// returns the effective power of a group
fn effective_power(group: &Group) -> usize
{
   group.nb_units * group.attack_damage
}

/// returns the damage the attackers would deal to the target
fn compute_damage(attackers: &Group, target: &Group) -> usize
{
   let base_damage = effective_power(&attackers);
   if target.weaknesses.contains(&attackers.attack_type)
   {
      base_damage * 2
   }
   else if target.immunities.contains(&attackers.attack_type)
   {
      0
   }
   else
   {
      base_damage
   }
}

/// reduces the number of units in a group as a function of the damages it took
fn take_damage(target: &mut Group, damage: usize) -> usize
{
   let unit_lost = damage / target.hit_points;
   if unit_lost >= target.nb_units
   {
      target.nb_units = 0;
   }
   else
   {
      target.nb_units -= unit_lost;
   }
   unit_lost
}

/// runs a single turn of combat
fn simulate_turn(mut groups: Vec<Group>) -> Option<Vec<Group>>
{
   let nb_groups = groups.len();

   // TARGET SELECTION
   // defines a selection order
   let mut attack_order: Vec<_> = (0..nb_groups).collect();
   attack_order.sort_unstable_by_key(|&i| (effective_power(&groups[i]), &groups[i].initiative));
   attack_order.reverse();
   // finds a target per attacker
   let mut attacked: Vec<bool> = vec![false; nb_groups];
   let mut targets: Vec<Option<usize>> = vec![None; nb_groups];
   for &group_id in &attack_order
   {
      let group = &groups[group_id];
      let target = groups.iter()
                         .enumerate()
                         .filter(|&(i, g)| (g.affiliation != group.affiliation) && (!attacked[i]))
                         .max_by_key(|(_, g)| (compute_damage(group, g), effective_power(g), g.initiative));
      if let Some((target_id, target)) = target
      {
         if compute_damage(group, target) > 0
         {
            targets[group_id] = Some(target_id);
            attacked[target_id] = true;
         }
      }
   }

   // ATTACK
   let mut total_death = 0;
   // defines an attack order
   attack_order.sort_unstable_by_key(|&i| &groups[i].initiative);
   attack_order.reverse();
   // attack group per group
   for group_id in attack_order
   {
      if let Some(target_id) = targets[group_id]
      {
         let damage = compute_damage(&groups[group_id], &groups[target_id]);
         total_death += take_damage(&mut groups[target_id], damage);
      }
   }

   // removes all dead units
   let groups = groups.into_iter().filter(|g| g.nb_units > 0).collect();
   if total_death == 0
   {
      // tie
      None
   }
   else
   {
      Some(groups)
   }
}

/// simulate the system until a group is eradicated
/// returns Ok if the immune system won
fn simulate(groups: &[Group]) -> Result<usize, usize>
{
   let mut groups = groups.to_vec();

   loop
   {
      match simulate_turn(groups)
      {
         None => return Err(0), // in case of tie we just say that the immune system lost
         Some(new_groups) =>
         {
            groups = new_groups;

            let has_system = groups.iter().any(|g| g.affiliation == Affiliation::System);
            if !has_system
            {
               let nb_units = total_units_alive(&groups);
               return Err(nb_units);
            }

            let has_infection = groups.iter().any(|g| g.affiliation == Affiliation::Infection);
            if !has_infection
            {
               let nb_units = total_units_alive(&groups);
               return Ok(nb_units);
            }
         }
      }
   }
}

//-----------------------------------------------------------------------------
// BOOST

/// boost all the groups in the immune system with the given value
fn boost_groups(groups: &[Group], boost: usize) -> Vec<Group>
{
   fn boost_group(group: Group, boost: usize) -> Group
   {
      match group.affiliation
      {
         Affiliation::Infection => group,
         Affiliation::System => Group { attack_damage: group.attack_damage + boost, ..group }
      }
   }

   groups.iter().cloned().map(|g| boost_group(g.clone(), boost)).collect()
}

fn find_minimum_boost(groups: &[Group]) -> usize
{
   let mut boost = 0;

   loop
   {
      println!("testing boost {}", boost);
      let groups = boost_groups(&groups, boost);
      match simulate(&groups)
      {
         Ok(units_left) => return units_left,
         Err(_) => boost += 1
      }
   }
}

//-----------------------------------------------------------------------------
// MAIN

fn total_units_alive(groups: &[Group]) -> usize
{
   groups.iter().map(|g| g.nb_units).sum()
}

fn main()
{
   let input_path = "./data/input.txt";
   let groups = input_data(input_path);

   // task1
   let units_left_alive = simulate(&groups);
   println!("nb units alive : {:?}", units_left_alive);

   // task2
   let units_left_alive = find_minimum_boost(&groups);
   println!("units left after minimum boost : {}", units_left_alive);
}
