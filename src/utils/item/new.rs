use super::*;

pub trait RandomBio<T> {
    /// Create new random sequences with specified number of elements.
    fn new_vec_with(nbases: &usize, charset: BioUtilsCharSet) -> Result<Vec<u8>>;
    /// Creates new u8 from the chosen characterset
    fn new_u8_with(charset: BioUtilsCharSet) -> Result<u8>;
}

impl<T> RandomBio<T> for T
{
    /// Create new random sequences with specified number of elements.
    fn new_vec_with(nbases: &usize, charset: BioUtilsCharSet) -> Result<Vec<u8>> {
        let mut rng = rand::thread_rng();
        let mut vec = Vec::with_capacity(*nbases);
        for _base in 0..*nbases {
            vec.push(charset.value().choose(&mut rng).copied().to_owned())
        };
        vec.into_iter().collect::<Option<Vec<u8>>>().ok_or(bail!("Cannot create new random"))
    }
    /// Creates new u8 from the chosen characterset
    fn new_u8_with(charset: BioUtilsCharSet) -> Result<u8> {
        let mut rng = rand::thread_rng();
        charset.value().choose(&mut rng).copied().ok_or(bail!("Cannot create new random"))
    }
}