//! Creates new random vectors and u8s from given charsets. Charsets can be a bioutils charset or a u8 slice
//! ```
//! use bioutils::utils::new::RandomBioVec;
//! use bioutils::charsets::bioutils::*;
//! let new_vec = Vec::<u8>::random_vec(&12, BioUtilsCharSet::Dna);
//! let new_vec_2 = Vec::<u8>::random_vec_with(&12, &[1u8,2u8,3u8]);
//! println!("{:?}", new_vec);
//! println!("{:?}", new_vec_2);
//! ```

use super::*;

pub trait RandomBioVec<T> {
    /// Create new random sequences with specified number of elements.
    fn random_vec(nbases: &usize, charset: BioUtilsCharSet) -> Result<Vec<u8>>;
    /// Creates new u8 from the chosen characterset
    fn random_u8(charset: BioUtilsCharSet) -> Result<u8>;

    /// Create new random sequences with specified number of elements.
    fn random_vec_with(nbases: &usize, charset: &[u8]) -> Result<Vec<u8>>;
    /// Creates new u8 from the chosen characterset
    fn random_u8_with(charset: &[u8]) -> Result<u8>;
}

impl<T> RandomBioVec<T> for T
{
    /// Create new random sequences with specified number of elements.
    fn random_vec(nbases: &usize, charset: BioUtilsCharSet) -> Result<Vec<u8>> {
        let mut rng = rand::thread_rng();
        let mut vec = Vec::with_capacity(*nbases);
        for _base in 0..*nbases {
            vec.push(charset.value().choose(&mut rng).copied().to_owned())
        };
        vec.into_iter().collect::<Option<Vec<u8>>>().ok_or(bail!("Cannot create new random"))
    }
    /// Creates new u8 from the chosen characterset
    fn random_u8(charset: BioUtilsCharSet) -> Result<u8> {
        let mut rng = rand::thread_rng();
        charset.value().choose(&mut rng).copied().ok_or(bail!("Cannot create new random"))
    }
    /// Create new random sequences with specified number of elements.
    fn random_vec_with(nbases: &usize, charset: &[u8]) -> Result<Vec<u8>> {
        let mut rng = rand::thread_rng();
        let mut vec = Vec::with_capacity(*nbases);
        for _base in 0..*nbases {
            vec.push(charset.choose(&mut rng).copied().to_owned())
        };
        vec.into_iter().collect::<Option<Vec<u8>>>().ok_or(bail!("Cannot create new random"))
    }
    /// Creates new u8 from the chosen characterset
    fn random_u8_with(charset: &[u8]) -> Result<u8> {
        let mut rng = rand::thread_rng();
        charset.choose(&mut rng).copied().ok_or(bail!("Cannot create new random"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::new::RandomBioVec;

    #[test]
    fn test() {
        let test = Vec::<u8>::random_vec(&12, BioUtilsCharSet::Dna);
        println!("{:?}", test);
    }
}
