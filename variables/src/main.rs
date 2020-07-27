fn main() {
  let mut x = 41;
  println!("The value of x is {}", x);
  x += 1;
  println!("The value of x is {}, x is mutable", x);
  println!("You can mutate x, but you can not change its type");
  let y = "42";
  println!("The value of y is {}, y is immutable", y);
  let y = 43;
  println!("The value of immutable y was shadowed to {}", y);
}