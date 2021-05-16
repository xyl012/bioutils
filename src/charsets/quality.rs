
//! Quality character (sub-)sets including Phred33, Phred64, and Solexa/Illumina 1.0 (for compatibility). Provided as u8 and str arrays.

use std::collections::HashMap;
use super::*;

/// Phred33 charset: ASCII 33-75
pub const PHRED33_U8: [u8; 43] = [
    b'!', b'"', b'#', b'$', b'%', b'&', 0x0027, b'(', b')', b'*', b'+', b',', b'-', b'.', b'/',
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b':', b';', b'<', b'=', b'>', b'?',
    b'@', b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K'];

/// Phred33 scores: 0-42
pub const PHRED33_SCORES_U8: [u8; 43] = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42];

/// Phred33 charset: ASCII 33-73
    pub const PHRED33_STR: [&str; 43] = [
    r#"!"#, r#"""#, r##"#"##, r#"$"#, r#"%"#, r#"&"#, r#"'"#, r#"("#, r#")"#, r#"*"#, r#"+"#,
    r#","#, r#"-"#, r#"."#, r#"/"#, r#"0"#, r#"1"#, r#"2"#, r#"3"#, r#"4"#, r#"5"#, r#"6"#, r#"7"#,
    r#"8"#, r#"9"#, r#":"#, r#";"#, r#"<"#, r#"="#, r#">"#, r#"?"#, r#"@"#, r#"A"#, r#"B"#, r#"C"#,
    r#"D"#, r#"E"#, r#"F"#, r#"G"#, r#"H"#, r#"I"#, r#"J"#, r#"K"#];

lazy_static! {
    /// Phred33 charset as hashset: ASCII 33-73
    pub static ref PHRED33_HASHSET_U8: HashSet<u8> = new_u8_hashset(&PHRED33_U8);
}
lazy_static! {
    /// Phred33 charset as hashset: ASCII 33-73
    pub static ref PHRED33_HASHSET_STR: HashSet<&'static str> = new_str_hashset(&PHRED33_STR);
}

lazy_static!{
    /// This is the quality score shifted 33 so if the u8 is 33, the score is 0. We can look that up with this hashmap.
    pub static ref PHRED33_HASHMAP_U8: HashMap<u8, u8> = vec![
        (b'!', 0), (b'"', 1), (b'#', 2), (b'$', 3), (b'%', 4), (b'&', 5), (0x0027, 6), (b'(', 7), (b')', 8), (b'*', 9), (b'+', 10), (b',', 11), (b'-', 12), (b'.', 13), (b'/', 14),
        (b'0', 15), (b'1', 16), (b'2', 17), (b'3',18), (b'4', 19), (b'5', 20), (b'6', 21), (b'7', 22), (b'8', 23), (b'9', 24), (b':', 25), (b';', 26), (b'<', 27), (b'=', 28), (b'>', 29), (b'?', 30),
        (b'@', 31), (b'A', 32), (b'B', 33), (b'C', 34), (b'D', 35), (b'E', 36), (b'F', 37), (b'G', 38), (b'H', 39), (b'I', 40), (b'J', 41), (b'K', 42)
    ].into_iter().collect();
}

lazy_static!{
    /// This is the quality score shifted 33 so if the u8 is 33, the score is 0. We can look that up with this hashmap.
    pub static ref PHRED33_HASHMAP_ENCODE_U8: HashMap<u8, u8> = vec![
        (0 , b'!'), (1, b'"'), (2, b'#'), (3, b'$'), (4, b'%'), (5, b'&'), (6, 0x0027), (7, b'('), (8, b')'), (9, b'*'), (10, b'+'), (11, b','), (12, b'-'), (13, b'.'), (14, b'/'),
        (15, b'0'), (16, b'1'), (17, b'2'), (18, b'3'), (19, b'4'), (20, b'5'), (21, b'6'), (22, b'7'), (23, b'8'), (24, b'9'), (25, b':'), (26, b';'), (27, b'<'), (28, b'='), (29, b'>'), (30, b'?'),
        (31, b'@'), (32, b'A'), (33, b'B'), (34, b'C'), (35, b'D'), (36, b'E'), (37, b'F'), (38, b'G'), (39, b'H'), (40, b'I'), (41, b'J'), (42, b'K')
    ].into_iter().collect();
}

/// Phred64 charset: ASCII 64-126
pub const PHRED64_U8: [u8; 63] = [
    b'@', b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O',
    b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', 0x005B, 0x005C, 0x005D, b'^',
    b'_', b'`', b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n',
    b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'{', b'|', b'}', b'~',
];

/// Phred64 scores: 0-62
pub const PHRED64_SCORES_U8: [u8; 63] = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62];


