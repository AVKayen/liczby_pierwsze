use std::{vec, usize};
use std::time::{Instant, Duration};

// 1.
fn naive_all_smaller(n: usize) -> usize {
    let mut c: usize = 3;
    for i in 6..=n {
        let mut s: bool = true;
        for j in 2..i {
            if i%j == 0 {
                s = false;
                break;
            }
        }
        if s {
            c+=1;
        }
    }
    return c
}

// 2.
fn naive_all_but_even(n: usize) -> usize {
    let mut c: usize = 3;
    for i in 6..=n {
        let mut s: bool = true;
        if i%2 == 0 {
            continue;
        }
        for j in (3..i).step_by(2) {
            if i%j == 0 {
                s = false;
                break;
            }
        }
        if s {
            c+=1;
        }
    }
    return c
}

// 3.
fn naive_all_but_even_to_sqrti(n: usize) -> usize {
    let mut c: usize = 3;
    for i in 6..=n {
        let mut s: bool = true;
        if i%2 == 0 {
            continue;
        }
        let mut j:usize = 3;
        while j*j <= i {
            if i%j == 0 {
                s = false;
                break;
            }
            j += 2;
        }
        if s {
            c+=1;
        }
    }
    return c
}

// 4.
fn eratosthenes(n: usize) -> usize {
    let mut c: usize = 0;
    let mut vec: Vec<bool> = vec![true; n];
    let mut j: usize = 1;
    vec[1] = false;
    while j*j <= n {
        if vec[j] {
            let mut i: usize = 2;
            while i*j < n {
                vec[i*j] = false;
                i += 1;
            }
        }
        j += 1;
    }
    for i in 1..n {
        if vec[i] {
            c+=1;
        }
    }
    return c
}

// 5.
fn atkin(n: usize) -> usize {
    let mut c: usize = 0;
    let mut vec: Vec<bool> = vec![false; n+1];
    let mut j: usize = 6;
    vec[2] = true;
    vec[3] = true;

    let mut x: usize = 1;

    while x*x <= n {
        let mut y: usize = 1;
        while y*y <= n {
            let mut t: usize = (4*x*x) + (y*y);
            if t <= n && (t % 12 == 1 || t % 12 == 5) {
                vec[t] ^= true;
            }
            t = (3*x*x) + (y*y);
            if t <= n && t % 12 == 7 {
                vec[t] ^= true;
            }
            if x > y {
                t = (3*x*x) - (y*y);
                if t <= n && t % 12 == 11 {
                    vec[t] ^= true;
                }
            }
            y += 1;
        }
        x += 1;
    }

    let mut pow: usize = 5;

    while pow*pow <= n {
        if vec[pow] {
            for i in (pow*pow..n).step_by(pow*pow) {
                vec[i] = false;
            }
        }
        pow+= 1;
    }

    for i in 1..=n {
        if vec[i] {
            c+=1;
        }
    }

    return c
}

fn main() {
    let n: [usize; 4] = [1000, 10000, 100000, 1000000];
    for i in n {
        let now = Instant::now();
        atkin(i);
        let elapsed = now.elapsed();
        println!("Metoda sita atkina dla n={} zajęła {} mikrosekund czasu procesora", i, elapsed.as_micros());

        let now = Instant::now();
        eratosthenes(i);
        let elapsed = now.elapsed();
        println!("Metoda sita erastotenesa dla n={} zajęła {} mikrosekund czasu procesora", i, elapsed.as_micros());
        

        let now = Instant::now();
        naive_all_but_even_to_sqrti(i);
        let elapsed = now.elapsed();
        println!("Metoda 3 dla n={} zajęła {} mikrosekund czasu procesora", i, elapsed.as_micros());

        if i == n[3] {
            println!("Ctrl+C aby anulować proces, to będzie się chwilkę mieliło :D")
        }

        let now = Instant::now();
        naive_all_but_even(i);
        let elapsed = now.elapsed();
        println!("Metoda 2 dla n={} zajęła {} mikrosekund czasu procesora", i, elapsed.as_micros());

        let now = Instant::now();
        naive_all_smaller(i);
        let elapsed = now.elapsed();
        println!("Metoda 1 dla n={} zajęła {} mikrosekund czasu procesora\n", i, elapsed.as_micros());
    }
}
