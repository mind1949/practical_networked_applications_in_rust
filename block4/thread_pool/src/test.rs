use log::info;

#[test]
fn new_thread_pool() {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Debug)
        .try_init()
        .unwrap();
    let mut pool = crate::pool::ThreadPool::new(10).unwrap();
    for i in 0..5 {
        let i = i;
        pool.spawn(move || {
            info!("exec task[{}]", i);
        })
    }
    std::thread::sleep(std::time::Duration::from_secs(3));
    pool.stop();
}
