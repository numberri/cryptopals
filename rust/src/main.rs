// I was considering using macros to do this, but this is an issue of "do I copy somebody
// else's code to make this easy or do I figure something out myself"
// I decided I'd prefer to do this myself, which means I do it badly and manually.

mod set_1 {
    pub mod challenge3;
    pub mod challenge4;
}
use set_1::challenge3::*;
use set_1::challenge4::*;

fn main() {
    let Ok(inp) = std::fs::read_to_string(format!("../inputs/challenge4.txt")) else {
        eprintln!("You fucked up on your file input :/"); 
        return;
    };
    let out = challenge_4(&inp);
    println!("{}", out);
    let extra = challenge_3(&out);
    println!("Decoding of XORed string: {}", extra);
}

