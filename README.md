# bioutils
Utilities for working with biological sequences based on UTF8 encoded strings. Functionality is very similar to biopython, which also encodes strings as UTF8. This crate is probably:

  <ul>Slower than u8 based counterparts </ul>
  <ul>tolerant of errors in strings, variable CIGAR/name strings </ul>
  <ul>does what you expect over punctuation and special characters </ul>


The main use case for this crate is to preprocess and annotate sequences that may be problematic for other Rust libraries at the string level, which may then be turned into u8 representations for processing, etc.
