use ascii::AsciiStr;
use cookie_factory::lib::std::io::Write;
use cookie_factory::{combinator::slice, SerializeFn};

pub fn ascii_chars<'a, W: 'a>(characters: &'a AsciiStr) -> impl SerializeFn<W> + 'a
where
    W: Write,
{
    let bytes = characters.as_bytes();
    slice(bytes)
}
