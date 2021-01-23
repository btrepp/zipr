use cookie_factory::lib::std::io::Write;
use cookie_factory::{combinator::slice, SerializeFn};
use zipr_data::borrowed::OEM437Str;

pub fn cp437_chars<'a, W: 'a>(characters: &'a OEM437Str<'a>) -> impl SerializeFn<W> + 'a
where
    W: Write,
{
    let bytes = characters;
    slice(bytes)
}
