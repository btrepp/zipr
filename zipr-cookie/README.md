# zipr-cookie

Serializers written using cookie-factory.
These are capable of writing to anything that implements
the write trait. 

Most consumers will wish to use the file serializer,
which will layout and serialize a iterator of zip entries.

The low level serializers are public so that non-standard use
cases can be created