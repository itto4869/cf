use round1050::io::Scanner;

fn main() {
    let mut sc = Scanner::new();
    let t: usize = sc.next();
    let mut ans = String::new();
    for _ in 0..t {
        let x: usize = sc.next();
        let n: usize = sc.next();
        if n % 2 == 0 {
            ans.push_str("0\n");
        } else {
            ans.push_str(&format!("{}\n", x));
        }
    }
    print!("{}", ans);
}