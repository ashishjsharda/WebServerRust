extern crate rand;

use std::io;
use rand::Rng;
fn main() {
   println!("Random number generator");
   let rand_num=rand::thread_rng().gen_range(1,101);
   println!("Random num generated is {}",rand_num);

}
