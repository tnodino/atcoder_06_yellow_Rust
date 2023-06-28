// https://atcoder.jp/contests/abc003/tasks/abc003_4

const MOD: usize = 1_000_000_007;

fn bin_power(mut x: usize, mut k: usize) -> usize {
    let mut ret = 1;
    while k > 0 {
        if k & 1 > 0 {
            ret = (ret * x) % MOD;
        }
        x = (x * x) % MOD;
        k >>= 1;
    }
    return ret;
}

pub struct Combination {
    fact: Vec<usize>,
    factinv: Vec<usize>,
}

impl Combination {
    pub fn new(n: usize) -> Self {
        let mut fact = vec![0; n+1];
        let mut factinv = vec![0; n+1];
        fact[0] = 1;
        factinv[0] = 1;
        for i in 1..=n {
            fact[i] = (fact[i-1] * i) % MOD;
            factinv[i] = bin_power(fact[i], MOD-2);
        }
        Self {
            fact,
            factinv,
        }
    }

    pub fn ncr(&mut self, n: usize, r: usize) -> usize {
        if n < r {
            return 0;
        }
        return ((self.fact[n] * self.factinv[r]) % MOD * self.factinv[n-r]) % MOD;
    }
}

use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        R: usize,
        C: usize,
        X: usize,
        Y: usize,
        D: usize,
        L: usize,
    }
    let N = X * Y;
    let mut Comb = Combination::new(N);
    let mut ans = 0;
    'outer: for bit in 0..1<<4 {
        let mut x = X;
        let mut y = Y;
        let mut cnt = 0;
        for i in 0..4 {
            if bit & (1 << i) > 0 {
                cnt += 1;
                if i % 2 == 0 {
                    if x == 0 {
                        continue 'outer;
                    }
                    x -= 1;
                }
                else {
                    if y == 0 {
                        continue 'outer;
                    }
                    y -= 1;
                }
            }
        }
        let res = (Comb.ncr(x * y, D + L) * Comb.ncr(D + L, D)) % MOD;
        if cnt % 2 == 0 {
            ans += res;
        }
        else {
            ans += MOD - res;
        }
        ans %= MOD;
    }
    ans *= (R - X + 1) * (C - Y + 1) % MOD;
    ans %= MOD;
    println!("{}", ans);
}