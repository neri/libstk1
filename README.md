# LIBSTK1

A compatible library for subset of stk1

* [Documentation](https://neri.github.io/libstk1/libstk1/)

# CAUTION

**THIS LIBRARY IS AN ALPHA VERSION**.
Compression and decompression itself is possible, but you will need to provide your own processing for file headers and data size outside the library.

# Feature

* stk1 is a simple and easy to decompress compression format.
* Since it uses only LZ compression, it is inferior in compression ratio to other formats that use entropy compression together, but I believe its merit is that it is simple, so decompression programs can be written compactly.
* Since no formal specification exists, this implementation is based on reverse engineering of the decoder.

The following _incompatibilities_ exist:
* The various limits are not official values.
* Only compressed data is supported; headers are not.

# Original specifications

(C) Kawai Hidemi

Related Documents: <http://osask.net/w/196.html> (But different from known final specifications)
