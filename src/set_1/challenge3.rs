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
                if result.matches("e").count() > 
                out.matches("e").count() {
                    out = result;
                }
            }
            Err(oops) => eprintln!("that didn't work, error {oops:?}")
        }
        ch += 1;
    }
    out
}