/// Phred64 charset: ASCII 64-126
pub const PHRED64_STR: [&str; 63] = [
    r#"@"#, r#"A"#, r#"B"#, r#"C"#, r#"D"#, r#"E"#, r#"F"#, r#"G"#, r#"H"#, r#"I"#, r#"J"#, r#"K"#,
    r#"L"#, r#"M"#, r#"N"#, r#"O"#, r#"P"#, r#"Q"#, r#"R"#, r#"S"#, r#"T"#, r#"U"#, r#"V"#, r#"W"#,
    r#"X"#, r#"Y"#, r#"Z"#, r#"["#, r#"\"#, r#"]"#, r#"^"#, r#"_"#, r#"`"#, r#"a"#, r#"b"#, r#"c"#,
    r#"d"#, r#"e"#, r#"f"#, r#"g"#, r#"h"#, r#"i"#, r#"j"#, r#"k"#, r#"l"#, r#"m"#, r#"n"#, r#"o"#,
    r#"p"#, r#"q"#, r#"r"#, r#"s"#, r#"t"#, r#"u"#, r#"v"#, r#"w"#, r#"x"#, r#"y"#, r#"z"#, r#"{"#,
    r#"|"#, r#"}"#, r#"~"#,
];
lazy_static! {
    /// Phred64 charset as hashset: ASCII 64-126
    pub static ref PHRED64_HASHSET_U8: HashSet<u8> = new_u8_hashset(&PHRED64_U8);
}
lazy_static! {
    /// Phred64 charset as hashset: ASCII 64-126
    pub static ref PHRED64_HASHSET_STR: HashSet<&'static str> = new_str_hashset(&PHRED64_STR);
}

lazy_static!{
    /// This is the quality score shifted 64 so if the u8 is 64, the score is 0. We can look that up with this hashmap.
    pub static ref PHRED64_HASHMAP_U8: HashMap<u8, u8> = vec![
        (b'@', 0), (b'A', 1), (b'B', 2), (b'C', 3), (b'D', 4), (b'E', 5), (b'F', 6), (b'G', 7), (b'H', 8), (b'I', 9), (b'J', 10), (b'K', 11), (b'L', 12), (b'M', 13), (b'N', 14), (b'O', 15),
        (b'P', 16), (b'Q', 17), (b'R', 18), (b'S', 19), (b'T', 20), (b'U', 21), (b'V', 22), (b'W', 23), (b'X', 24), (b'Y', 25), (b'Z', 26), (0x005B, 27), (0x005C, 28), (0x005D, 29), (b'^', 30),
        (b'_', 31), (b'`', 32), (b'a', 33), (b'b', 34), (b'c', 35), (b'd', 36), (b'e', 37), (b'f', 38), (b'g', 39), (b'h', 40), (b'i', 41), (b'j', 42), (b'k', 43), (b'l', 44), (b'm', 45), (b'n', 46),
        (b'o', 47), (b'p', 48), (b'q', 49), (b'r', 50), (b's', 51), (b't', 52), (b'u', 53), (b'v', 54), (b'w', 55), (b'x', 56), (b'y', 57), (b'z', 58), (b'{', 59), (b'|', 60), (b'}', 61), (b'~', 62),
    ].into_iter().collect();
}

lazy_static!{
    /// This is the quality score shifted 64 so if the u8 is 64, the score is 0. We can look that up with this hashmap.
    pub static ref PHRED64_HASHMAP_ENCODE_U8: HashMap<u8, u8> = vec![
        (0, b'@'), (1, b'A'), (2, b'B'), (3, b'C'), (4, b'D'), (5, b'E'), (6, b'F'), (7, b'G'), (8, b'H'), (9, b'I'), (10, b'J'), (11, b'K'), (12, b'L'), (13, b'M'), (14, b'N'), (15, b'O'),
        (16, b'P'), (17, b'Q'), (18, b'R'), (19, b'S'), (20, b'T'), (21, b'U'), (22, b'V'), (23, b'W'), (24, b'X'), (25, b'Y'), (26, b'Z'), (27, 0x005B), (28, 0x005C), (29, 0x005D), (30, b'^'),
        (31, b'_'), (32, b'`'), (33, b'a'), (34, b'b'), (35, b'c'), (36, b'd'), (37, b'e'), (38, b'f'), (39, b'g'), (40, b'h'), (41, b'i'), (42, b'j'), (43, b'k'), (44, b'l'), (45, b'm'), (46, b'n'),
        (47, b'o'), (48, b'p'), (49, b'q'), (50, b'r'), (51, b's'), (52, b't'), (53, b'u'), (54, b'v'), (55, b'w'), (56, b'x'), (57, b'y'), (58, b'z'), (59, b'{'), (60, b'|'), (61, b'}'), (62, b'~'),
    ].into_iter().collect();
}

