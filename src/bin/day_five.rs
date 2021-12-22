use advent_of_code_2021::util;

fn trace_matrix_w_diag(matrix: Vec<Vec<usize>>, coord1:String , coord2:String) -> Vec<Vec<usize>> {
 let start: Vec<&str> = coord1.split(',').collect();
 let end: Vec<&str> = coord2.split(',').collect();
 let x1: usize = start[0].to_string().parse().unwrap();
 let x2: usize = end[0].to_string().parse().unwrap();
 let y1: usize = start[1].to_string().parse().unwrap();
 let y2: usize = end[1].to_string().parse().unwrap();
 let mut mat = matrix.clone();
 if x1 == x2 {
   println!("coord 1 : {}, coord 2 : {}",coord1,coord2);
   if y1 < y2 {
   for i in y1..=y2 {
     println!("i = {}",i);
     mat[x1][i] = mat[x1][i] +1 ;
   }
   } else {
   for i in y2..=y1 {
     println!("i = {}",i);
     mat[x1][i] = mat[x1][i] +1 ;
   }
   }
 }
 else if y1 == y2  {
   println!("coord 1 : {}, coord 2 : {}",coord1,coord2);
   if x1 < x2 {
   for i in x1..=x2 {
     println!("i = {}",i);
     mat[i][y1] = mat[i][y1] +1 ;
   }
   } else {
   for i in x2..=x1 {
     println!("i = {}",i);
     mat[i][y1] = mat[i][y1] +1 ;
   }
   }
 } else {
 // on s'occupe du cas diagonale
 // méthode : calcul du vecteur -> x2 - x1 , y2 - y1 
 // ensuite boucle while pour aller du point 1 au point 2 
 // action classique
 // on aurait pu faire ça dès le début en fait... refacto à l'occasion
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
   println!("coord 1 : {}, coord 2 : {}",coord1,coord2);
   if y1 < y2 {
   for i in y1..=y2 {
     println!("i = {}",i);
     mat[x1][i] = mat[x1][i] +1 ;
   }
   } else {
   for i in y2..=y1 {
     println!("i = {}",i);
     mat[x1][i] = mat[x1][i] +1 ;
   }
   }
 }
 if y1 == y2  {
   println!("coord 1 : {}, coord 2 : {}",coord1,coord2);
   if x1 < x2 {
   for i in x1..=x2 {
     println!("i = {}",i);
     mat[i][y1] = mat[i][y1] +1 ;
   }
   } else {
   for i in x2..=x1 {
     println!("i = {}",i);
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

fn main() {
  let mut matrix: Vec<Vec<usize>> = vec![ vec![0usize ; 1000] ; 1000];
  let res = load_input("../input/input_day_five.txt", matrix ); 
  let mut count: usize = 0;
  for row in res.iter() {
    for item in row.iter() {
      if *item >= 2usize {
        count = count + 1 ;
      }
    }
  }
  println!("d5 p1 : {}",count);
}
