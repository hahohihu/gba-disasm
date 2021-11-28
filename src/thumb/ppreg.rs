use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::{get_bit, get_bits};

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
enum PCLR { 
    DoNotStoreLoad = 0,
    StoreLoad = 1
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
enum LoadStore { 
    Store = 0,
    Load = 1
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PushPopRegisters { 
    pclr: PCLR,
    mode: LoadStore,
    rlist: u8
}

impl From<u16> for PushPopRegisters { 
    fn from(raw: u16) -> Self { 
        debug_assert!(get_bits!(raw, 15..12) == 0b1011);
        debug_assert!(get_bits!(raw, 10..9) == 0b10);

        Self { 
            pclr: FromPrimitive::from_u8(get_bit(raw, 8) as u8).unwrap(),
            mode: FromPrimitive::from_u8(get_bit(raw, 11) as u8).unwrap(),
            rlist: get_bits!(raw, 7..0) as u8,
        }
    }
}


#[cfg(test)]
mod test { 
    use crate::thumb::ppreg::{PushPopRegisters, PCLR, LoadStore};
    use num_traits::FromPrimitive;
    use test_case::test_case;

    #[test_case(0b1011_0_10000000000, LoadStore::Store ;  "Store")]
    #[test_case(0b1011_1_10000000000, LoadStore::Load  ;   "Load")]
    fn load_store(input: u16, expected: LoadStore) { 
        assert_eq!(PushPopRegisters::from(input).mode, expected);
    }


    #[test_case(0b1011010_0_00000000, PCLR::DoNotStoreLoad ; "DoNotStoreLoad")]
    #[test_case(0b1011010_1_00000000, PCLR::StoreLoad ; "StoreLoad")]
    fn pclr(input: u16, expected: PCLR) {
        assert_eq!(PushPopRegisters::from(input).pclr, expected)
    }

    use proptest::prelude::*;

    proptest! { 
        #[test]
        fn props(
            pclr in 0..1 as u16,
            mode in 0..1 as u16,
            rlist in 0..0b11111111 as u16
        ) {
            let ppreg = PushPopRegisters {
                pclr: FromPrimitive::from_u8(pclr as u8).unwrap(),
                mode: FromPrimitive::from_u8(mode as u8).unwrap(),
                rlist: rlist as u8
            };

            let base = 0b1011_0100_0000_0000;
            let decoded_instruction = PushPopRegisters::from(base | (pclr << 8) | (mode << 11) | rlist);

            prop_assert_eq!(ppreg, decoded_instruction);
        }
    }
}