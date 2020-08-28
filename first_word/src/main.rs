use std::io::stdin;

fn main() {
  let mut sentence = String::new();

  println!("Enter a sentence:");
  stdin().read_line(&mut sentence).expect("Invalid Input");

  println!("First word is {}", first_word(sentence));
}

fn first_word(sentence: String) -> String {
  for (i, &char) in sentence.as_bytes().iter().enumerate() {
    if char == b' ' || char == b',' {
      return String::from(&sentence[0..i])
    }
  }
  return sentence
}
