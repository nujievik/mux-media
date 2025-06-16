use crate::MaxValue;

impl MaxValue for u32 {
    const MAX: Self = u32::MAX;
}

impl MaxValue for u64 {
    const MAX: Self = u64::MAX;
}
