#[derive(Debug)]
pub struct Shards<T> {
    pub(crate) shards: Vec<T>,
    pub(crate) shard_count: usize,
}

impl<T> Shards<T> {
    pub fn new(shards: Vec<T>) -> Self {
        let shard_count = shards.len();
        Self {
            shards,
            shard_count,
        }
    }

    pub fn shard_id(&self, client_id: u16) -> usize {
        (client_id as usize) % self.shard_count
    }

    pub fn get_shard(&mut self, client_id: u16) -> &mut T {
        let shard_id = self.shard_id(client_id);
        &mut self.shards[shard_id]
    }

    pub fn shards_slices(&self) -> &[T] {
        &self.shards
    }
}