/// Solexa/Illumina 1.0 charset: ASCII 59-126.
pub const SOLEXA_U8: [u8; 68] = [
    b';', b'<', b'=', b'>', b'?', b'@', b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J',
    b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z',
    0x005B, 0x005C, 0x005D, b'^', b'_', b'`', b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i',
    b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y',
    b'z', b'{', b'|', b'}', b'~',
];

/// Solexa/Illumina 1.0 charset: ASCII 59-126.
pub const SOLEXA_STR: [&str; 68] = [
    r#";"#, r#"<"#, r#"="#, r#">"#, r#"?"#, r#"@"#, r#"A"#, r#"B"#, r#"C"#, r#"D"#, r#"E"#, r#"F"#,
    r#"G"#, r#"H"#, r#"I"#, r#"J"#, r#"K"#, r#"L"#, r#"M"#, r#"N"#, r#"O"#, r#"P"#, r#"Q"#, r#"R"#,
    r#"S"#, r#"T"#, r#"U"#, r#"V"#, r#"W"#, r#"X"#, r#"Y"#, r#"Z"#, r#"["#, r#"\"#, r#"]"#, r#"^"#,
    r#"_"#, r#"`"#, r#"a"#, r#"b"#, r#"c"#, r#"d"#, r#"e"#, r#"f"#, r#"g"#, r#"h"#, r#"i"#, r#"j"#,
    r#"k"#, r#"l"#, r#"m"#, r#"n"#, r#"o"#, r#"p"#, r#"q"#, r#"r"#, r#"s"#, r#"t"#, r#"u"#, r#"v"#,
    r#"w"#, r#"x"#, r#"y"#, r#"z"#, r#"{"#, r#"|"#, r#"}"#, r#"~"#,
];
lazy_static! {
    /// Solexa/Illumina 1.0 charset: ASCII 59-126.
    pub static ref SOLEXA_HASHSET_U8: HashSet<u8> = new_u8_hashset(&SOLEXA_U8);
}
lazy_static! {
    /// Solexa/Illumina 1.0 charset: ASCII 59-126.
    pub static ref SOLEXA_HASHSET_STR: HashSet<&'static str> = new_str_hashset(&SOLEXA_STR);
}

/// Sanger charset: ASCII 33-126
pub const SANGER_U8: [u8; 94] = [
    b'!', b'"', b'#', b'$', b'%', b'&', 0x0027, b'(', b')', b'*', b'+', b',', b'-', b'.', b'/',
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b':', b';', b'<', b'=', b'>', b'?',
    b'@', b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O',
    b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', 0x005B, 0x005C, 0x005D, b'^',
    b'_', b'`', b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n',
    b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z', b'{', b'|', b'}', b'~',
    ];
/// Sanger charset: ASCII 33-126
pub const SANGER_STR: [&str; 94] = [
    r#"!"#, r#"""#, r##"#"##, r#"$"#, r#"%"#, r#"&"#, r#"'"#, r#"("#, r#")"#, r#"*"#, r#"+"#,
    r#","#, r#"-"#, r#"."#, r#"/"#, r#"0"#, r#"1"#, r#"2"#, r#"3"#, r#"4"#, r#"5"#, r#"6"#, r#"7"#,
    r#"8"#, r#"9"#, r#":"#, r#";"#, r#"<"#, r#"="#, r#">"#, r#"?"#, r#"@"#, r#"A"#, r#"B"#, r#"C"#,
    r#"D"#, r#"E"#, r#"F"#, r#"G"#, r#"H"#, r#"I"#, r#"J"#, r#"K"#, r#"L"#, r#"M"#, r#"N"#, r#"O"#,
    r#"P"#, r#"Q"#, r#"R"#, r#"S"#, r#"T"#, r#"U"#, r#"V"#, r#"W"#, r#"X"#, r#"Y"#, r#"Z"#, r#"["#,
    r#"\"#, r#"]"#, r#"^"#, r#"_"#, r#"`"#, r#"a"#, r#"b"#, r#"c"#, r#"d"#, r#"e"#, r#"f"#, r#"g"#,
    r#"h"#, r#"i"#, r#"j"#, r#"k"#, r#"l"#, r#"m"#, r#"n"#, r#"o"#, r#"p"#, r#"q"#, r#"r"#, r#"s"#,
    r#"t"#, r#"u"#, r#"v"#, r#"w"#, r#"x"#, r#"y"#, r#"z"#, r#"{"#, r#"|"#, r#"}"#, r#"~"#,
];

