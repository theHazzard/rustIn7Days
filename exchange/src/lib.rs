use std::fmt::Display;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct USD(i32);
#[derive(Debug, PartialEq)]
pub struct GBP(i32);
#[derive(Debug, PartialEq)]
pub struct CAD(i32);

pub trait ToUSD {
    fn to_usd(&self) -> USD;
    fn convert<T: FromUSD>(&self) -> T {
        T::from_usd(&self.to_usd())
    }
}

pub trait FromUSD {
    fn from_usd(u: &USD) -> Self;
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


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let g = GBP(200);
        let u = g.to_usd();

        assert_eq!(u, USD(260));

        let c = CAD::from_usd(&u);
        assert_eq!(c, CAD(338));

        let c2: CAD = g.convert();
        assert_eq!(c2, c);
    }
}
