// https://atcoder.jp/contests/abc295/tasks/abc295_e

use proconio::input;
use proconio::fastout;

const MOD: usize = 998_244_353;

#[allow(non_snake_case)]
fn pascal_triangle(N: usize) -> Vec<Vec<usize>> {
    let mut ret = vec![vec![0; N+1]; N+1];
    for i in 0..=N {
        for j in 1..=i {
            ret[i][j] = ret[i-1][j] + ret[i-1][j-1];
            ret[i][j] %= MOD;
        }
    }
    return ret;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        K: usize,
        A: [usize; N],
    }
    let comb = pascal_triangle(N);
    let mut ans = 0;
    for m in 1..=M {
        let mut cnt1 = 0;
        let mut cnt2 = 0;
        for a in A.iter() {
            if *a == 0 {
                cnt1 += 1;
            }
            if m <= *a {
                cnt2 += 1;
            }
        }
        if cnt2 >= N - K + 1 {
            ans += 1;
            continue;
        }
        if cnt1 + cnt2 < N - K + 1 {
            continue;
        }
    }
    println!("{}", ans % MOD);
}