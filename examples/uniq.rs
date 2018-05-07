use std::io;
use std::collections::HashSet;

fn main() {
    let mut uniq = HashSet::new();

    loop {
        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                //println!("{}",buffer);
                uniq.insert(buffer);
            }
            Err(error) => panic!("error: {}", error),
        }
    }

    println!("len: {}", uniq.len());
}
