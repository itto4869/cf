use round1062::io::Scanner;

fn main() {
    let mut sc = Scanner::new();
    let q: usize = sc.next();
    let mut out = String::new();
    for _case in 0..q {
        let n: usize = sc.next();
        let mut name1 = sc.bytes(n).to_vec();
        let mut name2 = sc.bytes(n).to_vec();
        name1.sort_unstable();
        name2.sort_unstable();
        if name1 == name2 {
            out.push_str("YES\n");
        } else {
            out.push_str("NO\n");
        }
    }
    print!("{}", out);
}