extern crate rand;

use rand::Rng;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::ops::Sub;
use std::cmp::Ordering;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash)]
struct U40 ([u8; 5]);  // BIG ENDIAN most significant byte is the first


impl U40 {
    fn new(arr : [u8;5]) -> U40 {
        U40(arr)
    }
}

impl Default for U40 {
    fn default() -> Self {
        U40::new([0u8;5])
    }
}

impl From<u64> for U40 {
    fn from(x : u64) -> Self {
        let b0 : u8 = ((x >> 32) & 0xff) as u8;
        let b1 : u8 = ((x >> 24) & 0xff) as u8;
        let b2 : u8 = ((x >> 16) & 0xff) as u8;
        let b3 : u8 = ((x >> 8) & 0xff) as u8;
        let b4 : u8 = (x & 0xff) as u8;
        U40([b0,b1,b2,b3,b4])
    }
}

impl U40 {
    fn to_u64(&self) -> u64 {
        let mut acc= 0u64;
        for i in 0..5usize {
            acc += self.0[4-i] as u64 * 256u64.pow(i as u32);
        }
        acc
    }
}


impl Sub for U40 {
    type Output = U40;

    fn sub(self, other: U40) -> U40 {
        U40::from(self.to_u64() - other.to_u64())
    }
}

fn main() {

    let n_vec  = [1_000_000usize,15_000_000,50_000_000,100_000_000,200_000_000,400_000_000,800_000_000];

    for n in n_vec.iter() {
        println!("n {} U40", n);
        //let mut vec = init_u32_random_vec(*n);
        let mut vec = init_U40_random_vec(*n);
        //let set : HashSet<u32> = HashSet::from_iter(vec.iter().cloned());
        let set : HashSet<U40> = HashSet::from_iter(vec.iter().cloned());
        //let _collisions = vec.len() - set.len();

        println!("% Collisions {:.2}%",  (1f64- (set.len() as f64 / *n as f64))*100f64   );

        //println!("{:?}",vec);
        vec.sort();
        //println!("{:?}",vec);

        let mut total = 0u64;
        let mut counters = [0u32; 5];
        for i in 0..n-1 {
            let delta = vec[i + 1].to_u64() - vec[i].to_u64();
            if delta < 2u64.pow(7) {
                counters[0]+=1;
            } else if delta < 2u64.pow(14) {
                counters[1]+=1;
            } else if delta < 2u64.pow(21) {
                counters[2]+=1;
            } else if delta < 2u64.pow(28) {
                counters[3]+=1;
            } else {
                counters[4]+=1;
            }
            total = total + delta as u64;
        }

        println!("avg:{:?}", (total as f64 / (n-1) as f64 ));
        println!("counters:{:?}", counters) ;

        let mut size: u32 = 0;
        for i in 0..5 {
            size += counters[i]*i as u32;
        }
        size += *n as u32 * 3; // adding height encoding in 3 bytes
        let mb_size = size as f64 / 2f64.powf(20f64);
        let mb_size_orig = (n*8) as f64 / 2f64.powf(20f64);
        println!("Filter Size {:.2}Mb ({:.2}%)", mb_size,  (mb_size/ mb_size_orig)*100f64 );
        println!("--------")
        //println!("Collision adjusted {:.2}Mb", collision as f64 * 1f64) //considering a 1Mb block
    }



    for n in n_vec.iter() {
        println!("n {} u32", n);
        //let mut vec = init_u32_random_vec(*n);
        let mut vec = init_u32_random_vec(*n);
        //let set : HashSet<u32> = HashSet::from_iter(vec.iter().cloned());
        let set : HashSet<u32> = HashSet::from_iter(vec.iter().cloned());
        //let _collisions = vec.len() - set.len();

        println!("% Collisions {:.2}%",  (1f64- (set.len() as f64 / *n as f64))*100f64   );

        //println!("{:?}",vec);
        vec.sort();
        //println!("{:?}",vec);

        let mut total = 0u64;
        let mut counters = [0u32; 5];
        for i in 0..n-1 {
            let delta = vec[i + 1] - vec[i];
            if delta < 2u32.pow(7) {
                counters[0]+=1;
            } else if delta < 2u32.pow(14) {
                counters[1]+=1;
            } else if delta < 2u32.pow(21) {
                counters[2]+=1;
            } else if delta < 2u32.pow(28) {
                counters[3]+=1;
            } else {
                counters[4]+=1;
            }
            total = total + delta as u64;
        }

        println!("avg:{:?}", (total as f64 / (n-1) as f64 ));
        println!("counters:{:?}", counters) ;

        let mut size: u32 = 0;
        for i in 0..5 {
            size += counters[i]*i as u32;
        }
        size += *n as u32 * 3; // adding height encoding in 3 bytes
        let mb_size = size as f64 / 2f64.powf(20f64);
        let mb_size_orig = (n*8) as f64 / 2f64.powf(20f64);
        println!("Filter Size {:.2}Mb ({:.2}%)", mb_size,  (mb_size/ mb_size_orig)*100f64 );
        println!("--------")
        //println!("Collision adjusted {:.2}Mb", collision as f64 * 1f64) //considering a 1Mb block
    }
}



