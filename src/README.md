# Bioutils

Please do not use this package yet, as it is still in very early development. Below are the goals of this package, please feel free to contribute at the github. Currently, the alphabet sections are correct and can be used, but the functions are still in progress.
 Utilities for working with biological sequences. The overall goal of this create is to have functionality similar to biopython. This crate will probably be:

   <ul>Slower than pure u8 based counterparts </ul>
   <ul>tolerant of errors in strings, variable CIGAR/name strings </ul>
   <ul>does what you expect over punctuation and special characters </ul>


 The main use case for this crate is to preprocess and annotate sequences while being robust to errors caused by assuming ASCII or IUPAC encoding. If the sequence has a new or corrupt character, these utilities may be able to check for or replace them. Sequences can then be turned into u8 representations for downstream processes.

 Current functionality is based on return values: 

   <ul>Replace UTF8 characters with pseudorandom nucleotides (ACTG, case-sensitive)</ul>
   <ul>Check whether the sequence is a homopolymer based on input value (boolean) </ul>
   <ul>create new pseudorandom sequences based on IUPAC designations </ul>

 Current [IUPAC](https://www.bioinformatics.org/sms/iupac.html) designations
