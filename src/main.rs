use rayon::prelude::*;
use std::collections::HashMap;
use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut limit: usize = 10_000_000;
    let mut debug = false;
    match args.len() {
        2 => limit = args[1].parse().unwrap(),
        3 => {
            limit = args[1].parse().unwrap();
            if args[2] == "debug" {
                debug = true;
            }
        }
        _ => (),
    }

    println!("Longest Collatz under: {}", limit);
    run("p014_brute   ", p014_brute, limit, debug);
    run("p014_vect    ", p014_vect, limit, debug);
    run("p014_map     ", p014_map, limit, debug);
    run("p014_parallel", p014_parallel, limit, debug)
}

fn run<F: FnOnce(usize) -> (usize, u32)>(name: &str, f: F, limit: usize, debug: bool) {
    let instant = Instant::now();
    let (longest, length) = f(limit);
    let elapsed = instant.elapsed().as_secs_f32();
    println!(
        "{} = {}, length = {}, duration = {:.8}s",
        name, longest, length, elapsed
    );
    if debug {
        println!("debug on");
        println!("{:?}", get_collatz_sequence(longest));
        println!("get_collatz({}) = {}", longest, get_collatz(longest))
    }
}

pub fn p014_brute(limit: usize) -> (usize, u32) {
    if limit < 3 {
        return (1, 1);
    }

    (2..limit)
        .into_iter()
        .map(|x| (x, get_collatz(x)))
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap()
}

pub fn p014_vect(limit: usize) -> (usize, u32) {
    if limit < 2 {
        return (1, 1);
    }

    let mut cache: Vec<u32> = vec![0; limit];

    let mut max = 0;
    let mut answer = 1;
    for i in 1..limit {
        let mut n = i;
        let mut cnt: u32 = 0;
        while n != 1 {
            cnt += 1;
            n = if n % 2 == 0 { n >> 1 } else { 3 * n + 1 };
            if n < i {
                cnt += cache[n] as u32;
                break;
            }
        }
        cache[i] = cnt;
        if cnt > max {
            max = cnt;
            answer = i
        }
    }
    (answer, cache[answer] + 1)
}

pub fn p014_map(limit: usize) -> (usize, u32) {
    if limit < 3 {
        return (1, 1);
    }

    let mut cache: HashMap<usize, u32> = HashMap::with_capacity(limit);

    let mut max = 0;
    let mut answer = 1;
    for i in 2..limit {
        let mut n = i;
        let mut cnt: u32 = 0;
        while n != 1 {
            cnt += 1;
            n = if n % 2 == 0 { n >> 1 } else { 3 * n + 1 };
            if n < i {
                match cache.get(&n) {
                    Some(v) => cnt += v,
                    None => (),
                }
                break;
            }
        }
        cache.insert(i, cnt);
        if cnt > max {
            max = cnt;
            answer = i
        }
    }
    (answer, cache.get(&answer).unwrap() + 1)
}

pub fn p014_parallel(limit: usize) -> (usize, u32) {
    if limit < 3 {
        return (1, 1);
    }

    (2..limit)
        .into_par_iter()
        .map(|x| (x, get_collatz(x)))
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap()
}

pub fn get_collatz(val: usize) -> u32 {
    let mut n = val;
    let mut cnt: u32 = 1;
    while n != 1 {
        cnt += 1;
        n = if n % 2 == 0 { n >> 1 } else { 3 * n + 1 };
    }
    return cnt;
}

pub fn get_collatz_sequence(val: usize) -> Vec<usize> {
    let mut n = val;
    let mut xs = vec![val];
    while n != 1 {
        n = if n % 2 == 0 { n >> 1 } else { 3 * n + 1 };
        xs.push(n);
    }
    return xs;
}
