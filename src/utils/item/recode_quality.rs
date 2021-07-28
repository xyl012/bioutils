
// use super::*;
// use crate::utils::item::check;
// use crate::utils::item::check::CheckAsRefSlice;
// // use crate::utils::item::check::CheckAsMutSlice;
// // use crate::utils::item::

// // pub trait CodeItemU8<T> {
// //     /// Returns the PHRED33 quality score from a PHRED33 quality encoding. The score is the u8 minus 33.
// //     fn decode_qual(&self) ->  Result<Vec<u8>>;
// //     /// Returns the PHRED64 quality score from a PHRED64 quality encoding. The score is the u8 minus 64.
// //     fn decode_qual_phred64(&self) ->  Result<Vec<u8>>;
// //     /// Returns the SANGER quality score from a SANGER quality encoding. The score is the u8 minus 33.
// //     fn decode_qual_sanger(&self) ->  Result<Vec<u8>>;
// //     /// Returns the PHRED33 quality encoding from a PHRED33 quality score. The score is the u8 minus 33.
// //     fn encode_qual(&self) ->  Result<Vec<u8>>;
// //     /// Returns the PHRED64 quality encoding from a PHRED64 quality score. The score is the u8 minus 64.
// //     fn encode_qual_phred64(&self) ->  Result<Vec<u8>>;
// //     /// Returns the SANGER quality encoding from a SANGER quality score. The score is the u8 minus 33.
// //     fn encode_qual_sanger(&self) -> Result<Vec<u8>>;
// // }

// // impl<T> CodeItemU8<T> for T where
// //     T: AsRef<[u8]>
// // {
// //     /// Returns the PHRED33 quality score from a raw PHRED33 quality encoding. The score is simply the u8 minus 33.
// //     fn decode_qual(&self) -> Result<Vec<u8>> {
// //         self.check_phred33();
// //         // }).collect::<Result<Vec<u8>>>()
// //         // .for_each(|u| PHRED33_HASHMAP.get(u).unwrap().to_owned()).collect::<Result<Vec<u8>>>()
// //     }
// //     // .into_iter().map(|(_, v)| v)
// //     /// Returns the PHRED64 quality score from a raw PHRED64 quality encoding. The score is simply the u8 minus 64.
// //     fn decode_qual_phred64(&self) -> Result<Vec<u8>> {
// //         self.as_ref().iter().map(|q| PHRED64_HASHMAP.get(&q)?.to_owned()).collect::<Result<Vec<u8>>>()
// //     }
// //     /// Returns the SANGER quality score from a SANGER quality encoding. The score is the u8 minus 33.
// //     fn decode_qual_sanger(&self) -> Result<Vec<u8>> {
// //         self.as_ref().iter().map(|q| SANGER_HASHMAP_DECODE.get(&q)?.to_owned()).collect::<Result<Vec<u8>>>()
// //     }
// //     /// Returns the PHRED33 quality encoding from a PHRED33 quality score. The score is the u8 minus 33.
// //     fn encode_qual(&self) -> Result<Vec<u8>> {
// //         self.as_ref().iter().map(|q| PHRED33_HASHMAP_ENCODE.get(&q)?.to_owned()).collect::<Result<Vec<u8>>>()
// //     }
// //     /// Returns the PHRED64 quality encoding from a PHRED64 quality score. The score is the u8 minus 64.
// //     fn encode_qual_phred64(&self) -> Result<Vec<u8>> {
// //         self.as_ref().iter().map(|q| PHRED64_HASHMAP_ENCODE.get(&q)?.to_owned()).collect::<Result<Vec<u8>>>()
// //     }
// //     /// Returns the SANGER quality encoding from a SANGER quality score. The score is the u8 minus 33.
// //     fn encode_qual_sanger(&self) -> Result<Vec<u8>> {
// //         self.as_ref().iter().map(|q| SANGER_HASHMAP_ENCODE.get(&q)?.to_owned()).collect::<Result<Vec<u8>>>()
// //     }
// // }

// pub trait OwnedRecodeQualitySlice<T> {
//     /// Returns the PHRED33 quality score from a raw PHRED33 quality encoding (-33).
//     fn decode_qual(&self) -> Result<Vec<u8>>;
//     /// Returns the PHRED64 quality score from a raw PHRED64 quality encoding (-64).
//     fn decode_qual_phred64(&self) -> Result<Vec<u8>>;
//     /// Returns the PHRED33 quality encoding from a PHRED33 quality score. (+33)
//     fn encode_qual(&self) -> Result<Vec<u8>>;
//     /// Returns the PHRED64 quality encoding from a PHRED64 quality score. (+64)
//     fn encode_qual_phred64(&self) -> Result<Vec<u8>>;
// }

