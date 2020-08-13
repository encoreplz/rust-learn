use std::io::stdin;

fn main() {
  let vec_length: u32;

  let mut i = 0;
  let mut vector_to_sort: Vec<i32> = vec![];
  let mut vec_length_buffer: String = String::new();
  let mut vec_element_buffer: String = String::new();

  loop {
    println!("Enter array length");

    stdin()
        .read_line(&mut vec_length_buffer)
        .expect("Failed to read line");

    vec_length = match vec_length_buffer.trim().parse() {
      Ok(parsed_number) => parsed_number,
      Err(_) => {
        println!("Enter a number");
        vec_length_buffer.clear();
        continue;
      },
    };

    break;
  }

  while i < vec_length {
    println!("Enter {} more numbers", vec_length - i);

    stdin()
      .read_line(&mut vec_element_buffer)
      .expect("Failed to read line");

    match vec_element_buffer.trim().parse() {
      Ok(parsed_number) => {
        vector_to_sort.push(parsed_number);
        vec_element_buffer.clear();
        i += 1;
      },
      Err(_) => {
        println!("Enter a number!");
        vec_element_buffer.clear();
        continue;
      },
    };
  }

  let ans: Vec<i32> = bubble_sort(vector_to_sort);

  println!("Sort finished: {:?}", ans);
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