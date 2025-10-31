use std::fmt::Write;
use round1062::io::Scanner;

fn main() {
    let mut sc = Scanner::new();
    let t: usize = sc.next();
    let mut out = String::new();
    for _ in 0..t {
        let n: usize = sc.next();
        let mut a: Vec<u64> = (0..n).map(|_| sc.next()).collect();
        let sort = a.iter().all(|&x| x % 2 == 0) || a.iter().all(|&x| x % 2 == 1);
        if !sort {
            a.sort();
        }
        for num in a {
            write!(&mut out, "{} ", num).unwrap();
        }
        write!(&mut out, "\n").unwrap();
    }
    print!("{}", out);
}