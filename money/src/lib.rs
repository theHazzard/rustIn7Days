#[derive(Debug, PartialEq, Clone)]
pub struct USD(i32);
#[derive(Debug, PartialEq, Clone)]
pub struct GBP(i32);
#[derive(Debug, PartialEq, Clone)]
pub struct CAD(i32);

#[derive(PartialEq, Debug)]
pub struct Transaction<A> {
    from_id: i32,
    to_id: i32,
    amount: A,
}

pub trait Account {
    fn id(&self) -> i32;
}
pub trait ToUSDv<F> {
    fn to_uv(&self, g: F) -> f32;
}

pub trait FromUSDv<F> {
    fn from_uv(&self, v: f32) -> F;
}

pub struct Ex {
    account_id: i32,
    cad: f32,
    gbp: f32,
}

impl Account for Ex  {
    fn id(&self) -> i32 {
        self.account_id
    }
}

pub trait ExchangeAccount<F, T> {
    fn exchange(&self, f_id: i32, t_id: i32, amount: F) 
        -> (Transaction<F>, Transaction<T>);
}

impl<E, F, T> ExchangeAccount<F, T> for E 
    where E: Exchange<F, T> + Account,
        F: Clone {
    
    fn exchange(&self, f_id: i32, t_id: i32, amount: F) -> (Transaction<F>, Transaction<T>) {
        let ft = Transaction { from_id: f_id, to_id: self.id(), amount: amount.clone() };
        let tt = Transaction { from_id: self.id(), to_id: t_id, amount: self.convert(amount) };
        (ft, tt)
    }
}

impl ToUSDv<GBP> for Ex {
    fn to_uv(&self, g: GBP) -> f32 {
        (g.0 as f32) * self.gbp
    }
}

impl FromUSDv<CAD> for Ex {
    fn from_uv(&self, a: f32) -> CAD {
        CAD((a / self.cad) as i32)
    }
}

pub trait Exchange<F, T> {
    fn convert(&self, f: F) -> T;
}

impl<E, F, T> Exchange<F, T> for E
    where E: ToUSDv<F> + FromUSDv<T> {

    fn convert(&self, f:F) -> T {
        self.from_uv(self.to_uv(f))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let ex = Ex { account_id: 30, cad: 0.7, gbp: 1.3 };
        
        let (ft, tt) = ex.exchange(20, 40, GBP(200));

        assert_eq!(ft, Transaction { from_id: 20, to_id: 30, amount: GBP(200)});
        assert_eq!(tt, Transaction { from_id: 30, to_id: 40, amount: CAD(371)});
    }
}
