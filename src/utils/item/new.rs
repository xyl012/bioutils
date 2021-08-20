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