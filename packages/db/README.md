# db package
kv contains models of key-value storage of internal data structure of Kardiachain.
`libmdbx` is used in this kv storage.

This package defines low-level data structure of `kv`, `state`, `bitmapdb` and `trie`, and provides high-level APIs for reading/writing: state, block...