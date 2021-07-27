use super::*;

pub trait CheckAsRefU8<T>{
    fn check_percent(&self) -> Result<&Self>;
    fn is_percent(&self) -> bool;
    fn check_phred33(&self) -> Result<&Self>;
    fn is_phred33(&self) -> bool;
    fn check_phred33_score(&self) -> Result<&Self>;
    fn is_phred33_score(&self) -> bool;
    fn check_phred64(&self) -> Result<&Self>;
    fn is_phred64(&self) -> bool;
    fn check_phred64_score(&self) -> Result<&Self>;
    fn is_phred64_score(&self) -> bool;
}

pub trait CheckAsMutU8<T>{
    fn mut_check_percent(&mut self) -> Result<&mut Self>;
    fn mut_is_percent(&mut self) -> bool;
    fn mut_check_phred33(&mut self) -> Result<&mut Self>;
    fn mut_is_phred33(&mut self) -> bool;
    fn mut_check_phred33_score(&mut self) -> Result<&mut Self>;
    fn mut_is_phred33_score(&mut self) -> bool;
    fn mut_check_phred64(&mut self) -> Result<&mut Self>;
    fn mut_is_phred64(&mut self) -> bool;
    fn mut_check_phred64_score(&mut self) -> Result<&mut Self>;
    fn mut_is_phred64_score(&mut self) -> bool;
}

impl<T> CheckAsRefU8<T> for T where 
T: AsRef<u8>
{
    fn check_percent(&self) -> Result<&Self> {
        match self.is_percent() {    
            true => Ok(self),
            false => bail!("Please supply a valid PERCENT (0-100, not fractional) as u8"),
        }
    }
    fn is_percent(&self) -> bool {
        PERCENT.contains(self.as_ref())
    }
    
    /// Validate a u8 is phred33 (33-75)
    fn check_phred33(&self) -> Result<&Self> {
        match self.is_phred33() {    
        true => Ok(self),
        false => bail!("Please supply a quality score (33-75, not fractional) as u8"),
        }
    }
    fn is_phred33(&self) -> bool {
        PHRED33.contains(self.as_ref())
    }

    fn check_phred33_score(&self) -> Result<&Self> {
        match self.is_phred33_score() {    
            true => Ok(self),
            false => bail!("Please supply a quality score (0-42) as u8"),
        }
    }
    
    fn is_phred33_score(&self) -> bool {
        PHRED33_SCORES.contains(self.as_ref())
    }
    /// Validate a u8 phred64 (64-126)
    fn check_phred64(&self) -> Result<&Self> {
        match self.is_phred64() {    
        true => Ok(self),
        false => bail!("Please supply a PHRED64 quality (64-126) as u8"),
        }
    }
    fn is_phred64(&self) -> bool {
        PHRED64.contains(self.as_ref())
    }
    /// Validate a u8 phred64 score (0-62)
    fn check_phred64_score(&self) -> Result<&Self> {
        match self.is_phred64_score() {    
            true => Ok(self),
            false => bail!("Please supply a quality score (0-42) as u8"),
        }
    }
    fn is_phred64_score(&self) -> bool {
        PHRED64_SCORES.contains(self.as_ref())
    }
}

impl<T> CheckAsMutU8<T> for T where 
T: AsMut<u8>
{
    fn mut_check_percent(&mut self) -> Result<&mut Self> {
        match self.mut_is_percent() {    
            true => Ok(self),
            false => bail!("Please supply a valid PERCENT (0-100, not fractional) as u8"),
        }
    }
    fn mut_is_percent(&mut self) -> bool {
        PERCENT.contains(self.as_mut())
    }
    
    /// Validate a u8 is phred33 (33-75)
    fn mut_check_phred33(&mut self) -> Result<&mut Self> {
        match self.mut_is_phred33() {    
        true => Ok(self),
        false => bail!("Please supply a quality score (33-75, not fractional) as u8"),
        }
    }
    fn mut_is_phred33(&mut self) -> bool {
        PHRED33.contains(self.as_mut())
    }

    fn mut_check_phred33_score(&mut self) -> Result<&mut Self> {
        match self.mut_is_phred33_score() {    
            true => Ok(self),
            false => bail!("Please supply a quality score (0-42) as u8"),
        }
    }
    fn mut_is_phred33_score(&mut self) -> bool {
        PHRED33_SCORES.contains(self.as_mut())
    }
    /// Validate a u8 phred64 (64-126)
    fn mut_check_phred64(&mut self) -> Result<&mut Self> {
        match self.mut_is_phred64() {    
        true => Ok(self),
        false => bail!("Please supply a PHRED64 quality (64-126) as u8"),
        }
    }
    fn mut_is_phred64(&mut self) -> bool {
        PHRED64.contains(self.as_mut())
    }
    /// Validate a u8 phred64 score (0-62)
    fn mut_check_phred64_score(&mut self) -> Result<&mut Self> {
        match self.mut_is_phred64_score() {    
            true => Ok(self),
            false => bail!("Please supply a quality score (0-42) as u8"),
        }
    }
    fn mut_is_phred64_score(&mut self) -> bool {
        PHRED64_SCORES.contains(self.as_mut())
    }
}

