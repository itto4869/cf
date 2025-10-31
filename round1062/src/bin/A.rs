use round1062::io::Scanner;

fn main() {
    let mut sc = Scanner::new();
    let t: usize = sc.next();
    for _ in 0..t {
        solve(&mut sc);
    }
}

fn solve(sc: &mut Scanner) {
    let x: Vec<u64> = (0..4).map(|_| sc.next()).collect();
    for i in 0..3 {
        if x[i] != x[i + 1] {
            println!("NO");
            return;
        }
    }
    println!("YES");
}