lazy_static! {
    /// Sanger charset as hashset: ASCII 33-126
    pub static ref SANGER_HASHSET_U8: HashSet<u8> = new_u8_hashset(&SANGER_U8);
}
lazy_static! {
    /// Sanger charset as hashset: ASCII 33-126
    pub static ref SANGER_HASHSET_STR: HashSet<&'static str> = new_str_hashset(&SANGER_STR);
}

// #[cfg(test)]
// mod tests {
//     use super::{PHRED33_U8, PHRED64_U8, SOLEXA_U8};
//     #[test]
//     fn test_phred33() {
//         let dec: Vec<u8> = (33..127).collect();
//         assert_eq!(dec, PHRED33_U8);
//     }
//     #[test]
//     fn test_phred64() {
//         let dec: Vec<u8> = (64..127).collect();
//         assert_eq!(dec, PHRED64_U8);
//     }
//     #[test]
//     fn test_solexa() {
//         let dec: Vec<u8> = (59..127).collect();
//         assert_eq!(dec, SOLEXA_U8);
//     }
// }


// ASCII Codes:
// Dec	Hex	Binary	HTML	Char	Description
// 0	00	00000000	&#0;	NUL	Null
// 1	01	00000001	&#1;	SOH	Start of Header
// 2	02	00000010	&#2;	STX	Start of Text
// 3	03	00000011	&#3;	ETX	End of Text
// 4	04	00000100	&#4;	EOT	End of Transmission
// 5	05	00000101	&#5;	ENQ	Enquiry
// 6	06	00000110	&#6;	ACK	Acknowledge
// 7	07	00000111	&#7;	BEL	Bell
// 8	08	00001000	&#8;	BS	Backspace
// 9	09	00001001	&#9;	HT	Horizontal Tab
// 10	0A	00001010	&#10;	LF	Line Feed
// 11	0B	00001011	&#11;	VT	Vertical Tab
// 12	0C	00001100	&#12;	FF	Form Feed
// 13	0D	00001101	&#13;	CR	Carriage Return
// 14	0E	00001110	&#14;	SO	Shift Out
// 15	0F	00001111	&#15;	SI	Shift In
// 16	10	00010000	&#16;	DLE	Data Link Escape
// 17	11	00010001	&#17;	DC1	Device Control 1
// 18	12	00010010	&#18;	DC2	Device Control 2
// 19	13	00010011	&#19;	DC3	Device Control 3
// 20	14	00010100	&#20;	DC4	Device Control 4
// 21	15	00010101	&#21;	NAK	Negative Acknowledge
// 22	16	00010110	&#22;	SYN	Synchronize
// 23	17	00010111	&#23;	ETB	End of Transmission Block
// 24	18	00011000	&#24;	CAN	Cancel
// 25	19	00011001	&#25;	EM	End of Medium
// 26	1A	00011010	&#26;	SUB	Substitute
// 27	1B	00011011	&#27;	ESC	Escape
// 28	1C	00011100	&#28;	FS	File Separator
// 29	1D	00011101	&#29;	GS	Group Separator
// 30	1E	00011110	&#30;	RS	Record Separator
// 31	1F	00011111	&#31;	US	Unit Separator
// 32	20	00100000	&#32;	space	Space
// 33	21	00100001	&#33;	!	Exclamation mark
// 34	22	00100010	&#34;	"	Double quote
// 35	23	00100011	&#35;	#	Number
// 36	24	00100100	&#36;	$	Dollar sign
// 37	25	00100101	&#37;	%	Percent
// 38	26	00100110	&#38;	&	Ampersand
// 39	27	00100111	&#39;	'	Single quote
// 40	28	00101000	&#40;	(	Left parenthesis
// 41	29	00101001	&#41;	)	Right parenthesis
// 42	2A	00101010	&#42;	*	Asterisk
// 43	2B	00101011	&#43;	+	Plus
// 44	2C	00101100	&#44;	,	Comma
// 45	2D	00101101	&#45;	-	Minus
// 46	2E	00101110	&#46;	.	Period
// 47	2F	00101111	&#47;	/	Slash
// 48	30	00110000	&#48;	0	Zero
// 49	31	00110001	&#49;	1	One
// 50	32	00110010	&#50;	2	Two
// 51	33	00110011	&#51;	3	Three
// 52	34	00110100	&#52;	4	Four
// 53	35	00110101	&#53;	5	Five
// 54	36	00110110	&#54;	6	Six
// 55	37	00110111	&#55;	7	Seven
// 56	38	00111000	&#56;	8	Eight
// 57	39	00111001	&#57;	9	Nine
// 58	3A	00111010	&#58;	:	Colon
// 59	3B	00111011	&#59;	;	Semicolon
// 60	3C	00111100	&#60;	<	Less than
// 61	3D	00111101	&#61;	=	Equality sign
// 62	3E	00111110	&#62;	>	Greater than
// 63	3F	00111111	&#63;	?	Question mark
// 64	40	01000000	&#64;	@	At sign
// 65	41	01000001	&#65;	A	Capital A
// 66	42	01000010	&#66;	B	Capital B
// 67	43	01000011	&#67;	C	Capital C
// 68	44	01000100	&#68;	D	Capital D
// 69	45	01000101	&#69;	E	Capital E
// 70	46	01000110	&#70;	F	Capital F
// 71	47	01000111	&#71;	G	Capital G
// 72	48	01001000	&#72;	H	Capital H
// 73	49	01001001	&#73;	I	Capital I
// 74	4A	01001010	&#74;	J	Capital J
// 75	4B	01001011	&#75;	K	Capital K
// 76	4C	01001100	&#76;	L	Capital L
// 77	4D	01001101	&#77;	M	Capital M
// 78	4E	01001110	&#78;	N	Capital N
// 79	4F	01001111	&#79;	O	Capital O
// 80	50	01010000	&#80;	P	Capital P
// 81	51	01010001	&#81;	Q	Capital Q
// 82	52	01010010	&#82;	R	Capital R
// 83	53	01010011	&#83;	S	Capital S
// 84	54	01010100	&#84;	T	Capital T
// 85	55	01010101	&#85;	U	Capital U
// 86	56	01010110	&#86;	V	Capital V
// 87	57	01010111	&#87;	W	Capital W
// 88	58	01011000	&#88;	X	Capital X
// 89	59	01011001	&#89;	Y	Capital Y
// 90	5A	01011010	&#90;	Z	Capital Z
// 91	5B	01011011	&#91;	[	Left square bracket
// 92	5C	01011100	&#92;	\	Backslash
// 93	5D	01011101	&#93;	]	Right square bracket
// 94	5E	01011110	&#94;	^	Caret / circumflex
// 95	5F	01011111	&#95;	_	Underscore
// 96	60	01100000	&#96;	`	Grave / accent
// 97	61	01100001	&#97;	a	Small a
// 98	62	01100010	&#98;	b	Small b
// 99	63	01100011	&#99;	c	Small c
// 100	64	01100100	&#100;	d	Small d
// 101	65	01100101	&#101;	e	Small e
// 102	66	01100110	&#102;	f	Small f
// 103	67	01100111	&#103;	g	Small g
// 104	68	01101000	&#104;	h	Small h
// 105	69	01101001	&#105;	i	Small i
// 106	6A	01101010	&#106;	j	Small j
// 107	6B	01101011	&#107;	k	Small k
// 108	6C	01101100	&#108;	l	Small l
// 109	6D	01101101	&#109;	m	Small m
// 110	6E	01101110	&#110;	n	Small n
// 111	6F	01101111	&#111;	o	Small o
// 112	70	01110000	&#112;	p	Small p
// 113	71	01110001	&#113;	q	Small q
// 114	72	01110010	&#114;	r	Small r
// 115	73	01110011	&#115;	s	Small s
// 116	74	01110100	&#116;	t	Small t
// 117	75	01110101	&#117;	u	Small u
// 118	76	01110110	&#118;	v	Small v
// 119	77	01110111	&#119;	w	Small w
// 120	78	01111000	&#120;	x	Small x
// 121	79	01111001	&#121;	y	Small y
// 122	7A	01111010	&#122;	z	Small z
// 123	7B	01111011	&#123;	{	Left curly bracket
// 124	7C	01111100	&#124;	|	Vertical bar
// 125	7D	01111101	&#125;	}	Right curly bracket
// 126	7E	01111110	&#126;	~	Tilde
// 127	7F	01111111	&#127;	DEL	Delete

// Range 	Offset 	Type 	Range
// Sanger standard
//     fastq-sanger 	33–126 	33 	PHRED 	0 to 93
// Solexa/early Illumina
//     fastq-solexa 	59–126 	64 	Solexa 	−5 to 62
// Illumina 1.3+
//     fastq-illumina 	64–126 	64 	PHRED 	0 to 62
