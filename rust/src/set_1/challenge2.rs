use data_encoding::HEXLOWER;

pub fn challenge_2(hex1: &str, hex2: &str) -> String {
    let bits1 = HEXLOWER.decode(hex1.as_bytes()).unwrap();
    let bits2 = HEXLOWER.decode(hex2.as_bytes()).unwrap();
    let bits3: Vec<u8> = bits1.to_owned()
        .iter()
        .zip((bits2.to_owned()).iter())
        .map(|(&x1, &x2) | x1 ^ x2)
        .collect();
    let out = HEXLOWER.encode(&bits3);
    out
}

/*
in main.rs: 
let out = challenge_2("1c0111001f010100061a024b53535009181c", "686974207468652062756c6c277320657965");
println!("{}", out);
*/