#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut count = 0;
    // let mut prev_colour = s[0];
    for i in 0..s.len() - 1 {
        if s[i] != s[i + 1] {
            count += 1;
            // prev_colour = s[i + 1];
        }
    }
    println!("{}", count);
}
