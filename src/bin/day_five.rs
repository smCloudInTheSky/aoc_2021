use advent_of_code_2021::util;
use std::convert::TryFrom;

fn trace_matrix_w_diag(matrix: Vec<Vec<usize>>, coord1:String , coord2:String) -> Vec<Vec<usize>> {
 let start: Vec<&str> = coord1.split(',').collect();
 let end: Vec<&str> = coord2.split(',').collect();
 let x1: usize = start[0].to_string().parse().unwrap();
 let x2: usize = end[0].to_string().parse().unwrap();
 let y1: usize = start[1].to_string().parse().unwrap();
 let y2: usize = end[1].to_string().parse().unwrap();
 let mut mat = matrix.clone();
 if x1 == x2 {
   if y1 < y2 {
   for i in y1..=y2 {
     mat[x1][i] = mat[x1][i] +1 ;
   }
   } else {
   for i in y2..=y1 {
     mat[x1][i] = mat[x1][i] +1 ;
   }
   }
 }
 else if y1 == y2  {
   if x1 < x2 {
   for i in x1..=x2 {
     mat[i][y1] = mat[i][y1] +1 ;
   }
   } else {
   for i in x2..=x1 {
     mat[i][y1] = mat[i][y1] +1 ;
   }
   }
 } else {
 // on s'occupe du cas diagonale
 // méthode : calcul du vecteur -> x2 - x1 , y2 - y1 
 // ensuite boucle while pour aller du point 1 au point 2 
 // action classique
 // on aurait pu faire ça dès le début en fait... refacto à l'occasion
   let mut vector: Vec<isize> = Vec::new();
   vector.push(isize::try_from(x2).unwrap() - isize::try_from(x1).unwrap()) ; 
   vector.push(isize::try_from(y2).unwrap() - isize::try_from(y1).unwrap()) ;
   let interval_x: f64 = vector[0] as f64 / ( vector[0].abs() + 1 ) as f64 ;
   let interval_y: f64 = vector[1] as f64 / ( vector[1].abs() + 1 ) as f64 ;
   let mut x_prev: f64 = x2 as f64 ;
   let mut y_prev: f64 = y2 as f64 ;
   if vector[0].abs() == vector[1].abs() { //égalité donc diagonale à 45°
     for i in 0..=(vector[0].abs() + 1 ) {
        let xi = x1 as f64 + (interval_x * i as f64).round()  ;
        let yi = y1 as f64 + (interval_y * i as f64).round()  ;
        if xi != x_prev && yi != y_prev {
          mat[xi as usize ][yi as usize ] = mat[xi as usize ][yi as usize ] + 1 ;  
          x_prev = xi ;
          y_prev = yi ; 
        }
     }
   }
 } 
 mat
}

fn trace_matrix(matrix: Vec<Vec<usize>>, coord1:String , coord2:String) -> Vec<Vec<usize>> {
 let start: Vec<&str> = coord1.split(',').collect();
 let end: Vec<&str> = coord2.split(',').collect();
 let x1: usize = start[0].to_string().parse().unwrap();
 let x2: usize = end[0].to_string().parse().unwrap();
 let y1: usize = start[1].to_string().parse().unwrap();
 let y2: usize = end[1].to_string().parse().unwrap();
 let mut mat = matrix.clone();
 if x1 == x2 {
   if y1 < y2 {
   for i in y1..=y2 {
     mat[x1][i] = mat[x1][i] +1 ;
   }
   } else {
   for i in y2..=y1 {
     mat[x1][i] = mat[x1][i] +1 ;
   }
   }
 }
 if y1 == y2  {
   if x1 < x2 {
   for i in x1..=x2 {
     mat[i][y1] = mat[i][y1] +1 ;
   }
   } else {
   for i in x2..=x1 {
     mat[i][y1] = mat[i][y1] +1 ;
   }
   }
 }
 mat
}

fn load_input(path: &str, matrix: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
  let lines = util::read_lines(path).unwrap(); 
  let mut mat_res = matrix.clone();
  for line in lines {
    if let Ok(element) = line {
      let mut coord: Vec<String> = Vec::new() ;
      for item in element.split_terminator("->"){
        coord.push(item.trim().to_string());
      }
      mat_res = trace_matrix(mat_res, coord[0].clone(), coord[1].clone());
    }
  }
  mat_res
}

fn load_input_w_diag(path: &str, matrix: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
  let lines = util::read_lines(path).unwrap(); 
  let mut mat_res = matrix.clone();
  for line in lines {
    if let Ok(element) = line {
      let mut coord: Vec<String> = Vec::new() ;
      for item in element.split_terminator("->"){
        coord.push(item.trim().to_string());
      }
      mat_res = trace_matrix_w_diag(mat_res, coord[0].clone(), coord[1].clone());
    }
  }
  mat_res
}

fn main() {
  let matrix: Vec<Vec<usize>> = vec![ vec![0usize ; 1000] ; 1000];
  let res = load_input("../input/input_day_five.txt", matrix.clone() ); 
  let mut count: usize = 0;
  for row in res.iter() {
    for item in row.iter() {
      if *item >= 2usize {
        count = count + 1 ;
      }
    }
  }
  println!("d5 p1 : {}",count);
  let res_p2 = load_input_w_diag("../input/input_day_five.txt", matrix.clone() ); 
  let mut count_p2: usize = 0;
  for row in res_p2.iter() {
    for item in row.iter() {
      if *item >= 2usize {
        count_p2 = count_p2 + 1 ;
      }
    }
  }
  println!("d5 p2 : {}",count_p2);
}
