use std::ops::AddAssign;
use std::cmp::PartialOrd;

pub struct Stepper<T> {
    curr: T,
    step: T,
    stop: T,
}

impl<T> Stepper<T> {
    pub fn new(start: T, stop: T, step: T) -> Self {
        Stepper {
            curr: start,
            step: step,
            stop: stop,
        }
    }
}

impl<T> Iterator for Stepper<T> 
    where T: AddAssign + Copy + PartialOrd {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.curr >= self.stop {
            return None;
        }

        let res = self.curr;
        self.curr += self.step;

        Some(res)
    }
}

pub fn sum_list<I, S>(list: I, mut start: S) -> S
    where I: Iterator<Item=S>,
        S: AddAssign, {
    
    let mut it = list.into_iter();
    while let Some(n) = it.next() {
        start += n;
    }
    start
    // for number in list {
    //     start += number;
    // }
    // start
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut c = 0;
        for n in Stepper::new(2, 10, 2) {
            c += n;
        }

        assert_eq!(c, 20);
        let sl = sum_list(Stepper::new(3, 10, 2), 0);

        assert_eq!(sl, 24);

        let fl = Stepper::new(4, 10, 2).fold(0, |acc, x| acc + x);
        assert_eq!(fl, 18);
    }
}
