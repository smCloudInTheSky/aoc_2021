use advent_of_code_2021::util;

fn read_increase(path: &str) -> i32 {
  let mut cur_val: i32 = 0 ; 
  let mut inc: i32 = 0 ;
  let lines = util::read_lines(path).unwrap();
  for (index, line) in lines.enumerate() {
      let val: i32 = line.unwrap().parse().unwrap();
      if index != 0 {

          if val < cur_val  {
              println!("{} : decreased", val);
              cur_val=val; 

          } else if val > cur_val {
              println!("{} : increased",val);
              inc = inc + 1 ;
              cur_val = val ;
          } 
      } else {
          println!( "{} N/A no previous value", val ) ;
          cur_val=val;
      }

  }
  inc
}

fn read_3(path: &str) -> i32 {
    let lines = util::read_lines(path).unwrap();
    let mut data: Vec<i32> = Vec::new();
    let mut inc: i32 = 0 ;
    for line in lines {
        data.push(line.unwrap().parse::<i32>().unwrap());
    }
    let size = data.len() - 2 ; 
    println!("vector size : {}",data.len());
    println!("{} : size", size);
    println!("{}, {}, {}, {} ",data[0],data[1],data[2],data[3]);
    for i in 0..size {
        if i != 0 {
        let a = data[i-1] + data[i] + data[i+1];
        let b = data[i] + data[i+1] + data[i+2];
        if a < b {
            println!("{} : increased", a ) ;
            inc = inc + 1 ; 
        } else if a > b {
            println!("{} : decreased", a ) ;
        } else {
            println!("{} : no change", a) ; 
        } 
        }
        else {
            let a = data[i] + data[i+1] + data[i+2];
            println!( "{} N/A no previous value", a ) ;
        }

    }
    inc 
}


fn main () { 

    let result = read_increase("../input/input_day_one.txt");
    let result2 = read_3("../input/input_day_one.txt" );
    println!("result : {}",result);
    println!("result : {}", result2);

}
