use data_encoding::HEXLOWER;
use data_encoding::BASE64;

pub fn challenge_1(hex: &str) -> String {
    let huh_whuh = HEXLOWER.decode(hex.as_bytes()); 
    let midpoint: Vec<u8> = huh_whuh.unwrap();
    let out = BASE64.encode(&midpoint);
    out
}

/*
in main.rs:
let out = challenge_1("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
println!("{}", out);
*/