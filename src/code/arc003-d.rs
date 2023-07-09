// https://atcoder.jp/contests/arc003/tasks/arc003_4

use proconio::input;
use rand::Rng;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        K: usize,
    }
    let mut NG = vec![vec![false; N]; N];
    for _ in 0..M {
        input! {
            A: usize,
            B: usize,
        }
        NG[A][B] = true;
        NG[B][A] = true;
    }
    let T = 2_500_000;
    let mut cnt = 0;
    'outer: for _ in 0..T {
        let mut vec = (0..N).collect::<Vec<usize>>();
        for _ in 0..K {
            let idx1 = rand::thread_rng().gen_range(0, N);
            let mut idx2 = rand::thread_rng().gen_range(0, N);
            while idx1 == idx2 {
                idx2 = rand::thread_rng().gen_range(0, N);
            }
            vec.swap(idx1, idx2);
        }
        vec.push(vec[0]);
        for i in 0..N {
            if NG[vec[i]][vec[i+1]] {
                continue 'outer;
            }
        }
        cnt += 1;
    }
    println!("{}", (cnt as f64) / (T as f64));
}