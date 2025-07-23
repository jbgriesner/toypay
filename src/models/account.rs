use rust_decimal::Decimal;

#[derive(Debug, Clone, Copy)]
pub struct Account {
    pub(crate) available: u32,
    pub(crate) held: u32,
    pub(crate) locked: bool,
}

impl Account {
    pub fn new() -> Self {
        Account {
            available: 0,
            held: 0,
            locked: false,
        }
    }

    pub(crate) fn available_as_decimal(&self) -> Decimal {
        Decimal::from(self.available) / Decimal::from(100)
    }

    pub(crate) fn held_as_decimal(&self) -> Decimal {
        Decimal::from(self.held) / Decimal::from(100)
    }

    pub(crate) fn total_as_decimal(&self) -> Decimal {
        Decimal::from(self.available + self.held) / Decimal::from(100)
    }
}