fn init_u32_random_vec(n : usize) -> Vec<u32>{
    let mut rng = rand::thread_rng();
    let mut vec = Vec::new();
    for _i in 0..n {
        vec.push(rng.gen::<u32>());
    }
    vec
}


fn init_U40_random_vec(n : usize) -> Vec<U40>{
    let mut rng = rand::thread_rng();
    let mut vec = Vec::new();
    for _i in 0..n {
        vec.push(U40::from(rng.gen::<u64>()));
    }
    vec
}



#[cfg(test)]
mod test {
    use super::U40;
    use std::mem;
    use std::collections::HashSet;

    #[test]
    fn test_U40() {

        assert_eq!(5, mem::size_of::<U40>());

        let zero = U40::from(0u64);
        assert_eq!(zero.to_u64(), 0u64);

        let one = U40::from(1u64);
        assert_eq!(one.clone()-zero.clone(), one.clone());

        let string = format!("{:?}",one.clone());
        assert_eq!(string, "U40([0, 0, 0, 0, 1])");

        let b = U40([4u8,0u8,6u8,0u8,1u8]);
        let a = U40([3u8,10u8,5u8,2u8,0u8]);
        let string = format!("{:?}",b-a);
        assert_eq!(string, "U40([0, 246, 0, 254, 1])");

        let mut vec = Vec::new();
        let mut set = HashSet::new();
        for i in 0..100u64 {
            let u40 = U40::from(100 - i);
            vec.push(u40.clone());
            set.insert( u40);
        }
        vec.sort();
        assert_eq!(vec[0],one.clone());

        assert_eq!(set.len(),100);
        set.insert(one.clone());
        assert_eq!(set.len(),100);
    }

}



/*


impl U40 {
    fn sub2(&self, other: &U40) -> U40 {
        let a = self.0;
        let b = other.0;

        let (r0, o0) = a[0].overflowing_sub(b[0]);

        let (r1t, o1a) = a[1].overflowing_sub(b[1]);
        let (r1, o1b) = r1t.overflowing_sub(bool_to_u8(o0));

        let (r2t, o2a) = a[2].overflowing_sub(b[2]);
        let (r2, o2b) = r2t.overflowing_sub(bool_to_u8(o1a || o1b));

        let (r3t, o3a) = a[3].overflowing_sub(b[3]);
        let (r3, o3b) = r3t.overflowing_sub(bool_to_u8(o2a || o2b));

        let (r4t, o4a) = a[4].overflowing_sub(b[4]);
        let (r4, o4b) = r4t.overflowing_sub(bool_to_u8(o3a || o3b));

        //TODO how to handle last overflow?

        let ris = [r0,r1,r2,r3,r4];
        U40::new(ris)
    }
}

*/


/*
impl PartialOrd for U40 {
    fn partial_cmp(&self, other: &U40) -> Option<Ordering> {
        let a = self.0;
        let b = other.0;

        for i in 0..5usize {
            match a[i].cmp(&b[i]) {
                Ord
            }

        }
    }
}
*/

/*

fn bool_to_u8(val: bool) -> u8 {
    if val {
        1
    } else {
        0
    }
}
*/

/*

fn transform_u64_to_array_of_u8(x:u64) -> [u8;8] {
    let b1 : u8 = ((x >> 56) & 0xff) as u8;
    let b2 : u8 = ((x >> 48) & 0xff) as u8;
    let b3 : u8 = ((x >> 40) & 0xff) as u8;
    let b4 : u8 = ((x >> 32) & 0xff) as u8;
    let b5 : u8 = ((x >> 24) & 0xff) as u8;
    let b6 : u8 = ((x >> 16) & 0xff) as u8;
    let b7 : u8 = ((x >> 8) & 0xff) as u8;
    let b8 : u8 = (x & 0xff) as u8;
    return [b1, b2, b3, b4, b5, b6, b7, b8]
}

*/