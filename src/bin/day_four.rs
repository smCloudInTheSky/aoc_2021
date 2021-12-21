use advent_of_code_2021::util;


/// charger les inputs  : ligne 1 les chiffres du bingo le reste des matrices 5x5 séparés par un espace
/// faire une boucle pour déterminer sur les inputs la première matrice gagnate
/// calculer le score final (produit de la ligne/colonne gagnante * dernier numéro)
fn load_input(path: &str) -> (String , Vec<Vec<Vec<usize>>>)  {
  let mut lines = util::read_lines(path).unwrap();
  let mut result = String::new() ;
  if let Ok(bingo) = lines.next().unwrap() { result = bingo ; } ; // première ligne contient la listes des inputs du bingo
  println!("{}",result);
  lines.next(); // on supprime la première ligne vide
  let mut inc_m: usize = 0 ;
  let mut inc_l: usize = 0 ;
  let mut matrix = vec![ vec![ vec![ 0usize ; 5 ] ; 5 ] ] ;
  let mut matrix_tmp = vec![ vec![ 0usize ; 5 ] ; 5 ];
  for line in lines {
    if let Ok(content) = line {
      println!("{}", content);
      if content != "" {
        let liste: Vec<usize> =  content.split_whitespace().map(|s| s.parse::<usize>().expect("parse error")).collect();
        for (index, element) in liste.iter().enumerate() {
          matrix_tmp[inc_l][index] = *element;
        }
        println!("matrix number {} line {} complete !", inc_m,inc_l);
        if inc_l == 4 {
          println!("chargement de la matrice");
          matrix.push(matrix_tmp.clone());
          println!("matrice chargée");
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

fn calc_winner (mat: Vec<Vec<usize>>, bingo: Vec<usize>) -> bool {
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
  for i in 
  
}

fn main () { 
  let (liste , matrix ) = load_input("../input/input_day_four.txt");
  let bingo = liste.split_terminator(',');
  let mut bingo_tmp: Vec<usize> = vec![] ; 
  for num in bingo {
    bingo_tmp.push(num.parse::<usize>().unwrap()  ) ;
    for mat in matrix.iter() {
      calc_winner(mat.clone(), bingo_tmp.clone()) ;
    }
  }
}
