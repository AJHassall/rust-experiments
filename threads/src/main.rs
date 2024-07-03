

mod timer;

use std::fs;
use std::sync::Mutex;
use rayon::prelude::*;

use std::collections::HashMap;

//const max: u32 = 9;
trait WriteToFile {
    fn write_to_file(&self, filename: &str);
}
impl WriteToFile for Vec<u32>{
    fn write_to_file(&self, filename: &str) {
        
        let result = fs::write(filename, self
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n"));

        match result {
            Ok(_) => { println!("Writen to {}", filename)},
            Err(_) =>{ println!("Error writing to file")}
        }

    }

}

fn main() {

    let mut max = 1_000_000_000;

    generate_primes_threaded(max).write_to_file("filename.txt");
    
    generate_primes(max)
        .write_to_file("generate_primes.txt");

    let mut primes = generate_primes_threaded_rayon(max);
    primes.sort();
    primes.write_to_file("generate_primes_threaded_rayon.txt");
}

fn is_prime(n: u32) -> bool {

    if n <= 1 {
        return false;
        
    }
    
    for i in 2..=((n as f64).sqrt() as u32){
        if n % i == 0 {
            return false;
        }
        
    }
    true
}

fn compare_is_even_methods() {
        {   
            let _t = timer::Timer::new();
            {
                for i in 0..10000000{
                    is_even_mod(i);
                }
            }
            print!("mod:            ");
        }
        

        {
            let _t = timer::Timer::new();
            for i in 0..10000000{
                is_even_bit_wise_and(i);
            }
            print!("bit wise and:   ");
        }

        println!("=======");
}

fn generate_primes_threaded_rayon(max: u32)->Vec<u32> {
    println!("Generating primes up to {max} with rayon");

    let _t = timer::Timer::new();

    (2..=max).into_par_iter()
        .filter(|a|is_prime(*a))
        .collect()
}


use std::sync::{Arc};
use std::thread;

fn generate_primes_threaded(max: u32) -> Vec<u32> {
    println!("Threaded execution");

    let _t = timer::Timer::new();

    let num_threads = 8;

    // Calculate sub-range size
    let sub_range_size = (max + num_threads - 1) / num_threads; // Ensure all numbers are covered

    // Thread-safe container for final primes (empty initially)
    let final_primes = Arc::new(Mutex::new(vec![]));

    // Spawn threads, each responsible for a sub-range
    let threads = (0..num_threads)
        .map(|thread_id| {
            let start = thread_id * sub_range_size + 2; // Start from 2 (inclusive)
            let end = (thread_id + 1) * sub_range_size; // End (exclusive)
            let final_primes_clone = Arc::clone(&final_primes);

            thread::spawn(move || {
                let mut local_primes = vec![];
                let mut candidate = start;
                while candidate < end {
                    let square_root = (candidate as f64).sqrt() as u32 + 1;
                    let is_prime = (2..=square_root).all(|p| candidate % p != 0);
                    if is_prime {
                        local_primes.push(candidate);
                    }
                    candidate += 1;
                }

                // Acquire lock once to append to final primes
                let mut final_primes_lock = final_primes_clone.lock().unwrap();
                final_primes_lock.append(&mut local_primes);
            })
        })
        .collect::<Vec<_>>();

    threads.into_iter().for_each(|thread| thread.join().unwrap());

    // Extract and sort final primes
    let mut final_primes_vec = final_primes.lock().unwrap().to_vec();
    final_primes_vec.sort(); // Sort for efficiency if order matters

    final_primes_vec
}





fn generate_primes(max: u32) -> Vec<u32> {
    println!("Single thread");
    
    let _t = timer::Timer::new();
    let mut primes = vec![2];

    let mut candidate = 3;
    while candidate < max{
        let square_root = (candidate as f64).sqrt() as u32 + 1;

        let is_prime = primes
            .iter()
            .take_while(|p| p <= &&square_root)
            .all(|p| candidate % p != 0);

        if is_prime {
            primes.push(candidate);
        }
        candidate += 2;
    }

    primes
}   

fn generate_unique_numbers() {
    let n = 0;
    let mut v = vec![];

    let mut hash_map = HashMap::new();
    let mut i = 0;
    while i < 100  {
    for offset in 0..20 {
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


fn is_even_mod(n: i128) -> bool {
     n % 2 == 0
}

fn is_even_bit_wise_and(n: i128) -> bool {
    n & 1 == 0
}

#[test]
fn test_is_prime() {
    assert!(is_prime(2));
    assert!(is_prime(313));
    assert!(!is_prime(500));
    assert!(!is_prime(3675));
}

#[test]
fn test_is_even_mod() {
    assert!(is_even_mod(2));
    assert!(is_even_mod(1000000));
    assert!(!is_even_mod(503));
    assert!(!is_even_mod(3675));
}
#[test]
fn test_is_even_bit_wise_and() {
    assert!(is_even_bit_wise_and(2));
    assert!(is_even_bit_wise_and(1000000));
    assert!(!is_even_bit_wise_and(503));
    assert!(!is_even_bit_wise_and(3675));
}