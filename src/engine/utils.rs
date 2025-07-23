use anyhow::{anyhow, Result};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;

pub trait DecimalToU32 {
    fn decimal_to_u32(self) -> Result<u32>;
}

impl DecimalToU32 for Decimal {
    fn decimal_to_u32(self) -> Result<u32> {
        if self < Decimal::ZERO {
            return Err(anyhow!("Amount cannot be negative"));
        }

        const MAX_AMOUNT: Decimal = Decimal::from_parts(4294967295, 0, 0, false, 2);
        if self > MAX_AMOUNT {
            return Err(anyhow!("Amount too large"));
        }

        let result = (self * Decimal::from(100))
            .to_u32()
            .ok_or_else(|| anyhow!("Invalid amount precision"))?;

        Ok(result)
    }
}
