# bioutils
Utilities for working with biological sequences. Functionality is very similar to biopython. This crate is probably:

  <ul>Slower than pure u8 based counterparts </ul>
  <ul>tolerant of errors in strings, variable CIGAR/name strings </ul>
  <ul>does what you expect over punctuation and special characters </ul>


The main use case for this crate is to preprocess and annotate sequences while being robust to errors caused by assuming ASCII or IUPAC encoding. If the sequence has a new or corrupt character, these utilities may be able to replace them. Sequences can then be turned into u8 representations for downstream processes.
