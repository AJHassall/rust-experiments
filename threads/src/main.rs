

mod timer;

use std::fs;
use std::sync::Arc;
use std::sync::Mutex;

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

//const max: u32 = 9;

fn main() {
    
    //generate_primes_threaded_rayon();

    //generate_unique_numbers();
    generate_primes_threaded(100000);
    generate_primes(100000)
    //test();
  //  a_test();
    //generate_primes();
    //generate_primes_threaded_rayon();
        
}

fn a_test(){

    let mut handles = vec![];
    let mut main_thread_vec = vec![];
    for i in 0..10{

        let h =std::thread::spawn(move || {
            let mut spawned_thread_vec = vec![];
            for j in 0..10{
                spawned_thread_vec.push(j);
            }
            spawned_thread_vec
        });
        handles.push(h);


    }

    for h in handles{
        main_thread_vec.extend(h.join().unwrap());
    }


    println!("{:?}", main_thread_vec);
    
}
fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;

    }

    
    for i in 2..n{
        if n % i == 0 {
            return false;
        }
        
    }
    true
}

fn compare_is_even_methods() {
    for j in 0..10{

        println!("run {}",j);
        {
            let _t = timer::Timer::new();
            print!("mod:            ");
            for i in 0..100000{
                let _ =  is_even_mod(i);
            }
        }

        {
            print!("bit wise and:   ");
            let _t = timer::Timer::new();
            for i in 0..100000{
                let _ =  is_even_bit_wise_and(i);
            }
        }

        println!("=======");
    }
}

fn generate_primes_threaded_rayon(max: u32) {
    use rayon::prelude::*;

    println!("Generating primes with rayon");
    let _t = timer::Timer::new();

    let primes: Arc<Mutex<Vec<u32>>> = Arc::new(Mutex::new(Vec::new()));

    const THREADS: u32 = 10;
    
    let total_iterations = max / THREADS;
    
    (0..THREADS).into_par_iter().for_each(|offset| {
        let primes_shared = Arc::clone(&primes);
        let mut  i: u32 = 0;
        while primes_shared.lock().unwrap().len() < total_iterations as usize {
            let n = i + offset;
            if is_prime(n) {
                primes_shared.lock().unwrap().push(n);
            }
            i+=THREADS;
        }
    });

    let _ =primes.lock().unwrap().iter().map(|s| println!("{}", s));
    fs::write("primes_rayon.txt", primes.lock().unwrap().iter().map(|x| x.to_string()).collect::<Vec<String>>().join("\n")).unwrap();
}


fn generate_primes_threaded(max: u32) {
    println!("Generating primes with threads");

    let _t = timer::Timer::new();
    let mut handles = vec![];
    let threads: u32 = 20;

    let candidates_per_thread = max / threads;

    for offset in 0..threads {
        let handle = std::thread::spawn(move || {

            let mut primes = vec![];
            for i in 0..candidates_per_thread{
                let n =offset * candidates_per_thread  + i;

                if  is_prime(n){
                    primes.push(n);
                }
            }
            primes

        });

        handles.push(handle);

        
    }

  // let mut numnums =vec![];
    for handle in handles {
        let mut nums = handle.join().unwrap();
     //   numnums.append(&mut nums)
       // println!("{:?}", new_primes);
    
    }

   // println!("{} primes", numnums.len());

}

fn generate_primes(max: u32) {
    println!("Single thread");
    
    let _t = timer::Timer::new();
    let mut primes = vec![2];

    let mut candidate = 3;
    while primes.len() < max as usize{
        //println!("{}, {}", primes.len(), max);
        let square_root = (candidate as f64).sqrt() as u64 + 1;

        let is_prime = primes
            .iter()
            .take_while(|p| p <= &&square_root)
            .all(|p| candidate % p != 0);

        if is_prime {
            primes.push(candidate);
        }
        candidate += 2;
    }
    
    // fs::write("primes_single_thread.txt", primes
    //     .iter()
    //     .map(|x| x.to_string())
    //     .collect::<Vec<String>>().join("\n"))
    //     .unwrap();
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
    assert_eq!(true, is_prime(2));
    assert_eq!(true, is_prime(313));
    assert_eq!(false, is_prime(500));
    assert_eq!(false, is_prime(3675));
}

#[test]
fn test_is_even_mod() {
    assert_eq!(true, is_even_mod(2));
    assert_eq!(true, is_even_mod(1000000));
    assert_eq!(false, is_even_mod(503));
    assert_eq!(false, is_even_mod(3675));
}
#[test]
fn test_is_even_bit_wise_and() {
    assert_eq!(true, is_even_bit_wise_and(2));
    assert_eq!(true, is_even_bit_wise_and(1000000));
    assert_eq!(false, is_even_bit_wise_and(503));
    assert_eq!(false, is_even_bit_wise_and(3675));
}