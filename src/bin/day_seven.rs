use advent_of_code_2021::util ;
use rayon::prelude::* ;
use std::convert::TryFrom;

fn sum_from_zero( n: i64) -> i64 {
  (0 .. n+1).fold(0,|a,b| a + b)
}

fn calc_fuel_rev(pos: &u64, list: &Vec<u64>, res: &mut u64) {
  let mut tmp: i64 = 0 ;
  for (index, value) in list.iter().enumerate() {
    tmp += (*value as i64 ) * ( sum_from_zero( ( *pos as i64 - index as i64 ).abs()) ) ; 
    if (tmp as u64) > *res { // no need to continue
      break; 
    }
  }
  if (tmp as u64 ) < *res {
    *res = tmp as u64 ;
  }
}
fn optimize_movement_rev( list: &Vec<u64>) -> u64 {
  let mut res: u64 = 0 ;
  //on trie la liste
  let mut new_list: Vec<u64> = list.clone();
  new_list.par_sort_unstable();
  let len = new_list.len() ; 
  let mut count: Vec<u64> = vec![0u64 ; ( new_list[ (len - 1)  ] + 1 ) as usize ];
  for element in new_list.iter() {
    count[*element as usize] += 1 ;
  }
  let mut res = u64::MAX ;
  for pos in 0..=len {
    calc_fuel_rev(&(pos as u64) ,&count,&mut res) ; 
println!("pos {} value : {}",pos,res) ;
  }
  res 
}


//return the fuel cost for a given position
fn calc_fuel(pos: &u64, list: &Vec<u64>, res: &mut u64) {
  let mut tmp: i64 = 0 ;
  for (index, value) in list.iter().enumerate() {
    tmp += (*value as i64 ) * ( ( *pos as i64 - index as i64 ).abs() ) ; 
    if (tmp as u64) > *res { // no need to continue
      break; 
    }
  }
  if (tmp as u64 ) < *res {
    *res = tmp as u64 ;
  }
}

fn optimize_movement( list: &Vec<u64>) -> u64 {
  //on trie la liste
  let mut new_list: Vec<u64> = list.clone();
  new_list.par_sort_unstable();
  let len = new_list.len() ; 
  let mut count: Vec<u64> = vec![0u64 ; ( new_list[ (len - 1)  ] + 1 ) as usize ];
  for element in new_list.iter() {
    count[*element as usize] += 1 ;
  }
  let mut res = u64::MAX ;
  for pos in 0..=len {
    calc_fuel(&(pos as u64) ,&count,&mut res) ; 
println!("pos {} value : {}",pos,res) ;
  }
  res 
}


//take day seven input and save it into a Vector
fn read_input (path: &str ) -> Vec<u64> {
  let mut lines = util::read_lines(path).unwrap() ;
  let mut mat: Vec<u64> = Vec::new() ;
  let mut list_string: Vec<&str> = Vec::new() ;
  if let Ok(element) = lines.next().unwrap() {
    list_string = element.split(',').collect() ;
    for num in list_string.iter() {
      mat.push(num.to_string().parse::<u64>().unwrap()) ;
    }
  }  
  mat
}


fn main() {
  let mat = read_input("../input/input_day_seven.txt");
  let res = optimize_movement(&mat) ; 
  let res2 = optimize_movement_rev(&mat) ; 
  println!("d7 p1 : {}", res);
  println!("d7 p2 : {}", res2);

}
