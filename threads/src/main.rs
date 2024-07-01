

mod timer;

use std::fs;
use std::sync::Arc;
use std::sync::Mutex;

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
fn main() {

    generate_primes_threaded();
    generate_primes_threaded_rayon();
    //test();

        
}
fn is_prime(n: i128) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..n/2{
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn generate_primes_threaded_rayon() {
    use rayon::prelude::*;

    let _t = timer::Timer::new();

    let primes = Arc::new(Mutex::new(Vec::new()));
    let primes_shared = Arc::clone(&primes);

    const THREADS: i32 = 10;
    let max = 1000;
    
    let total_iterations = max / (THREADS as i128);
    let mut current_iterations = 0;
    
    (0..THREADS).into_par_iter().for_each(|offset| {
        let mut i = offset as i128 * total_iterations;
        while i < max {
            if is_prime(i) {
                primes_shared.lock().unwrap().push(i);
            }
            i += total_iterations;

        }
    });
}


fn generate_primes_threaded() {
    let primes = Arc::new(Mutex::new(Vec::new()));
    let _t = timer::Timer::new();

    const max: i128 = 100;
    const THREADS: i32 = 10;
        
    let mut handles = Vec::new();
    for offset in 0..THREADS {      
        let primes_shared = Arc::clone(&primes);
        let handle = std::thread::spawn( move || {
            let mut  i = 0;
            while primes_shared.lock().unwrap().len() < 1000000 {
                if is_prime(i+offset as i128) {
                    primes_shared.lock().unwrap().push(i+offset as i128);
                }
                i+=THREADS as i128;

                // if offset == 0 {
                //     println!("{}%", i as f64 / max as f64 * 100.0);
                    
                // }
            }               
        });


        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }
    
    let mut file = File::create("primes.txt");
    fs::write("primes.txt", primes.lock().unwrap().iter().map(|x| x.to_string()).collect::<Vec<String>>().join("\n")).unwrap();
    for p in primes.lock().unwrap().iter() {
        //println!("{}", p);
    }
}

fn generate_primes() {
    println!("Single thread");
    {
        let _t = timer::Timer::new();
        let mut primes = vec![];

        for n in 0..1000000 {
            if is_prime(n) {
                primes.push(n);
            }
        }
    }
}   

fn generate_unique_numbers() {
    let n = 0;
    let mut v = vec![];

    let mut hash_map = HashMap::new();
    let mut i = 0;
    while i < 100  {
        for offset in 0..14 {
            print!("{},    ", i+offset);
            v.push(i+offset);

            match hash_map.get(&(i+offset)) {
                Some(count) => {
                    hash_map.insert(i+offset, count+1);
                }
                None => {
                    hash_map.insert(i+offset, 1);
                }
            }
        }
        i+= 20;
        println!();
    }
    let has_duplicates = hash_map.values().any(|a| *a > 1);
    if has_duplicates {
        println!("has duplicates");
        
    }
    else {
        println!("no duplicates");
    }
}


#[test]
fn test() {
    assert_eq!(true, is_prime(2));
    assert_eq!(true, is_prime(313));
    assert_eq!(false, is_prime(500));
    assert_eq!(false, is_prime(3675));
}