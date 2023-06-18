use adder::*;

pub fn multiplier(a: i32, b: i32) -> i32 {
    let mut res = 0;
    for _i in 0..b {
        res = adder(res, a);
    }
    res
}
