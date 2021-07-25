use super::*;

pub trait CheckPercentAsRefSlice<T> {
    /// Checks if T is comprised of valid percentages
    fn check_percentages(&self) -> Result<&Self>;
    /// Returns a boolean if T is comprised of valid percentages
    fn is_percentages(&self) -> bool;
}

impl<T> CheckPercentAsRefSlice<T> for T where
T: AsRef<[u8]>
{
    /// Checks if T is comprised of valid percentages
    fn check_percentages(&self) -> Result<&Self> {
        match self.is_percentages() {
            true => Ok(self),
            false => bail!("Contains non-percentage elements"),
        }
    }
    /// Returns a boolean if T is comprised of valid percentages
    fn is_percentages(&self) -> bool {
        self.as_ref().iter().all(|x| PERCENTAGE_U8.contains(&x))
    }
}

pub trait CheckPercentAsMutSlice<T> {
    /// Checks if T is comprised of valid percentages
    fn mut_check_percentages(&self) -> Result<&Self>;
    /// Returns a boolean if T is comprised of valid percentages
    fn mut_is_percentages(&self) -> bool;
}

impl<T> CheckPercentAsMutSlice<T> for T where
T: AsMut<[u8]>
{
    /// Checks if T is comprised of valid percentages
    fn mut_check_percentages(&self) -> Result<&Self> {
        match self.mut_is_percentages() {
            true => Ok(self),
            false => bail!("Contains non-percentage elements"),
        }
    }
    /// Returns a boolean if T is comprised of valid percentages
    fn mut_is_percentages(&self) -> bool {
        self.as_mut().iter().all(|x| PERCENTAGE_U8.contains(&x))
    }
}