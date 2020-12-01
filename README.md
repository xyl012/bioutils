# bioutils
Utilities for working with biological sequences. Functionality is very similar to biopython. This crate is probably:

  <ul>Slower than pure u8 based counterparts </ul>
  <ul>tolerant of errors in strings, variable CIGAR/name strings </ul>
  <ul>does what you expect over punctuation and special characters </ul>


The main use case for this crate is to preprocess and annotate sequences while being robust to errors caused by assuming ASCII or IUPAC encoding. If the sequence has a new or corrupt character, these utilities may be able to replace them. Sequences can then be turned into u8 representations for downstream processes.

Currently functionality is based on return values: 

  <ul>Replace UTF8 characters with pseudorandom nucleotides (ACTG, case-sensitive) (returns a string or str) </ul>
  <ul>Check whether the sequence is a homopolymer, palindrome, etc. based on input value (boolean) </ul>
  <ul>Convert string/str to u8 or u8 to string/str (ASCII is same as UTF8 for ASCII characters) </ul>
  <ul>create new pseudorandom sequences based on IUPAC designations </ul>
  
Current [IUPAC](https://www.bioinformatics.org/sms/iupac.html) designations
