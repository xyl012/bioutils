// Copyright 2020 Christopher Sugai
//! Sections of ASCII Codes are used in different types of quality encodings. Here, each encoding's possible characters are placed into a type of 'alphabet'. 
//! In our example, these are used to generate pseudorandom quality strings, but in practice should be used to interconvert between scores.
//! Three encodings included: Phred33, Phred64, and Sanger. Note that Sanger quality alphabet includes 33-126, or all 'printable' ASCII, and covers both PHRED quality character alphabets and more.
//! See charsets::quality.rs for more detailed information




// if non-ascii found, will replace with !, a q score of 0 by default. May also return error.


// 