// https://atcoder.jp/contests/abc317/tasks/abc317_f

use proconio::input;
use proconio::fastout;
use num::integer::lcm;

const MOD: usize = 998_244_353;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A1: usize,
        A2: usize,
        A3: usize,
    }
    let M = 60;
    let flg = [[[0, 1], [0, 0]], [[1, 1], [0, 1]]];
    let mut DP = vec![vec![vec![vec![vec![vec![vec![0; A3]; A2]; A1]; 2]; 2]; 2]; M+1];
    DP[0][1][1][1][0][0][0] = 1;
    for i in 0..M {
        let n = 1 << i;
        let bit = match N & n {
            0 => 0,
            _ => 1,
        };
        for j1 in 0..2 {
            for j2 in 0..2 {
                for j3 in 0..2 {
                    for k1 in 0..2 {
                        for k2 in 0..2 {
                            for k3 in 0..2 {
                                for l1 in 0..A1 {
                                    for l2 in 0..A2 {
                                        for l3 in 0..A3 {
                                            if (k1 + k2 + k3) % 2 == 0 {
                                                DP[i+1][flg[j1][k1][bit]][flg[j2][k2][bit]][flg[j3][k3][bit]][(l1+k1*n)%A1][(l2+k2*n)%A2][(l3+k3*n)%A3] += DP[i][j1][j2][j3][l1][l2][l3];
                                                DP[i+1][flg[j1][k1][bit]][flg[j2][k2][bit]][flg[j3][k3][bit]][(l1+k1*n)%A1][(l2+k2*n)%A2][(l3+k3*n)%A3] %= MOD;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    let mut ans = DP[M][1][1][1][0][0][0];
    ans = (ans + MOD - (N / lcm(A1, A2)) % MOD) % MOD;
    ans = (ans + MOD - (N / lcm(A1, A3)) % MOD) % MOD;
    ans = (ans + MOD - (N / lcm(A2, A3)) % MOD) % MOD;
    ans = (ans + MOD - 1) % MOD;
    println!("{}", ans);
}