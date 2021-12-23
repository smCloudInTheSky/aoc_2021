use advent_of_code_2021::util ;
use rayon::prelude::* ;

// [0 1 2 3 4 5 6 7 8 ] 

//fonction qui fait une itération
fn fibo(list: &mut Vec<u8> ) {
  list.par_iter_mut().for_each(|item| {
    if item == &0 {
      *item = 6 ;
    } else { 
      *item -= 1 ;
    }
  }
  );
  let newborn: Vec<u8> = list.clone();
  newborn.iter().for_each(|element| {
    if element == &0 {
    list.push(8)
    }}) ; 
}

fn calc_fibo_smart(list: Vec<u8>) -> Vec<u64> {
  let mut res: Vec<u64> = vec![0u64 ; 9] ; 
  list.iter().for_each(|i| {
    if *i == 0u8 {
      res[0] += 1;
    } else if *i == 1u8 {
      res[1] += 1;
    } else if *i == 2u8 {
      res[2] += 1;
    } else if *i == 3u8 {
      res[3] += 1;
    } else if *i == 4u8 {
      res[4] += 1;
    } else if *i == 5u8 {
      res[5] += 1;
    } else if *i == 6u8 {
      res[6] += 1;
    } else if *i == 7u8 {
      res[7] += 1;
    } else if *i == 8u8 {
      res[8] += 1;
    }
  });
  res 
}

// prend en input une liste avec le nombre de poisson à chaque niveau
// sauvegarde le 0 dans une vartiable temporaire et shift tout le monde
// ajoute à 6 le nombre de poisson à 0 puis crée autant de poisson en 8
fn fibo_smart(list: &mut Vec<u64>) {
  let newborn = list[0];
  for i in 1..list.len() {
    list[i - 1] = list[i] ;
  }
  list[6] = list[6] + newborn; 
  list[8] = newborn;

}
 
// fonction qui charge le fichier texte et renvoit l'état initial
fn read_input (path: &str ) -> Vec<u8> {
  let mut lines = util::read_lines(path).unwrap() ;
  let mut mat: Vec<u8> = Vec::new() ;
  let mut list_string: Vec<&str> = Vec::new() ;
  if let Ok(element) = lines.next().unwrap() {
    list_string = element.split(',').collect() ;
    for num in list_string.iter() {
      mat.push(num.to_string().parse::<u8>().unwrap()) ;
    }
  }  
  mat
}



fn main () {
  let mat = read_input("../input/input_day_six.txt");
  let mut day: u16 = 0 ;
  let mut mat_fib = mat.clone() ; 
//  // if you have 16Gb and time to wait -> easiest solution but inapropriate
//  while day < 256 {
//    if day == 80 {
//    println!("p1 : {}",mat_fib.len()); 
//    }
//    fibo(&mut mat_fib);
//    day += 1 ;
//  } 
//  println!("d6 p2 : {}",mat_fib.len());
  let mut mat_smart = calc_fibo_smart(mat.clone());
  let mut day_smart: u16 = 0 ;
  while day_smart < 256 {
    if day == 80 {
      println!("p1 : {}",mat_smart.par_iter().sum::<u64>()); 
    }
    println!("day : {}",day_smart);
    fibo_smart(&mut mat_smart);
    day_smart += 1 ;
  } 
  println!("d6 p2 : {}",mat_smart.par_iter().sum::<u64>());
}
