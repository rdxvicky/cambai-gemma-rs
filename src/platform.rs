#[cfg(target_os = "linux")]
use jemallocator::Jemalloc;
#[cfg(target_os = "linux")]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

pub fn threads_hint() -> usize {
    let n = num_cpus::get_physical();
    (n.max(2)).min(8)
}