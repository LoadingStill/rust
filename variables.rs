//standard variable
  //can not change the vaule while in the immutable state, you can change the variable into a mutable state to change (but this is not the default)
fn main() {
    let bunnies = 2;
}

//32 bit variable
  //immutable
fn main() {
    let bunnies: i32 = 4;
}


//multiple variables in the same line
  //immutable
fn main() {
    let (bunnies, carrots) = (8, 50);
}
