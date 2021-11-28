use contracts::debug_requires;
use num_traits::FromPrimitive;
use crate::{get_bit, get_bits, types::{Register, LoadStore, ByteWord}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LoadStoreReg {
    bw: ByteWord,
    ls: LoadStore,
    offset: Register,
    base: Register,
    target: Register,
}

impl From<u16> for LoadStoreReg { 
    #[debug_requires(get_bits!(raw, 15..12) == 0b0101)]
    #[debug_requires(get_bit(raw, 9) == 0)]
    fn from (raw: u16) -> Self { 
        Self {
            target: Register(get_bits!(raw, 2..0) as u8),
            base: Register(get_bits!(raw, 5..3) as u8),
            offset: Register(get_bits!(raw, 8..6) as u8),
            ls: FromPrimitive::from_u8(get_bit(raw, 11)).unwrap(),
            bw: FromPrimitive::from_u8(get_bit(raw, 10)).unwrap(),
        }
    }
}


#[cfg(test)]
mod test {
    use super::LoadStoreReg;
    use crate::types::{Register, LoadStore, ByteWord};
    use test_case::test_case;

    #[test_case(ByteWord::Byte, 1)]
    #[test_case(ByteWord::Word, 0)]
    fn byteword(bw: ByteWord, bit: u16) {
        let lsr = LoadStoreReg::from(0b0101_0_0_0_000_000_000 | bit << 10);
        assert_eq!(lsr.bw, bw);
    }

    #[test_case(LoadStore::Load, 1)]
    #[test_case(LoadStore::Store, 0)]
    fn loadstore(ls: LoadStore, bit: u16) {
        let lsr = LoadStoreReg::from(0b0101_0_0_0_000_000_000 | bit << 11);
        assert_eq!(lsr.ls, ls);
    }

    use proptest::prelude::*;
    proptest! {
        #[test]
        fn values_match(
            offset in 0..(1 << 3) as u16,
            base in 0..(1 << 3) as u16,
            target in 0..(1 << 3) as u16,
        ) {
            let lsr = LoadStoreReg::from(0b0101_0_0_0_000_000_000 | (offset << 6) | (base << 3) | (target));
            assert_eq!(lsr.offset, Register(offset as u8));
            assert_eq!(lsr.base, Register(base as u8));
            assert_eq!(lsr.target, Register(target as u8));
        }
    }
}


