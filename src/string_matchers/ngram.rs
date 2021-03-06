use std::cmp;
use std::convert::TryInto;
use crate::string_matchers::hamming;

pub fn compute(source: &str, target: &str, ngram_size: i32) -> f64 {
    let special = '\n';
    let source_len = source.len();
    let target_len = target.len();

    let mut cost;

    if source == target { return 0.0; }

    if source_len == 0 || target_len == 0 {
        return 1.0;
    }

    // if either smaller than ngram size, just do hamming
    if source_len < ngram_size.try_into().unwrap() || target_len < ngram_size.try_into().unwrap() {
        return (hamming::compute(source, target)/cmp::max(source_len as i32, target_len as i32)) as f64;
    }

    let n = ngram_size as usize;
    let sa_length = source_len + n - 1;
    let mut sa = vec![special; sa_length];
    let mut _swapper: Vec<f64> = Vec::new();

    // construct sa
    for (i, item) in sa.iter_mut().enumerate() {
        if i < n - 1 {
            *item = special;    // prefix
        } else {
            let value = (i as i32) - (n as i32) + 1;
            let source_char = source.chars().nth(value as usize).unwrap();
            *item = source_char;
        }
    }

    let mut p: Vec<f64> = vec![0.0; source_len+1];
    let mut d: Vec<f64> = vec![0.0; source_len+1];

    // jth ngram of target
    let mut t_j = vec![special; n];

    for (i, item) in p.iter_mut().enumerate().take(source_len+1){
        *item = i as f64;
    }

    for j in 1..target_len+1 {
        if j < n {
            for item in t_j.iter_mut().take(n-j) {
                *item = special;
            }
            for (ti, item) in t_j.iter_mut().enumerate().take(n).skip(n-j) {
                let target_char = target.chars().nth(ti-(n-j)).unwrap();
                *item = target_char;
            }
        } else {
            t_j = target[(j-n)..j].chars().collect();
        }

        d[0] = j as f64;

        for i in 1..source_len+1 {
            cost = 0;
            let mut tn = n;
            // compare sa to t_j
            for ni in 0..n {
                if sa[i-1+ni] != t_j[ni] {
                    cost+=1;
                } else if sa[i-1+ni] == special {
                    // discount matches on prefix
                    tn-=1;
                }
            }
            let ec = (cost/tn) as f64;
            d[i] = min_f64(
                min_f64(d[i-1]+1.0, p[i]+1.0),
                p[i-1]+ec
            );
        }

        _swapper = p;
        p = d;
        d = _swapper;
    }

    p[source_len] / cmp::max(target_len, source_len) as f64
}

fn min_f64(x: f64, y: f64) -> f64 {
    if x < y {
        x
    } else {
        y
    }
}