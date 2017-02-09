use time;

fn since(time: &time::Tm) -> time::Duration {
    let now = time::now().to_timespec();
    let tm_spec = time.to_timespec();
    now - tm_spec
}
