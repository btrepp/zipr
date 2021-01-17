# zipr

Zip in rust

This is pure rust implemenation of manipulating zip files.
NOTE: Very much alpha. Data structures are most likely designed to change shape.

## Components

### zipr

Meta crate making the other crates easier to use

### zipr-data

The core data structures. Hopefully this can be re-useable to other crates even if the parsers/serializers aren't.
Note this is no-std. So is very minimal

### zipr-nom

Converts [u8] -> to Zipr-core data structures

### zipr-cookie

The opposite of zipr nom, convert zipr-data structures into serializes using cookie-factory

### zipr-compression

Crate repsonsible for decompressing and compressing to CompressedData<'a> data structures.
The library is somewhat useable without this, assuming you provide you own implementations 
for compression.

### zipr-std

Helper traits and functions to make zipr easier to use if you are in a std environment.
Eg converts zip paths into Path structures, and converts dos times into chrono based times

### zipr-cli

A cli tool that allows you to manipulate zip files.
This tool is probably not as useful as existing unzip/zip tools, but it provies examples of how to use some
of the structures and helps test that they provide useful interfaces.

## Aims

This was inspired by the nom parsing library, and that there wasn't a zip implementation.

Ideally we provide a nice parser so other zip-like formats can use this as a combinator, in creating there own parsers (eg I believe other formats are just zipped xml).

## Features

- Zipr cli can extract and list files. 
- Zipr cli pretends to be unzip if aliased as unzip
- Most types are implemented in some form
- Data driven
- No standard support for core data structures

## Features that need to be implemented

- Some more ergonomic functions
- MMap to speed up the zipr-cli on large files.


## Contributing

Pull requests are welcome. Though raising an issue first is probably preferred.
If there is a zip file that fails to parse. A test utilizing the minimal case would be greatly appreciated.

A good contribution is small sample files for the assets folder. Zip isn't always implemented consistently, so 
while I have tried to follow the spec, other implementations don't really. The more examples we can test
against, the better we can make it compatible