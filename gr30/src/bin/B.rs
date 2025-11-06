use gr30::io::Scanner;

fn main() {
    let mut sc = Scanner::new();
    let t: usize = sc.next();
    let mut ans = String::new();
    for _ in 0..t {
        let n: usize = sc.next();
        let mut a: Vec<u64> = Vec::with_capacity(n);
        let mut ok = false;
        for i in 0..n {
            let y: u64 = sc.next();
            if ok {
                continue;
            }
            for j in 0..i {
                let k = y % a[j];
                if k % 2 == 0 {
                    ans.push_str(&format!("{} {}\n", a[j], y));
                    ok = true;
                    break;
                }
            }
            a.push(y);
        }
        if !ok {
            ans.push_str("-1\n");
        }
    }
    print!("{}", ans);
}