
#[derive(Debug, Copy, Clone)]
pub enum OFSize {
    U8,
    U16,
    U32,
    U48,
    U64
}

pub fn optimal_field_size(num: u64) -> OFSize {
    if num < u8::max_value() as u64 {
        OFSize::U8
    } else if num < u16::max_value() as u64 {
        OFSize::U16
    } else if num < u32::max_value() as u64 {
        OFSize::U32
    }  else if num < 2u64.pow(48) - 1 {
        OFSize::U48
    } else {
        OFSize::U64
    }
}