use gr30::io::Scanner;

fn main() {
    let mut sc = Scanner::new();
    // 例: n を読んで、配列 a を読む
    let t: usize = sc.next();
    let mut ans = String::new();
    for _ in 0..t {
        let n: usize = sc.next();
        let mut min_a = i64::MAX;
        let mut max_a = i64::MIN;
        for _ in 0..n {
            let a = sc.next();
            min_a = min_a.min(a);
            max_a = max_a.max(a);
        }
        let x: i64 = sc.next();
        if x >= min_a && x <= max_a {
            ans.push_str("YES\n");
        } else {
            ans.push_str("NO\n");
        }
    }
    print!("{}", ans);
}