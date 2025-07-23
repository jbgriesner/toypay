#[derive(Debug, Clone, Copy)]
pub struct Transaction {
    pub(crate) client: u16,
    pub(crate) amount: u32,
    pub(crate) disputed: bool,
}
