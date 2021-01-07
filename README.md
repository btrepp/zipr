# zipr
Rust zip library using nom

This aims to be a pure zip implementation using rust.

NOTE: Very much alpha. Data structures are most likely designed to change shape.

## Components

### Zipr

Meta crate making the other crates easier to use

### Zipr-core 

The core data structures. Hopefully this can be re-useable to other crates even if the parsers/serializers aren't.

### Zipr-nom

Converst [u8] -> to Zipr-core data structures

### Zipr-cli

A cli tool that allows you to manipulate zip files.
This tool is probably not as useful as existing unzip/zip tools, but it provies examples of how to use some
of the structures and helps test that they provide useful interfaces.

## Aims

This was inspired by looking at the nom parsing library, and that there wasn't a zip implementation.
So hopefully this fills that gap for that library.

Ideally we provide a nice parser so other zip-like formats can use this as a combinator, in creating there own parsers (eg I believe other formats are just zipped xml).

Features that need to be implemented
- Other compression types from store/deflate
- Serialization, using cookie-factory
- no std support, nom + miniz-oxide. Should help the code be super portable
- Further refinements of the data types
- Some more ergonomic functions
- MMap to speed up the zipr-cli on large files.

## Features

- Zipr cli can extract and list files. 
- Zipr cli pretends to be unzip if aliased as unzip
- Most types are implemented in some form
- Data driven
