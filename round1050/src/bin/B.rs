use round1050::io::Scanner;

fn main() {
    let mut sc = Scanner::new();
    let t: usize = sc.next();
    let mut out = String::new();
    for _case in 0..t {
        // 問題に応じて入力を読む
        let n: usize = sc.next();
        let mut ans: i64 = 0;
        for _ in 0..n {
            let x: i64 = sc.next();
            ans += x;
        }
        out.push_str(&format!("{}\n", ans));
    }
    print!("{}", out);
}