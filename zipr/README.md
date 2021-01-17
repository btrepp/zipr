# zipr

A metapackage bringing all the zipr packages together for ease of use.


## Features

Features can be disabled if not used. This is allows you to prune
of sections if you don't need them. Eg if you only ever deal with stored
files, you don't need compression, if you only plan to serialize from 
your program, you don't need parser support

| Feature        | Purpose                                           | 
| :------------- | :-----------------------------------------------: |
| std            | std lib support. Conversions and other sugar      |
| nom            | parser support using nom                          | 
| compression    | decompress/compresss support using miniz-oxide    | 
| cookie-factory | serialization using cookie-factory                |