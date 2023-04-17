// https://atcoder.jp/contests/agc004/tasks/agc004_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
    }
    let mut a = Vec::new();
    for _ in 0..H {
        input! {
            s: String,
        }
        let s = s.chars().collect::<Vec<char>>();
        a.push(s);
    }
    let mut red = a.clone();
    let mut blue = a.clone();
    for i in 0..H {
        red[i][0] = '#';
        blue[i][W-1] = '#';
        if i % 2 == 0 {
            for j in 1..W-1 {
                red[i][j] = '#';
            }
        }
        else {
            for j in 1..W-1 {
                blue[i][j] = '#';
            }
        }
    }
    for i in 0..H {
        println!("{}", red[i].iter().collect::<String>());
    }
    println!();
    for i in 0..H {
        println!("{}", blue[i].iter().collect::<String>());
    }
}