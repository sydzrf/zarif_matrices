use ndarray::Array;
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;
use std::thread;
use std::time::Instant;
extern crate rayon;
use std::sync::{Arc, Mutex};
//use std::thread;

fn main() {
    let start = Instant::now();
    //give multiple thread safe owner by using smart pointer
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        //spawn ten threads each thread inc our value once
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            for i in 0..200 {
                let a = Array::random((i, i), Uniform::new(0., 10.));
                let b = Array::random((i, i), Uniform::new(0., 10.));

                println!("{:.2}", a.dot(&b));
                println!("For {} X {} Matrices.", i, i);
            }

            *num += 1;
        });
        handles.push(handle);
    }

    //we join our threads at the end this synchronises which will start/end at diff times

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Total threads: {}", *counter.lock().unwrap());
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}
