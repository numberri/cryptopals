/*
Challenge:
     One of the 60-character strings in this file has been encrypted by single-character XOR.
     Find it.
     (Your code from #3 should help.) 
Go plan:
    This is annoying - however there is a good chance that when being
    XORed with a character it will have a most frequent character that
    isn't in the most common e, a, r, i, o, t, n, s
    Weird thing I noticed - there is a solid chance that the string with
    the least amount of spaces may be the one that is XORed.

    In a Rust newbie way, the most challenging thing here may be opening
    the file... >_<
*/

use data_encoding::HEXLOWER;

use crate::challenge_3;

pub fn challenge_4(input: &str) -> String {
    let hexes = input.lines();
    let mut out: String = "".to_owned();
    let mut raw: &str = "";
    for line in hexes {
        let bits = HEXLOWER.decode(line.as_bytes()).unwrap();
        match String::from_utf8(bits) {
            Ok(_yay) => {
                let decode = challenge_3(line);
                if decode.matches(" ").count() > 
                out.matches(" ").count() {
                    raw = line;
                    out = decode;
                }
            }
            Err(_oops) => ()
        }
    }
    raw.to_owned()
}

/*
In main.rs:
    let Ok(inp) = std::fs::read_to_string(format!("../inputs/challenge4.txt")) else {
        eprintln!("You fucked up on your file input :/"); 
        return;
    };
    let out = challenge_4(&inp);
    println!("{}", out);
    let extra = challenge_3(&out);
    println!("Decoding of XORed string: {}", extra);
*/