// Re-export time crate
pub use time::*;
use time;

pub fn since(time: &time::Tm) -> time::Duration {
    let now = time::now().to_timespec();
    let tm_spec = time.to_timespec();
    now - tm_spec
}
