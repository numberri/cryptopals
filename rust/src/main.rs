// I was considering using macros to do this, but this is an issue of "do I copy somebody
// else's code to make this easy or do I figure something out myself"
// I decided I'd prefer to do this myself, which means I do it badly and manually.

mod set_1 {
    pub mod challenge3;
}
use set_1::challenge3::*;

fn main() {
    let out = challenge_3("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    println!("{}", out);
}
