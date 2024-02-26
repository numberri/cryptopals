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

Aside after doing this:
I know that as someone who is not used to Rust, I am very inclined to
do this using may let statements. Looking at this now, I realise that
I may only need one
*/

pub fn challenge_1_alt_a(hex: &str) -> String {
    let out = BASE64.encode(&
        HEXLOWER.decode(hex.as_bytes())
        .unwrap());
    out
}

//I have been informed I don't even need the let. Goddamn

pub fn challenge_1_alt_b(hex: &str) -> String {
    BASE64.encode(&HEXLOWER.decode(hex.as_bytes()).unwrap())
}
