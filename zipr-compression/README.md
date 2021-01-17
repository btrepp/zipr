# zipr-compression

This handles the compression and decompression
for the zipr library. Compression is handled as a seperate
stage from parsing the zip structure, as you can pick and 
choose what to decompress or compress. Allowing it to be optimized
for specific use-cases