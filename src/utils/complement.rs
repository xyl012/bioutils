// use super::*;

// use crate::utils::check::*;

// // TODO move into recode as basically the same thing.
// pub trait ComplementAsRefU8 {
//     // /// Returns the reverse nucleotide complement
//     // fn reverse_nucleotide_complement(&self) -> Result<Vec<u8>>;
//     /// Returns the nucleotide complement
//     fn nucleotide_complement(&self) -> Result<Vec<u8>>;
// }

// // impl<T> ComplementAsRef<T> for T
// // where
// //     T: AsRef<[u8]>,
// // {
// impl ComplementAsRefU8 for u8
// {
//     // /// Returns the reverse nucleotide complement
//     // fn reverse_nucleotide_complement(&self) -> Result<Vec<u8>> {
//     //     self.as_ref().iter()
//     //         .rev()
//     //         .map(|nt| NUCLEOTIDE_COMPLEMENT_HASHMAP.get(nt).copied().ok_or(bail!("Contains non-base u8s")))
//     //         .collect()
//     // }

//     // /// Returns the nucleotide complement
//     // fn nucleotide_complement(&self) -> Result<Vec<u8>> {
//     //     self.as_ref().iter()
//     //         .map(|nt| NUCLEOTIDE_COMPLEMENT_HASHMAP.get(nt).copied().ok_or(bail!("Contains non-base u8s")))
//     //         .collect()
//     // }

//     /// Checks if self can be encoded (encoding contains all u8 in self) and encodes self.
//     fn nucleotide_complement(&self, code: BioUtilsRecodeSet) -> Option<u8> {
//         if code.value().charset.contains(&self) {

//         }
//     }

// }


// // pub trait ComplementAsMutSlice<T> {
// //     /// Returns the reverse nucleotide complement
// //     fn mut_reverse_nucleotide_complement(&mut self) -> Result<&mut Self>;
// //     /// Returns the nucleotide complement
// //     fn mut_nucleotide_complement(&mut self) -> Result<&mut Self>;
// // }

// // impl<T> ComplementAsMutSlice<T> for T
// // where
// //     T: AsMut<[u8]>,
// // {
// //     /// Returns the reverse nucleotide complement
// //     fn mut_reverse_nucleotide_complement(&mut self) -> Result<&mut Self> {
// //         self.mut_result_is_all_charset(BioUtilsCharSet::Dna)?
// //             .as_mut().iter_mut()
// //             .rev()
// //             .for_each(|nt| { 

// //         });
// //         Ok(self)
// //     }

// //     /// Returns the nucleotide complement
// //     fn mut_nucleotide_complement(&mut self) -> Result<&mut Self> {
// //         self.as_mut().iter_mut()
// //             .map(|nt| NUCLEOTIDE_COMPLEMENT_HASHMAP.get(nt).copied().ok_or(bail!("Contains non-base u8s")))
// //             .collect()
// //     }
// // }

