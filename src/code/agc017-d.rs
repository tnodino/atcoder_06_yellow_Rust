// https://atcoder.jp/contests/agc017/tasks/agc017_d

use proconio::input;
use proconio::fastout;

#[allow(non_snake_case)]
fn dfs(pos: usize, pre: usize, G: &Vec<Vec<usize>>) -> usize {
    let mut ret = 0;
    for nxt in G[pos].iter() {
        if *nxt == pre {
            continue;
        }
        ret ^= dfs(*nxt, pos, &G) + 1;
    }
    return ret;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut G = vec![Vec::new(); N];
    for _ in 0..N-1 {
        input! {
            x: usize,
            y: usize,
        }
        G[x-1].push(y-1);
        G[y-1].push(x-1);
    }
    if dfs(0, N, &G) > 0 {
        println!("Alice");
    }
    else {
        println!("Bob");
    }
}