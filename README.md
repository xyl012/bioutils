# bioutils
Utilities for working with UTF8 encoded biological sequences. Functionality is very similar to biopython, with additional functionality to work with u8 data downstream. This crate is probably:

  <ul>Slower than u8 based counterparts </ul>
  <ul>tolerant of errors in strings, variable CIGAR/name strings </ul>
  <ul>does what you expect over punctuation and special characters </ul>


The main use case for this crate is to preprocess and annotate problematic sequences for other Rust libraries at the string level. These sequences may then be turned into u8 representations for processing, etc.
