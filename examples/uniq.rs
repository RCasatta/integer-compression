extern crate sha2;
extern crate data_encoding;

use std::io;
use std::collections::HashSet;
use sha2::{Sha256, Digest};
use data_encoding::HEXLOWER;


fn main() {
    let mut uniq = HashSet::new();

    loop {
        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                //println!("({})",buffer.trim());
                let bytes = buffer.into_bytes();
                match HEXLOWER.decode(&bytes[0..bytes.len()-1]) {
                    Ok(vec) => {
                        let mut hasher = Sha256::default();
                        hasher.input(&vec[..]);
                        let sha_result = hasher.result();
                        let trunc = &sha_result[0..4];
                        //println!("({:?})",trunc);
                        uniq.insert(trunc.to_owned());
                    },
                    Err(_) => {
                        println!("non parsable");
                    }
                }


            }
            Err(error) => panic!("error: {}", error),
        }
    }

    println!("len: {}", uniq.len());
}