// impl<T> OwnedRecodeQualitySlice<T> for T
// where
//     T: AsRef<[u8]>,
// {
//     /// Returns the PHRED33 quality score from a raw PHRED33 quality encoding (-33).
//     fn decode_qual(&self) -> Result<Vec<u8>> {
//         self.as_ref().iter().map(|u| PHRED33_HASHMAP_DECODE.get(u).copied()).collect::<Option<Vec<u8>>>().ok_or(bail!("Contains non-PHRED33 u8s"))
//     }
//     /// Returns the PHRED64 quality score from a raw PHRED64 quality encoding (-64).
//     fn decode_qual_phred64(&self) -> Result<Vec<u8>> {
//         self.as_ref().iter().map(|u| PHRED64_HASHMAP_DECODE.get(u).copied()).collect::<Option<Vec<u8>>>().ok_or(bail!("Contains non-PHRED64 u8s"))
//     }
//     /// Returns the PHRED33 quality encoding from a PHRED33 quality score. (+33)
//     fn encode_qual(&self) -> Result<Vec<u8>> {
//         self.as_ref().iter().map(|u| PHRED33_HASHMAP_ENCODE.get(u).copied()).collect::<Option<Vec<u8>>>().ok_or(bail!("Contains non-PHRED33 score u8s"))
//     }
//     /// Returns the PHRED64 quality encoding from a PHRED64 quality score. (+64)
//     fn encode_qual_phred64(&self) -> Result<Vec<u8>> {
//         self.as_ref().iter().map(|u| PHRED64_HASHMAP_ENCODE.get(u).copied()).collect::<Option<Vec<u8>>>().ok_or(bail!("Contains non-PHRED64 score u8s"))
//     }
// }

// pub trait MutCodeItemU8<T> {
//     /// Returns the PHRED33 quality score from a PHRED33 quality encoding. The score is the u8 minus 33. Sanger is also shifted 33.
//     fn mut_decode_qual(&mut self) -> Result<&mut Self>;
//     /// Returns the PHRED64 quality score from a PHRED64 quality encoding. The score is the u8 minus 64.
//     fn mut_decode_qual_phred64(&mut self) -> Result<&mut Self>;
//     /// Returns the PHRED33 quality encoding from a PHRED33 quality score. The score is the u8 minus 33. Sanger is also shifted 33.
//     fn mut_encode_qual(&mut self) -> Result<&mut Self>;
//     /// Returns the PHRED64 quality encoding from a PHRED64 quality score. The score is the u8 minus 64.
//     fn mut_encode_qual_phred64(&mut self) -> Result<&mut Self>;
// }

// impl<T> MutCodeItemU8<T> for T
// where
//     T: AsMut<[u8]>,
// {
//     /// Returns the PHRED33 quality score from a raw PHRED33 quality encoding (-33).
//     fn mut_decode_qual(&mut self) -> Result<&mut Self> {
//         match self.mut_is_phred33() {
//             true => {self.as_mut().iter_mut().for_each(|u| *u = *u-33); Ok(self)},
//             false => bail!("Contains non-PHRED33 u8s")
//         }
//     }
//     /// Returns the PHRED64 quality score from a raw PHRED64 quality encoding (-64).
//     fn mut_decode_qual_phred64(&mut self) -> Result<&mut Self> {
//         match self.mut_is_phred64() {
//             true => {self.as_mut().iter_mut().for_each(|u| *u = *u-64); Ok(self)},
//             false => bail!("Contains non-PHRED64 u8s")
//         } 
//     }
//     /// Returns the PHRED33 quality encoding from a PHRED33 quality score (+33).
//     fn mut_encode_qual(&mut self) -> Result<&mut Self> {
//         match self.mut_is_phred33_scores() {
//             true => {self.as_mut().iter_mut().for_each(|u| *u = *u+33); Ok(self)},
//             false => bail!("Contains non-PHRED33 score u8s")
//         }
//     }
//     /// Returns the PHRED64 quality encoding from the quality score (+64).
//     fn mut_encode_qual_phred64(&mut self) -> Result<&mut Self> {
//         match self.mut_is_phred64_scores() {
//             true => {self.as_mut().iter_mut().for_each(|u| *u = *u+64); Ok(self)},
//             false => bail!("Contains non-PHRED64 score u8s")
//         }
//     }
// }

// // /// Returns the PHRED64 quality score from a raw PHRED64 quality encoding (-64).
//     // fn decode_qual_phred64(&self) -> Result<Vec<u8>> {
//     //     match self.is_phred64() {
//     //         true => {Ok(self.as_ref().iter().map(|u| u-64).collect())},
//     //         false => bail!("Contains non-PHRED64 u8s")
//     //     } 
//     // }
//     // /// Returns the PHRED33 quality encoding from a PHRED33 quality score (+33).
//     // fn encode_qual(&self) -> Result<Vec<u8>> {
//     //     match self.is_phred33_scores() {
//     //         true => {Ok(self.as_ref().iter().map(|u| u+33).collect())},
//     //         false => bail!("Contains non-PHRED33 score u8s")
//     //     }
//     // }
//     // /// Returns the PHRED64 quality encoding from the quality score (+64).
//     // fn mut_encode_qual_phred64(&mut self) -> Result<&mut Self> {
//     //     match self.mut_is_phred64_scores() {
//     //         true => {self.as_mut().iter_mut().for_each(|u| *u = *u+64); Ok(self)},
//     //         false => bail!("Contains non-PHRED64 score u8s")
//     //     }
//     // }