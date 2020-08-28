use std::io::stdin;
use std::process::Command;

use rand::Rng;

fn main() {
  let mut vector: Vec<i32> = vec![];
  let vec_length: i32 = parse_input_number("Input vec to sort length");
  let range: [i32; 2] = [
    parse_input_number("Input min value"),
    parse_input_number("Input max value"),
  ];

  for _ in 0..vec_length {
    vector.push(rand::thread_rng().gen_range(range[0], range[1]));
  }

  println!("\x1b[0;36mSorting...");

  let ans: Vec<i32> = bubble_sort_colored(vector);

  println!("\x1b[0;36mSort finished: {:?}", ans);
}

fn bubble_sort_colored(n: Vec<i32>) -> Vec<i32> {
  let mut list = n;
  let mut i = 0;
  let mut j = 0;

  while i < list.len() {
    while j < list.len() {
      if list[i] > list[j] {
        let tmp = list[i];
        list[i] = list[j];
        list[j] = tmp;
      }
      j += 1;
    }
    i += 1;
    j = i + 1;
    let mut colored_vec: Vec<String> = vec![];

    for elem in list.iter() {
      let color = get_number_color(elem, &list);
      colored_vec.push([color, elem.to_string()].concat());
    };

    println!("{}", colored_vec.join(", "));

    wait(".1");
  }

  list
}

fn bubble_sort(n: Vec<i32>) -> Vec<i32> {
  let mut list = n;
  let mut i = 0;
  let mut j = 0;

  while i < list.len() {
    while j < list.len() {
      if list[i] > list[j] {
        let tmp = list[i];
        list[i] = list[j];
        list[j] = tmp;
      }
      j += 1;
    }
    i += 1;
    j = i + 1;
  }

  list
}

fn wait(time: &str) {
  Command::new("sleep")
    .arg(time)
    .spawn()
    .unwrap()
    .wait()
    .unwrap();
}

fn parse_input_number(message: &str) -> i32 {
  let mut buffer = String::new();
  let number: i32;

  println!("{}", message);

  stdin()
    .read_line(&mut buffer)
    .expect("Failed to read line");

  loop {
    number = match buffer.trim().parse() {
      Ok(parsed_number) => parsed_number,
      Err(_) => {
        println!("Enter a number");
        buffer.clear();
        continue;
      },
    };

    break;
  }

  number
}

fn get_number_color(num: &i32, vector: &Vec<i32>) -> String {
  let sorted_vec: Vec<i32> = bubble_sort(vector.clone());
  let mut color: String = String::from("Red");

  for (index, vec_num) in sorted_vec.iter().enumerate() {
    if vec_num == num {
      let bigger_then = 100 * index / vector.len();

      color = match bigger_then {
        bigger_then if bigger_then < 16 => String::from("\x1b[0;31m"),
        bigger_then if bigger_then < 32 => String::from("\x1b[1;31m"),
        bigger_then if bigger_then < 48 => String::from("\x1b[1;33m"),
        bigger_then if bigger_then < 64 => String::from("\x1b[0;32m"),
        bigger_then if bigger_then < 80 => String::from("\x1b[1;34m"),
        bigger_then if bigger_then <= 100 => String::from("\x1b[0;35m"),
        _ => String::from("black"),
      };
    }
  }
  color
}
