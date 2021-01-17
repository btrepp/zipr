# zipr-cli

Command line using the zipr libraries

This aims to be a pure zip implementation using rust.

NOTE: Very much alpha. Data structures are most likely designed to change shape.

## Commands

### list

`zipr list file.zip`

Lists out the contents of the zip file

### add

`zipr add file.zip file_to_add.txt`

Adds file_to_add.txt to the file.zip zip file.
Will crate the file if it doesn't exist

### extract

`zipr extract file.zip [files] -o outputfolder`

Extract files from file.zip into output folder.
If files aren't specified it's all files
If output folder isn't specified it is the current folder


### show-comment

`zipr show-comment file.zip` 

Print the zip file command to stdou

### inspect

`zipr inspect file.zip --kind kind --offset x`

Parse the offset into the low level datstructure kind and
print the result to std out. This is useful for debugging
why zip-files failed to parse, or serialized incorrectly

## Alias compatibility

If the binary is alias (symlinked) as unzip or zip, it provides
a simple compatibility layer to those programs. It potentially
could be used in place of them. Though not all flags are supported

