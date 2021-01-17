# zipr-nom

Parsers for zipr-data structures written using nom.
These allow us to parse bytes into the 'borrow' data structures.

The most useful is the ZipEntry iterator parser. Which returns
a iterator of zipentries. This is evaluated lazily when the consumer iterates.
Leaving flow control and resource usage up to the consumer of the zip file.

It should be relatively fast to skip over unwanted files, as they are not decompressed,
only the metadata is parsed.