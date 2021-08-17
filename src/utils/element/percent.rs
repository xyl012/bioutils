//! Calculates percents and checks whether the percent is 0-100.
//! Intakes self as u8, with target use each element of a slice

use super::*;
use super::tryfrom::*;
use anyhow::{Result, Error, bail};

/// Intakes self as u8 and denominator as u8, returns percent as a u8. 
pub trait PercentAsRefU8 {
    /// Returns the percent of u8s as u8
    fn percent_u8(&self, denominator: &u8) -> Result<u8>;
}

impl PercentAsRefU8 for u8
{
    /// Returns the percent of u8s as u8
    fn percent_u8(&self, denominator: &u8) -> Result<u8> {
    let percent = (100 * u8::from(*self) + denominator / 2) / denominator;
    match PercentU8::try_from(percent) {
        Ok(_) => Ok(percent),
        Err(_) => bail!("Percent not valid or within the range 0-100"),
    }}
}

/// Intakes self as u8 and denominator as usize, returns percent as a usize. 
pub trait PercentAsRefUsize {
    /// Returns the percent of u8 and usize as usize
    fn percent_usize(&self, denominator: &usize) -> Result<usize>;
}

impl PercentAsRefUsize for u8
{
    /// Returns the percent of u8 and usize as usize
    fn percent_usize(&self, denominator: &usize) -> Result<usize> {
    let percent = (100 * usize::try_from(*self)? + denominator / 2) / denominator;
    match PercentUsize::try_from(percent) {
        Ok(_) => Ok(percent as usize),
        Err(_) => bail!("Percent not valid or within the range 0-100"),
    }}
}

/// Intakes self as u8 and denominator as u64, returns percent as a u64.
pub trait PercentAsRefU64 {
    /// Returns the percent of u8 and u64 as u64
    fn percent_u64(&self, denominator: &u64) -> Result<u64>;
}

impl PercentAsRefU64 for u8
{
    /// Returns the percent of u8 and u64 as u64
    fn percent_u64(&self, denominator: &u64) -> Result<u64> {
    let percent = (100 * u64::from(*self) + denominator / 2) / denominator;
    match PercentU64::try_from(percent) {
        Ok(_) => Ok(percent as u64),
        Err(_) => bail!("Percent not valid or within the range 0-100"),
    }}
}

impl PercentAsRefU64 for u64
{
    /// Returns the percent of u64 and u64 as u64
    fn percent_u64(&self, denominator: &u64) -> Result<u64> {
    let percent = (100 * self + denominator / 2) / denominator;
    match PercentU64::try_from(percent) {
        Ok(_) => Ok(percent as u64),
        Err(_) => bail!("Percent not valid or within the range 0-100"),
    }}
}

/// Calculates a percent with usize numerator and denominator
pub fn percent_u8(numerator: &u8, denominator: &u8) -> Result<u8> {
    let percent = (100 * numerator + denominator / 2) / denominator;
    match PercentU8::try_from(percent) {
        Ok(_) => Ok(percent),
        Err(_) => bail!("Percent not valid or within the range 0-100"),
    }
}

/// Calculates a percent with usize numerator and denominator
pub fn percent_usize(numerator: &usize, denominator: &usize) -> Result<usize> {
    let percent = (100 * numerator + denominator / 2) / denominator;
    match PercentUsize::try_from(percent) {
        Ok(_) => Ok(percent),
        Err(_) => bail!("Percent not valid or within the range 0-100"),
    }
}

/// Calculates a percent with u64 numerator and denominator
pub fn percent_u64(numerator: &u64, denominator: &u64) -> Result<u64> {
    let percent = (100 * numerator + denominator / 2) / denominator;
    match PercentU64::try_from(percent) {
        Ok(_) => Ok(percent),
        Err(_) => bail!("Percent not valid or within the range 0-100"),
    }
}

