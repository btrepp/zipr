# zipr-data

The core zipr data structures. This crate is no-std and doesn't alloc.
This should make it compatible with any platform. There is no special dependencies here.

## borrow

Because we are no alloc, most useful data structures are in borrow.
These are scoped to the lifetime of the bytes that created them.

It can help to think of these are 'views' or 'pointers' over the supplied
bytes, they can only exist while the bytes do.

This does give a benefit that they are incredibly lightweight, and can be copied
as long as the bytes are around.