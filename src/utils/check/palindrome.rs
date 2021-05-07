// Copyright (c) 2021 Kana LLC

pub trait CheckPalindrome<T> {
    /// Generic trait to check if T is a palindrome
    fn is_palindrome(&self) -> bool;
}

impl<T> CheckPalindrome<T> for T
where
    T: IntoIterator,
    T::Item: PartialEq,
    T::IntoIter: DoubleEndedIterator,
    T: Copy,
{
    /// Generic impl to check if T is a palindrome
    fn is_palindrome(&self) -> bool {
    let mut iter = self.into_iter();
    while let (Some(front), Some(back)) = (iter.next(), iter.next_back()) {
        if front != back {
            return false;
        }
    }
    true
    }
    //TODO palindrome with error 1
}


/// Generic function to check if T is a palindrome.
pub fn is_palindrome<T>(iterable: T) -> bool
where
    T: IntoIterator,
    T::Item: PartialEq,
    T::IntoIter: DoubleEndedIterator,
{
    let mut iter = iterable.into_iter();
    while let (Some(forward), Some(backward)) = (iter.next(), iter.next_back()) {
        if forward != backward {
            return false;
        }
    }
    true
}
