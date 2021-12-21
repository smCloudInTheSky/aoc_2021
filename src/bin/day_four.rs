use advent_of_code_2021::util;


/// charger les inputs  : ligne 1 les chiffres du bingo le reste des matrices 5x5 séparés par un espace
/// faire une boucle pour déterminer sur les inputs la première matrice gagnate
/// calculer le score final (produit de la ligne/colonne gagnante * dernier numéro)
fn load_input(path: &str) -> (String , Vec<Vec<Vec<usize>>>)  {
  let mut lines = util::read_lines(path).unwrap();
  let mut result = String::new() ;
  if let Ok(bingo) = lines.next().unwrap() { result = bingo ; } ; // première ligne contient la listes des inputs du bingo
  lines.next(); // on supprime la première ligne vide
  let mut inc_m: usize = 0 ;
  let mut inc_l: usize = 0 ;
  let mut matrix = vec![ vec![ vec![ 0usize ; 5 ] ; 5 ] ] ;
  let mut matrix_tmp = vec![ vec![ 0usize ; 5 ] ; 5 ];
  for line in lines {
    if let Ok(content) = line {
      if content != "" {
        let liste: Vec<usize> =  content.split_whitespace().map(|s| s.parse::<usize>().expect("parse error")).collect();
        for (index, element) in liste.iter().enumerate() {
          matrix_tmp[inc_l][index] = *element;
        }
        if inc_l == 4 {
          matrix.push(matrix_tmp.clone());
        }
        inc_l = inc_l + 1 ; 
      } else {
        inc_m = inc_m + 1 ;
        inc_l = 0  ;
      }
    }
  }
  ( result , matrix ) 
}

fn calc_winner (mat: Vec<Vec<usize>>, bingo: Vec<usize>) -> usize {
  let mut pos = vec![ vec![ 0usize; 5 ] ; 5 ] ;
  for i in 0..=4 {
    for j in 0..=4 {
      if bingo.contains(&mat[i][j]) {
        pos[i][j] = 1 ;
      } else {
        pos[i][j] = 0 ;
      }
    }
  }
  for i in 0..=4 {
    let col = pos[0][i] + pos[1][i] + pos[2][i] + pos[3][i] + pos[4][i] ;
    if col == 5 {
      let mut result: usize = 0 ;
      for i in 0..=4 { 
        for j in 0..=4 {
          result = result + mat[i][j] * ( 1 - pos[i][j] ) ;
        }
      }
      result = result * bingo.clone().pop().unwrap() ;
      return result ;  
    }
    let row = pos[i][0] + pos[i][1] + pos[i][2] + pos[i][3] + pos[i][4] ; 
    if row == 5 {
      let mut result: usize = 0 ;
      for i in 0..=4 { 
        for j in 0..=4 {
          result = result + mat[i][j] * ( 1- pos[i][j] ) ;
        }
      }
      result = result * bingo.clone().pop().unwrap() ;
      return result ;
    }
  } 
  return 0;
}

fn main () { 
  let (liste , matrix ) = load_input("../input/input_day_four.txt");
  let bingo = liste.split_terminator(',');
  let mut bingo_tmp: Vec<usize> = vec![] ; 
  'first: for num in bingo.clone() {
    bingo_tmp.push(num.parse::<usize>().unwrap()  ) ;
    for mat in matrix.clone().iter() {
      let mat_res = calc_winner(mat.clone(), bingo_tmp.clone()) ;
      if mat_res != 0 { 
        println!("first winning board score is : {}", mat_res);
        break 'first ;
      }
    }
  }
  let mut list_mat = matrix.clone() ; 
  let mut last_res: usize = 0 ;
  'last: for num in bingo.clone() {
    let mut to_delete: Vec<usize> = Vec::new();
    bingo_tmp.push(num.parse::<usize>().unwrap()  ) ;
    for (index, mat) in list_mat.iter().enumerate() {
      let mat_res = calc_winner(mat.clone(), bingo_tmp.clone()) ;
      if mat_res != 0 { 
        last_res = mat_res ;
        to_delete.push(index) ; 
        println!("current length of the matrix list : {}",list_mat.len());
        if list_mat.len() == 1 {
          println!("last winning board score is : {}",mat_res);
          break 'last;
        }
      }
    }
    for i in to_delete.iter().rev() {
      list_mat.remove(*i);
    }
  }
  println!("last winning score board is : {}",last_res);
}
