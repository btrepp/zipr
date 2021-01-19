use core::{convert::TryFrom, marker::PhantomData};
use typenum::Unsigned;

/// A range limited number from lower to upper inclusive
/// used to verify we are inside the range
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Bounded<Lower: Unsigned, Upper: Unsigned> {
    value: u8,
    _lower: PhantomData<Lower>,
    _upper: PhantomData<Upper>,
}

#[derive(Debug, PartialEq)]
pub struct OutOfRangeError {
    input: u8,
    lower: u8,
    upper: u8,
}

impl<L, U> From<Bounded<L, U>> for u8
where
    L: Unsigned,
    U: Unsigned,
{
    fn from(x: Bounded<L, U>) -> Self {
        x.value
    }
}

impl<L, U> TryFrom<u8> for Bounded<L, U>
where
    U: Unsigned,
    L: Unsigned,
{
    type Error = OutOfRangeError;

    fn try_from(input: u8) -> Result<Bounded<L, U>, Self::Error> {
        let lower = <L>::to_u8();
        let upper = <U>::to_u8();
        if lower <= input && input <= upper {
            Ok(Bounded {
                value: input,
                _lower: PhantomData,
                _upper: PhantomData,
            })
        } else {
            Err(OutOfRangeError {
                input,
                lower,
                upper,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use core::convert::TryInto;

    use super::*;
    use typenum::{U0, U10};

    #[test]
    fn test_mod10_valid_1() {
        let _: Bounded<U0, U10> = 1.try_into().unwrap();
    }

    #[test]
    fn test_mod10_valid_0() {
        let _: Bounded<U0, U10> = 0.try_into().unwrap();
    }

    #[test]
    fn test_mod10_invalid_11() {
        let result: Result<Bounded<U0, U10>, OutOfRangeError> = 11.try_into();
        let expected = OutOfRangeError {
            input: 11,
            lower: 0,
            upper: 10,
        };
        assert_eq!(Err(expected), result);
    }

    #[test]
    fn test_mod10_invalid_20() {
        let result: Result<Bounded<U0, U10>, OutOfRangeError> = 20.try_into();
        let expected = OutOfRangeError {
            input: 20,
            lower: 0,
            upper: 10,
        };
        assert_eq!(Err(expected), result);
    }
}
