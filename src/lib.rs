extern crate typenum;
#[macro_use]
extern crate generic_array;

use std::ops::{Add, Sub, Div};
use std::iter::{Peekable, Iterator, FromIterator, repeat};
use typenum::{U1, U2, U3, U4, U5, U6, U7, U8, Unsigned, Quot, Sum};
use generic_array::{ArrayLength, GenericArray};

pub trait PeekableIterator : std::iter::Iterator {
    fn peek(&mut self) -> Option<&Self::Item>;
}

impl<I: std::iter::Iterator> PeekableIterator for std::iter::Peekable<I> {
    fn peek(&mut self) -> Option<&Self::Item> {
        std::iter::Peekable::peek(self)
    }
}

#[derive(Debug, Clone)]
pub struct Bits<N>
where N: Unsigned,
      N: Add<U8>,
      Sum<N, U8>: Div<U8>,
      Quot<Sum<N, U8>, U8>: ArrayLength<u8>
{
    data: GenericArray<u8, Quot<Sum<N, U8>, U8>>
}

pub type Bits1 = Bits<U1>;
pub type Bits2 = Bits<U2>;
pub type Bits3 = Bits<U3>;
pub type Bits4 = Bits<U4>;
pub type Bits5 = Bits<U5>;
pub type Bits6 = Bits<U6>;
pub type Bits7 = Bits<U7>;
pub type Bits8 = Bits<U8>;
pub type Bit = Bits1;

impl<N> Bits<N>
where N: Unsigned,
      N: Add<U8>,
      Sum<N, U8>: Div<U8>,
      Quot<Sum<N, U8>, U8>: ArrayLength<u8> {

    pub fn len(&self) -> usize { N::to_usize() }

    pub fn consume_iter<'a>(offset: usize, iter: &mut PeekableIterator<Item=&'a u8>) -> Result<Bits<N>, &'static str>
    {
        // Skip until offset
        for _ in 0..(offset / 8) { iter.next(); }

        let shift = offset % 8;

        // TODO: (for both), determine when N < 8 (or shift + N < 8); if so, don't consume a byte at all unless N+shift wraps over the byte boundary
        if shift == 0 {
            let mut data: GenericArray<u8, Quot<Sum<N, U8>, U8>> = GenericArray::from_iter(repeat(0));
            let l = (N::to_usize() + 8) / 8;
            for i in 0..(l - 1) {
                data[i] = *iter.next().unwrap();
            }
            data[l-1] = **iter.peek().unwrap();

            Ok(Bits { data })
        }
        else {
            let rshift = 8 - shift;
            let rmask = (1 >> rshift) - 1;
            let mut data: GenericArray<u8, Quot<Sum<N, U8>, U8>> = GenericArray::from_iter(repeat(0));
            let l = (N::to_usize() + 8) / 8;
            for i in 0..l {
                data[i] = (*iter.next().unwrap() << shift) | ((**iter.peek().unwrap() & rmask) >> rshift);

            Ok(Bits { data })
        }
    }
}

#[macro_export]
macro_rules! bytes {
    [$($x:expr),*] => (
        Bits { data: arr![u8; $($x),*] }
    );
}

//fn shift_remaining(offset: usize, iter: &mut Iterator) {
//
//}

//macro_rules! unpack_iter {
//    ($($x: ident, $l: Unsigned), $offset: usize, $iter: Iterator<u8>) => ({
//        let $x : Bits<$l> = Bits::from_iter($offset, $iter);
//    });
//    ($($x: ident, $l: Unsigned), *, $offset: usize, $iter: Iterator<u8>) => ({
//        unpack_iter!(($x, $l), $offset, $iter);
//        unpack_iter!(*, ($offset + $l::to_usize()), $iter);
//    });
//}
//
//#[macro_export]
//macro_rules! unpack {
//    (($($x: ident, $l: Unsigned), *, $rem: ident) = $it: IntoIterable<u8>) => {
//        let iter = it.iter().peekable();
//        unpack_iter!(($x, $l), *, U0, $rem)
//        let $rem : Vec<u8> = FromIterable(iter);
//    }
//}

#[cfg(test)]
mod tests {
    use typenum::{U3, U7, U8, U9, U17, Unsigned};
    use Bits;

    #[test]
    fn struct_size() {
        let bits: Bits<U17> = Bits { data: arr![u8; 0, 0, 0] };
        assert_eq!(bits.data.len(), 3);
        assert_eq!(bits.len(), 17);
    }

    #[test]
    fn byte_macro() {
        let bits: Bits<U17> = bytes![0, 0, 0];
        assert_eq!(bits.data.len(), 3);
        assert_eq!(bits.len(), 17);
    }

    #[test]
    fn bits_from_iter() {
        let bytes: Vec<u8> = vec![0xAA, 0xAA];
        let mut iter = bytes.iter().peekable();
        let bits0: Bits<U8> = Bits::consume_iter(0, &mut iter).ok().expect("Failed to get first 8 bits out");
        let bits1: Bits<U7> = Bits::consume_iter(1, &mut iter).ok().expect("Failed to skip 1 and get next 7 bits out");
    }

    // TODO: Test four cases of bit shifts: offset == or != 0, and N (+shift) >= or < 8

    #[test]
    fn test_shift_remaining() {

    }

    #[test]
    fn unpack_macro() {
//        let bits: Bits<U17> = bytes![0xAA, 0xAA, 0xAA];
    }

    #[test]
    fn help() {
        assert_eq!(U3::to_u32(), 3);
        assert_eq!(U3::I32, 3);
    }
}
