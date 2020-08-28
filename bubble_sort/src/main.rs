use std::io::stdin;
use std::process::Command;

use rand::Rng;

const INSERTION: &str = "insertion";
const BUBBLE: &str = "bubble";

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

  println!("\x1b[0;36mSort finished: {:?}", sort_algorithm_aggregator(vector));
}

fn sort_algorithm_aggregator(list_to_sort: Vec<i32>) -> Vec<i32> {
  let mut sort_algorithm = String::new();
  println!("\x1b[0;36mChoose algorith to use ({:?})", vec![INSERTION, BUBBLE].join(" | "));
  stdin().
    read_line(&mut sort_algorithm)
    .expect("Failed to read line");

  match sort_algorithm.trim() {
    BUBBLE => bubble_sort(list_to_sort),
    INSERTION => insertion_sort(list_to_sort),
    _ => list_to_sort,
  }
}

fn bubble_sort(arr: Vec<i32>) -> Vec<i32> {
  let mut list = arr;
  let mut i = 0;
  let mut j = 0;

  while i < list.len() {
    while j < list.len() {
      if list[i] > list[j] {
        list.swap(i, j);
      }
      j += 1;
    }
    i += 1;
    j = i + 1;

    print_list_colored(&list);
    wait("1");
  }

  list
}

fn insertion_sort(arr: Vec<i32>) -> Vec<i32> {
  let mut list = arr;

  for i in 1..list.len() {
    let mut j = i;
    while j > 0 && list[j - 1] > list[j] {
      list.swap(j - 1, j);
      j -= 1;
    }
    print_list_colored(&list);
    wait("1");
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
  let number: i32;

  println!("{}", message);

  loop {
    let mut buffer = String::new();

    stdin()
      .read_line(&mut buffer)
      .expect("Failed to read line");

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

fn print_list_colored(list:  &Vec<i32>) {
  let mut colored_vec: Vec<String> = vec![];

  for elem in list.iter() {
    let color = get_number_color(elem, &list);
    colored_vec.push([color, elem.to_string()].concat());
  };

  println!("{}", colored_vec.join(", "));
}

fn get_number_color(num: &i32, vector: &Vec<i32>) -> String {
  let mut list: Vec<i32> = vector.clone();
  list.sort();
  let mut color: String = String::from("Red");

  for (index, vec_num) in list.iter().enumerate() {
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
