use super::*;

pub trait CheckAsRefU8<T>{
    fn check_percentage_u8(&self) -> Result<&Self>;
    fn is_percentage_u8(&self) -> bool;
    fn check_phred33_u8(&self) -> Result<&Self>;
    fn is_phred33_u8(&self) -> bool;
    fn check_phred33_score_u8(&self) -> Result<&Self>;
    fn is_phred33_score_u8(&self) -> bool;
    fn check_phred64_u8(&self) -> Result<&Self>;
    fn is_phred64_u8(&self) -> bool;
    fn check_phred64_score_u8(&self) -> Result<&Self>;
    fn is_phred64_score_u8(&self) -> bool;
}

pub trait CheckAsMutU8<T>{
    fn mut_check_percentage_u8(&mut self) -> Result<&mut Self>;
    fn mut_is_percentage_u8(&mut self) -> bool;
    fn mut_check_phred33_u8(&mut self) -> Result<&mut Self>;
    fn mut_is_phred33_u8(&mut self) -> bool;
    fn mut_check_phred33_score_u8(&mut self) -> Result<&mut Self>;
    fn mut_is_phred33_score_u8(&mut self) -> bool;
    fn mut_check_phred64_u8(&mut self) -> Result<&mut Self>;
    fn mut_is_phred64_u8(&mut self) -> bool;
    fn mut_check_phred64_score_u8(&mut self) -> Result<&mut Self>;
    fn mut_is_phred64_score_u8(&mut self) -> bool;
}

impl<T> CheckAsRefU8<T> for T where 
T: AsRef<u8>
{
    fn check_percentage_u8(&self) -> Result<&Self> {
        match self.is_percentage_u8() {    
            true => Ok(self),
            false => bail!("Please supply a valid percentage (0-100, not fractional) as u8"),
        }
    }
    fn is_percentage_u8(&self) -> bool {
        PERCENTAGE_U8.contains(self.as_ref())
    }
    
    /// Validate a u8 is phred33 (33-75)
    fn check_phred33_u8(&self) -> Result<&Self> {
        match self.is_phred33_u8() {    
        true => Ok(self),
        false => bail!("Please supply a quality score (33-75, not fractional) as u8"),
        }
    }
    fn is_phred33_u8(&self) -> bool {
        PHRED33_U8.contains(self.as_ref())
    }

    fn check_phred33_score_u8(&self) -> Result<&Self> {
        match self.is_phred33_score_u8() {    
            true => Ok(self),
            false => bail!("Please supply a quality score (0-42) as u8"),
        }
    }
    
    fn is_phred33_score_u8(&self) -> bool {
        PHRED33_SCORES_U8.contains(self.as_ref())
    }
    /// Validate a u8 phred64 (64-126)
    fn check_phred64_u8(&self) -> Result<&Self> {
        match self.is_phred64_u8() {    
        true => Ok(self),
        false => bail!("Please supply a PHRED64 quality (64-126) as u8"),
        }
    }
    fn is_phred64_u8(&self) -> bool {
        PHRED64_U8.contains(self.as_ref())
    }
    /// Validate a u8 phred64 score (0-62)
    fn check_phred64_score_u8(&self) -> Result<&Self> {
        match self.is_phred64_score_u8() {    
            true => Ok(self),
            false => bail!("Please supply a quality score (0-42) as u8"),
        }
    }
    fn is_phred64_score_u8(&self) -> bool {
        PHRED64_SCORES_U8.contains(self.as_ref())
    }
}

impl<T> CheckAsMutU8<T> for T where 
T: AsMut<u8>
{
    fn mut_check_percentage_u8(&mut self) -> Result<&mut Self> {
        match self.mut_is_percentage_u8() {    
            true => Ok(self),
            false => bail!("Please supply a valid percentage (0-100, not fractional) as u8"),
        }
    }
    fn mut_is_percentage_u8(&mut self) -> bool {
        PERCENTAGE_U8.contains(self.as_mut())
    }
    
    /// Validate a u8 is phred33 (33-75)
    fn mut_check_phred33_u8(&mut self) -> Result<&mut Self> {
        match self.mut_is_phred33_u8() {    
        true => Ok(self),
        false => bail!("Please supply a quality score (33-75, not fractional) as u8"),
        }
    }
    fn mut_is_phred33_u8(&mut self) -> bool {
        PHRED33_U8.contains(self.as_mut())
    }

    fn mut_check_phred33_score_u8(&mut self) -> Result<&mut Self> {
        match self.mut_is_phred33_score_u8() {    
            true => Ok(self),
            false => bail!("Please supply a quality score (0-42) as u8"),
        }
    }
    fn mut_is_phred33_score_u8(&mut self) -> bool {
        PHRED33_SCORES_U8.contains(self.as_mut())
    }
    /// Validate a u8 phred64 (64-126)
    fn mut_check_phred64_u8(&mut self) -> Result<&mut Self> {
        match self.mut_is_phred64_u8() {    
        true => Ok(self),
        false => bail!("Please supply a PHRED64 quality (64-126) as u8"),
        }
    }
    fn mut_is_phred64_u8(&mut self) -> bool {
        PHRED64_U8.contains(self.as_mut())
    }
    /// Validate a u8 phred64 score (0-62)
    fn mut_check_phred64_score_u8(&mut self) -> Result<&mut Self> {
        match self.mut_is_phred64_score_u8() {    
            true => Ok(self),
            false => bail!("Please supply a quality score (0-42) as u8"),
        }
    }
    fn mut_is_phred64_score_u8(&mut self) -> bool {
        PHRED64_SCORES_U8.contains(self.as_mut())
    }
}

