// use super::*;

// use crate::utils::check::*;

// pub trait ComplementAsRefSlice<T> {
//     /// Returns the reverse nucleotide complement
//     fn reverse_nucleotide_complement(&self) -> Result<Vec<u8>>;
//     /// Returns the nucleotide complement
//     fn nucleotide_complement(&self) -> Result<Vec<u8>>;
// }

// impl<T> ComplementAsRefSlice<T> for T
// where
//     T: AsRef<[u8]>,
// {
//     /// Returns the reverse nucleotide complement
//     fn reverse_nucleotide_complement(&self) -> Result<Vec<u8>> {
//         self.as_ref().iter()
//             .rev()
//             .map(|nt| NUCLEOTIDE_COMPLEMENT_HASHMAP.get(nt).copied().ok_or(bail!("Contains non-base u8s")))
//             .collect()
//     }

//     /// Returns the nucleotide complement
//     fn nucleotide_complement(&self) -> Result<Vec<u8>> {
//         self.as_ref().iter()
//             .map(|nt| NUCLEOTIDE_COMPLEMENT_HASHMAP.get(nt).copied().ok_or(bail!("Contains non-base u8s")))
//             .collect()
//     }
// }

// pub trait ComplementAsMutSlice<T> {
//     /// Returns the reverse nucleotide complement
//     fn mut_reverse_nucleotide_complement(&mut self) -> Result<&mut Self>;
//     /// Returns the nucleotide complement
//     fn mut_nucleotide_complement(&mut self) -> Result<&mut Self>;
// }

// impl<T> ComplementAsMutSlice<T> for T
// where
//     T: AsMut<[u8]>,
// {
//     /// Returns the reverse nucleotide complement
//     fn mut_reverse_nucleotide_complement(&mut self) -> Result<&mut Self> {
//         self.mut_result_is_all_charset(BioUtilsCharSet::Dna)?
//             .as_mut().iter_mut()
//             .rev()
//             .for_each(|nt| { 

//         });
//         Ok(self)
//     }

//     /// Returns the nucleotide complement
//     fn mut_nucleotide_complement(&mut self) -> Result<&mut Self> {
//         self.as_mut().iter_mut()
//             .map(|nt| NUCLEOTIDE_COMPLEMENT_HASHMAP.get(nt).copied().ok_or(bail!("Contains non-base u8s")))
//             .collect()
//     }
// }

