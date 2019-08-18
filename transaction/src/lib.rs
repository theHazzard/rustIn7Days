// use std::fmt::Display;
// use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct USD(i32);
#[derive(Debug, PartialEq, Clone)]
pub struct GBP(i32);
#[derive(Debug, PartialEq, Clone)]
pub struct CAD(i32);

pub trait ToUSD {
    fn to_usd(&self) -> USD;
    fn convert<T: FromUSD>(&self) -> T {
        T::from_usd(&self.to_usd())
    }
}

pub struct Ex {
    account_id: i32,
    cad: f32,
    gbp: f32,
}

#[derive(PartialEq, Debug)]
pub struct Transaction<A> {
    from_id: i32,
    to_id: i32,
    amount: A,
}

pub trait Account {
    fn id(&self) -> i32;
}

impl Account for Ex {
    fn id(&self) -> i32 {
        self.account_id
    }
}

pub trait FromUSD {
    fn from_usd(u: &USD) -> Self;
}

pub trait ToUSDv<F> {
    fn to_uv(&self, from: F) -> f32;
}

impl ToUSDv<GBP> for Ex {
    fn to_uv(&self, from: GBP) -> f32 {
        (from.0 as f32) * self.gbp
    }
}

pub trait FromUSDv<F> {
    fn from_uv(&self, to: f32) -> F;
}

impl FromUSDv<CAD> for Ex {
    fn from_uv(&self, from: f32) -> CAD {
        CAD((from/self.cad) as i32)
    }
}

pub trait Exchange <F,T> {
    fn convert(&self, from: F) -> T;
}

impl<E,F,T> Exchange<F,T> for E 
    where E: ToUSDv<F> + FromUSDv<T>{
    fn convert(&self, from: F) -> T {
        self.from_uv(self.to_uv(from))
    }
}
// impl Display for USD {
//     fn fmt(&self, f: &mut fmt::Formatter) ->  fmt::Result {

//     }    
// }

impl ToUSD for GBP {
    fn to_usd(&self) -> USD {
        USD ((self.0*130) / 100)
    }
}

impl FromUSD for CAD {
    fn from_usd(u: &USD) -> CAD {
        CAD((u.0 * 130) / 100)
    }
}


pub trait ExchangeAccount<F,T> {
    fn exchange(&self, from_id: i32, to_id: i32, amount: F) 
        -> (Transaction<F>, Transaction<T>);
}

impl<E,F,T> ExchangeAccount<F,T> for E 
    where E: Exchange<F,T> + Account,
        F: Clone {
    fn exchange(&self, from_id: i32, to_id: i32, amount: F) 
        -> (Transaction<F>, Transaction<T>) {
            let ft = Transaction { from_id: from_id, to_id: self.id(), amount: amount.clone() };
            let tt = Transaction { from_id: self.id(), to_id: to_id, amount: self.convert(amount)};
            (ft, tt)
        }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let ex = Ex { account_id: 30, cad: 0.7, gbp: 1.3};
        let (ft, tt) = ex.exchange(20, 40, GBP(200));
        assert_eq!(ft, Transaction { from_id: 20, to_id: 30, amount: GBP(200)});
        assert_eq!(tt, Transaction { from_id: 30, to_id: 40, amount: CAD(371)});
    }
}
