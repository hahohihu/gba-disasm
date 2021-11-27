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
struct PushPopRegisters { 
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
