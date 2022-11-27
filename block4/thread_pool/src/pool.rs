use crate::error::Result;
use log::debug;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

/// 线程池
pub struct ThreadPool<F>
where
    F: FnOnce() + 'static + Send,
{
    cond: Arc<Condvar>,
    stopped: Arc<Mutex<Box<bool>>>,
    tasks: Arc<Mutex<Vec<F>>>,
    handles: Vec<thread::JoinHandle<()>>,
}

impl<F> ThreadPool<F>
where
    F: FnOnce() + 'static + Send,
{
    /// 创建一个线程池实例
    pub fn new(threads: u32) -> Result<Self> {
        let cond = Arc::new(Condvar::new());
        let tasks = Arc::new(Mutex::new(Vec::<F>::new()));
        let stopped = Arc::new(Mutex::new(Box::new(false)));

        let mut handles = Vec::new();
        for id in 0..threads {
            let tasks = Arc::clone(&tasks);
            let stopped = Arc::clone(&stopped);
            let cond = Arc::clone(&cond);
            let handle = thread::Builder::new().name(id.to_string()).spawn(move || {
                let mut stopped = stopped.lock().unwrap();
                while !(stopped.as_ref()) {
                    stopped = cond.wait(stopped).unwrap();
                    if let Some(task) = tasks.lock().unwrap().pop() {
                        debug!("thread[{}] receive a task", id);
                        task();
                    }
                }
            })?;
            handles.push(handle);
        }
        Ok(Self {
            cond,
            stopped,
            tasks,
            handles,
        })
    }

    /// 分配线程执行 task
    pub fn spawn(&mut self, task: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let stopped = self.stopped.lock().unwrap();
        if !stopped.as_ref() {
            let mut tasks = self.tasks.lock().unwrap();
            tasks.push(task);
            self.cond.notify_one();
        }
    }

    /// 关闭协程池，释放资源
    pub fn stop(&mut self) {
        {
            let mut stopped = self.stopped.lock().unwrap();
            if !stopped.as_ref() {
                *stopped = Box::new(true);
                self.cond.notify_all();
            }
        }
        while let Some(handle) = self.handles.pop() {
            handle.join().unwrap();
        }
    }
}
