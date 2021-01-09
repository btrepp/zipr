fn seq<T, E>(it: impl Iterator<Item = Result<T, E>>) -> Result<Vec<T>, E> {
    let mut result = Vec::new();
    for ts in it {
        match ts {
            Ok(i) => result.push(i),
            Err(e) => return Err(e),
        }
    }
    Ok(result)
}

pub trait Sequence<I, E, T: Iterator<Item = Result<I, E>>> {
    fn sequence(self) -> Result<Vec<I>, E>;
}

impl<I, E, T: Iterator<Item = Result<I, E>>> Sequence<I, E, T> for T {
    fn sequence(self) -> Result<Vec<I>, E> {
        seq(self)
    }
}
