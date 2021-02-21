use std::cmp::Reverse;

macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let mut s = {
            use std::io::Read;
            let mut s = String::new();
             std::io::stdin().read_to_string(&mut s).unwrap();
             s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

fn main() {
    input!{
        n: i32,
        k: i64,
    }
    let mut ans = n;

    for i in 0..k {
        let v = toVector(ans);
        let mut g1_v = v.clone();
        let mut g2_v = v.clone();
        g1_v.sort();
        g2_v.sort_by_key(|&x| Reverse(x));
        let mut g1 = toInt(g1_v);
        let mut g2 = toInt(g2_v);
        ans = g1 - g2;
    }
    println!("{}",ans);
}

fn toVector(mut n: i32) -> std::vec::Vec<i32> {
    let mut d = Vec::new();
    while n != 0 {
        d.push(n%10);
        n /= 10;
    }
    d
}

fn toInt(v: std::vec::Vec<i32>) -> i32 {
    let mut n = 0;
    let mut i = 0;
    for d in v {
        n += d * 10_i32.pow(i);
        i += 1;
    }
    n
}