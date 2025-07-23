use std::thread::available_parallelism;

pub fn get_cpus() -> usize {
    available_parallelism().map(|n| n.get()).unwrap_or(4)
}
