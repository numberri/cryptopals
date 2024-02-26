use data_encoding::HEXLOWER;

pub fn challenge_3(hex: &str) -> String {
    let bits = HEXLOWER.decode(hex.as_bytes()).unwrap();
    let mut ch: u8 = 32;
    let mut out: String = "".to_owned();
    while ch < 127 {
        let bits_iter = bits.iter();
        let test = bits_iter.map(|x| x ^ ch).collect();
        let out_possible = String::from_utf8(test);
        match out_possible {
            Ok(result) => {
                // println!("{}", result);
                // note that E is not the most common letter here.
                // it may be worth outputting several strings that have
                // other common letters as the most frequent as well
                // edit: ' ' is best lmao
                if result.matches(" ").count() > 
                out.matches(" ").count() {
                    out = result;
                }
            }
            Err(oops) => eprintln!("that didn't work, error {oops:?}")
        }
        ch += 1;
    }
    out
}

/* 
in main.rs:
let out = challenge_3("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
println!("{}", out);
*/