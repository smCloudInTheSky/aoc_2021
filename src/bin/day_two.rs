use advent_of_code_2021::util;

fn read_positions(path: &str) -> Vec<i32> {
  let mut pos: Vec<i32> = Vec::new() ;
  // horizontal position
  pos.push(0);
  // depth
  pos.push(0);
  // aim
  pos.push(0);
  let lines = util::read_lines(path).unwrap();
  for line in lines {
      let line_c = line.unwrap().clone() ;
      let mut content = line_c.split_whitespace();
      let mov = content.next().unwrap() ;
      let val = content.next() ; 
      println!("movement : {}", mov);
      println!("content : {}", val.unwrap());
      match mov {
          "forward" => { 
              pos[0] = pos[0] + val.unwrap().parse::<i32>().unwrap();
              pos[1] = pos[1] + pos[2] * val.unwrap().parse::<i32>().unwrap();
          },
          "down" => {
              pos[2] = pos[2] + val.unwrap().parse::<i32>().unwrap()
          },
          "up" => {
              pos[2] = pos[2] - val.unwrap().parse::<i32>().unwrap() 
          },
          _ => println!("Unknwon movement"),
      }
  }
  pos
}

fn main () { 
    let result = read_positions("../input/input_day_two.txt");
    let final_value = result[0] * result[1];
    println!("result : {}",final_value);

}
