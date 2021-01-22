use cookie_factory::lib::std::io::Write;
use cookie_factory::{combinator::slice, SerializeFn};
use zipr_data::CP437Str;

pub fn cp437_chars<'a, W: 'a>(characters: &'a CP437Str<'a>) -> impl SerializeFn<W> + 'a
where
    W: Write,
{
    let bytes = characters.as_slice();
    slice(bytes)
}
