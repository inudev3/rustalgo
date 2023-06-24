use std::io;

fn read_input() -> (usize, usize, usize) {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut iter = buffer.split_whitespace();
    let n = iter.next().unwrap().parse().unwrap();
    let m = iter.next().unwrap().parse().unwrap();
    let k = iter.next().unwrap().parse().unwrap();
    (n, m, k)
}

fn min(a: usize, b: usize) -> usize {
    if a < b { a } else { b }
}

fn get_kth_string(n: usize, m: usize, k: usize, comb: &Vec<Vec<usize>>) -> String {
    if n == 0 {
        return "z".repeat(m);
    }
    if m == 0 {
        return "a".repeat(n);
    }
    if k <= comb[n + m - 1][n - 1] {
        format!("a{}", get_kth_string(n - 1, m, k, comb))
    } else {
        format!("z{}", get_kth_string(n, m - 1, k - comb[n + m - 1][n - 1], comb))
    }
}

fn main() {
    let (n, m, k) = read_input();
    let max = n + m + 1;
    let mut comb = vec![vec![0; max]; max];
    comb[0][0] = 1;
    for i in 1..max {
        comb[i][0] = 1;
        for j in 1..=i {
            comb[i][j] = min(comb[i - 1][j - 1] + comb[i - 1][j], k + 1);
        }
    }
    if k > comb[n + m][n] {
        println!("-1");
    } else {
        println!("{}", get_kth_string(n, m, k, &comb));
    }
}
