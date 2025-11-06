use gr30::io::Scanner;

fn main() {
    let mut sc = Scanner::new();

    let t: usize = sc.next();
    let mut ans = String::new();
    for _ in 0..t {
        let n = sc.next();
        let m = sc.next();
        let mut sword = Vec::with_capacity(n);
        let mut monster = Vec::with_capacity(m);
        let mut monster_c = Vec::with_capacity(m);
        for _ in 0..n {
            let a: u64 = sc.next();
            sword.push(a);
        }
        for _ in 0..m {
            let b: u64 = sc.next();
            monster.push(b);
        }
        for _ in 0..m {
            let c: u64 = sc.next();
            monster_c.push(c);
        }

        let mut monster_and_c = Vec::with_capacity(m);
        let mut monster_only = Vec::with_capacity(m);
        for (&b, c) in monster.iter().zip(monster_c) {
            if c > 0 {
                monster_and_c.push((b, c));
            } else {
                monster_only.push(b);
            }
        }

        monster_and_c.sort_by(|a, b| a.1.cmp(&b.1));
        monster_and_c.reverse();
        monster_only.sort();
        monster_only.reverse();
        sword.sort_unstable();
        sword.reverse();

        let mut i = 0;
        let mut j = 0;
        let mut cnt = 0;
        while j < monster_and_c.len() {
            let x = sword[i];
            let (y, c) = monster_and_c[j];
            if x < y {
                j += 1;
                continue;
            } else {
                j += 1;
                sword[i] = x.max(c);
                cnt += 1;
            }
        }
        j = 0;
        while j < monster_only.len() && i < n {
            let x = sword[i];
            let y = monster_only[j];
            if x < y {
                j += 1;
                continue;
            } else {
                j += 1;
                i += 1;
                cnt += 1;
            }
        }

        ans.push_str(&format!("{}\n", cnt));
    }
    print!("{}", ans);
}