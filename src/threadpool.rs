//! 一个并行执行方法的线程池实现。
//!
//!  一个典型的线程池包含三个部分：
//!
//! 1. 工作线程（Worker Threads）：线程池中固定数量的线程，循环从任务队列中取任务执行
//! 2. 任务队列（Task Queue）：存放待执行任务，生产者将任务放入队列，工作线程从中取任务执行
//! 3. 任务调度器（Scheduler）：负责把任务分配给工作线程（可以是简单的FIFO，也可以是工作窃取等复杂策略）
//!
//! # 线程池设计要点
//!
//! ## 任务存放方式
//!
//! 被执行的任务应该是一个函数或闭包，需要满足以下要求：
//!
//! * `FnOnce() + Send + 'static`
//! * 必须可在线程间移动（`Send`），生成周期独立于创建线程（`'static`）
//!
//! ## 共享队列
//!
//! * 多线程同时读写任务队列 -> 需要同步/互斥机制
//! * 采用的方案： `crossbeam::channel` （无锁队列）
//!
//! ## 线程循环
//!
//! 每个工作线程是一个无限循环：
//!
//! ```rust
//! loop {
//!     let task = task_queue.pop();
//!     task();
//! }
//! ```
//!
//! 当线程池关闭时，需要发信号让线程跳出循环
//!
//! ## 线程复用
//!
//! * 避免频繁创建/消费线程
//! * 任务完成后线程继续等待下一个任务
//!
//! ## 关闭线程池
//!
//! 调用者希望线程池优雅关闭：`drop` 任务队列（所有接收端关闭）
//!
//! # 示例
//!
//! ```rust
//! use std::thread;
//! use crate::threadpool::ThreadPool;
//!
//! let threadpool = ThreadPool::new(4); // 创建容量为4的线程池
//!
//! for i in 0..8 {
//!     threadpool.execute(move || {
//!         println!("任务 {} 在线程 {:?}", i, thread::current().id());
//!     });
//! }
//!
//! threadpool.join(); // 等待所有任务完成
//! println!("所有任务完成");
//! ```

use std::thread;

use crossbeam::channel::{self, Sender};

#[allow(dead_code)]
pub type Job = Box<dyn FnOnce() + Send + 'static>;

#[allow(dead_code)]
pub struct ThreadPool {
    /// 工作线程组
    workers: Vec<thread::JoinHandle<()>>,
    /// 任务发送者
    sender: Option<Sender<Job>>,
}

#[allow(dead_code)]
impl ThreadPool {
    pub fn new(num_threads: usize) -> Self {
        let (sender, receiver) = channel::unbounded::<Job>();
        let workers: Vec<thread::JoinHandle<()>> = (0..num_threads)
            .map(|_| {
                let receiver = receiver.clone();
                thread::spawn(move || {
                    while let Ok(job) = receiver.recv() {
                        job();
                    }
                })
            })
            .collect();

        Self {
            workers,
            sender: Some(sender),
        }
    }

    /// 在线程池中执行 `task` 方法。
    ///
    /// # 示例
    ///
    /// 在一个容量为2的线程池中执行4个任务：
    ///
    /// ```
    /// use threadpool::ThreadPool;
    ///
    /// let pool = ThreadPool::new(2);
    /// pool.execute(|| println!("hello"));
    /// pool.execute(|| println!("world"));
    /// pool.execute(|| println!("foo"));
    /// pool.execute(|| println!("bar"));
    /// pool.join();
    /// ```
    pub fn execute<F>(&self, task: F)
    where
        F: FnOnce() + Send + 'static,
    {
        if let Some(sender) = &self.sender {
            sender
                .send(Box::new(task))
                .expect("ThreadPool::execute unable to send job into queue.");
        }
    }

    pub fn shutdown(&mut self) {
        // 取出 sender 并 drop
        self.sender.take();

        // 所有 worker 都会在 recv() 出错后推出循环
        for worker in self.workers.drain(..) {
            worker.join().unwrap();
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.shutdown();
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use crossbeam::channel;

    use super::ThreadPool;

    #[test]
    fn test_execute_task_in_threadpool() {
        let thread_pool = ThreadPool::new(4);

        for i in 0..8 {
            thread_pool.execute(move || {
                println!("任务 {i} 在线程 {:?}", thread::current().id());
            });
            thread::sleep(Duration::from_millis(20));
        }
    }

    #[test]
    fn test_execute_parallel_tasks_with_data() {
        let thread_pool = ThreadPool::new(2);
        let (tx, rx) = channel::unbounded::<i32>();

        for i in 0..=10 {
            let tx = tx.clone();
            thread_pool.execute(move || {
                tx.send(i).unwrap();
            });
        }

        drop(tx);

        let result: i32 = rx.iter().sum();
        assert_eq!(result, 55);
    }

    #[test]
    fn test_shutdown_threadpool() {
        let mut thread_pool = ThreadPool::new(4);

        for i in 0..10 {
            thread_pool.execute(move || {
                println!("运行任务 {i}（线程：{:?}）", thread::current().id());
                thread::sleep(Duration::from_millis(10));
            });
        }

        thread_pool.shutdown();
        println!("线程池已关闭。");
    }
}
