extern crate sha2;
extern crate data_encoding;

use std::io;
use std::collections::HashSet;

use data_encoding::HEXLOWER;


fn main() {
    let mut uniq = HashSet::new();
    let mut count_i = 0u64;
    let mut count_o = 0u64;
    let mut buffer = String::new();

    loop {
        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                //println!("({})",buffer.trim());
                let splitted : Vec<&str> = buffer.split(" ").collect();
                if splitted.len() != 4 {
                    break;
                }
                let mut tx_hex_bytes = match HEXLOWER.decode(&splitted[2].to_owned().into_bytes()) {
                    Ok(result) => result,
                    Err(_) => break,
                };

                let mut index_bytes = splitted[3].to_owned().into_bytes();

                match splitted[0] {
                    "I" => {
                        tx_hex_bytes.reverse(); //this is in le, going be
                        tx_hex_bytes.append(&mut index_bytes);
                        uniq.remove(&tx_hex_bytes);
                        count_i += 1;
                    },
                    "O" => {
                        tx_hex_bytes.append(&mut index_bytes);
                        uniq.insert(tx_hex_bytes.clone());
                        count_o += 1;

                    },
                    _ => break,
                }

            }
            Err(error) => panic!("error: {}", error),
        }
    }
    println!("count_i {}", count_i);
    println!("count_o {}", count_o);
    println!("uniq len: {}", uniq.len());
}
