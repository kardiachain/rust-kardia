use crate::types::round::RoundStep;

pub fn compare_hrs(h1: u64, r1: u32, s1: RoundStep, h2: u64, r2: u32, s2: RoundStep) -> isize {
    if h1 < h2 {
        return -1;
    } else if h1 > h2 {
        return 1;
    }
    if r1 < r2 {
        return -1;
    } else if r1 > r2 {
        return 1;
    }
    if s1.lt(&s2) {
        return -1;
    } else if s1.gt(&s2) {
        return 1;
    }
    return 0;
}
