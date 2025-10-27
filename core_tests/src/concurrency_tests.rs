use std::{
    hint::spin_loop,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};

/// 一个简单的自旋锁（SpinLock）实现
///
/// # 特点
/// - 基于 `AtomicBool` 实现，线程安全
/// - 忙等待（spin）式锁，不会让出 CPU
/// - 适用于锁持有时间极短的场景（如计数器、自定义同步原语）
///
/// # 内存语义
/// - `Ordering::Acquire`：确保在成功获取锁后，后续操作看到锁之前的所有写入。
/// - `Ordering::Release`：确保在释放锁前的写入对之后获取锁的线程可见。
///
/// # 示例
/// ```
/// use std::sync::Arc;
/// use std::thread;
///
/// // 定义自旋锁
/// use crate::concurrency_tests::SpinLock; // 实际使用时替换为正确模块路径
///
/// let lock = Arc::new(SpinLock::new());
/// let counter = Arc::new(std::sync::atomic::AtomicUsize::new(0));
///
/// // 启动多个线程竞争锁
/// let mut handles = vec![];
/// for _ in 0..4 {
///     let lock_clone = Arc::clone(&lock);
///     let counter_clone = Arc::clone(&counter);
///     handles.push(thread::spawn(move || {
///         for _ in 0..1000 {
///             lock_clone.lock();              // 加锁
///             counter_clone.fetch_add(1, Ordering::Relaxed);
///             lock_clone.unlock();            // 解锁
///         }
///     }));
/// }
///
/// for h in handles {
///     h.join().unwrap();
/// }
///
/// assert_eq!(counter.load(Ordering::Relaxed), 4000);
/// println!("最终计数结果：{}", counter.load(Ordering::Relaxed));
/// ```
///
/// 输出（顺序可能不同）：
/// ```text
/// 最终计数结果：4000
/// ```
#[allow(dead_code)]
struct SpinLock {
    /// 是否已上锁
    locked: AtomicBool,
}

#[allow(dead_code)]
impl SpinLock {
    /// 创建一个未上锁的自旋锁
    fn new() -> Self {
        Self { locked: AtomicBool::new(false) }
    }

    /// 获取锁（阻塞直到成功）
    ///
    /// - 使用 `compare_exchange` 将 `locked` 从 `false` 改为 `true`。
    /// - 如果失败（锁已被占用），进入自旋等待。
    fn lock(&self) {
        while self
            .locked
            .compare_exchange(
                false,             // 期望值（未上锁）
                true,              // 新值（上锁）
                Ordering::Acquire, // 确保在成功获取锁后，后续操作看到锁之前的所有写入。
                Ordering::Relaxed, // 获取锁失败时，不需要任何额外的内存同步保证。
            )
            .is_err()
        {
            // 自旋等待（忙等）
            std::hint::spin_loop();
        }
    }

    /// 释放锁
    ///
    /// - 直接将 `locked` 置为 `false`。
    /// - 使用 `Ordering::Release` 保证写入可见性。
    fn unlock(&self) {
        self.locked.store(false, Ordering::Release); // `Ordering::Release`：确保在释放锁前的写入对之后获取锁的线程可见。
    }
}

/// 无锁版计数器
#[allow(dead_code)]
struct Counter {
    value: AtomicUsize,
}

#[allow(dead_code)]
impl Counter {
    fn new() -> Self {
        Self { value: AtomicUsize::new(0) }
    }

    fn increment(&self) {
        // 以下实现等价于 self.value.fetch_add(1);
        let mut current = self.value.load(Ordering::Relaxed);

        loop {
            let new = current + 1;
            match self.value.compare_exchange(current, new, Ordering::SeqCst, Ordering::Relaxed) {
                Ok(_) => break,
                Err(v) => {
                    current = v;
                    spin_loop();
                }
            }
        }
    }

    fn get(&self) -> usize {
        self.value.load(Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use std::{
        pin::Pin,
        sync::{
            Arc, Mutex, RwLock,
            atomic::{AtomicUsize, Ordering},
            mpsc::{RecvError, SendError, channel},
        },
        task::{Context, Poll},
        time::Duration,
    };

    use super::{Counter, SpinLock};

    #[test]
    fn test_concurrency_move() {
        let v = vec![1, 2, 3];

        let handle = std::thread::spawn(move || {
            println!("线程中的向量: {:?}", v);
        });

        // println!("{v:?}");   // 编译报错：borrow of moved value: `v`

        handle.join().unwrap();
    }

    #[test]
    fn test_concurrency_borrow() {
        let v = Arc::new(vec![1, 2, 3]);

        let handle = std::thread::spawn({
            let v = Arc::clone(&v);
            move || {
                println!("线程中的向量：{:?}", v);
            }
        });

        handle.join().unwrap();
        assert_eq!(v.as_ref(), &vec![1, 2, 3]);
    }

    #[test]
    fn test_concurrency_multithread() {
        let handles: Vec<_> = (0..3)
            .map(|i| {
                std::thread::spawn(move || {
                    println!("{i}");
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn test_concurrency_mut_borrow() {
        let v = Arc::new(Mutex::new(vec![1, 2, 3]));

        let handle = std::thread::spawn({
            let v = Arc::clone(&v);

            move || {
                if let Ok(mut v) = v.lock() {
                    v.push(4);
                } else {
                    eprintln!("锁中毒，跳过操作");
                }
            }
        });

        handle.join().unwrap();

        let v = v.lock().unwrap();
        println!("{v:?}");
        assert_eq!(*v, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_concurrency_mut_borrow_for_multithread() {
        let v = Arc::new(Mutex::new(vec![0]));

        let handles = (1..4).map(|i| {
            let v = Arc::clone(&v);
            std::thread::spawn(move || {
                if let Ok(mut v) = v.lock() {
                    v.push(i);
                } else {
                    eprintln!("锁中毒，跳过操作");
                }
            })
        });

        for handle in handles {
            handle.join().unwrap();
        }

        let v = v.lock().unwrap();
        assert!(v.contains(&1));
        assert!(v.contains(&2));
        assert!(v.contains(&3));
    }

    #[test]
    fn test_concurrency_rwlock() {
        let val = Arc::new(RwLock::new(1));
        let mut handles = Vec::new();

        for i in 0..3 {
            let val = Arc::clone(&val);
            std::thread::spawn(move || {
                let val = *val.read().unwrap();
                println!("读线程 {i} 读取到：{val}");
            });
        }

        {
            let val = Arc::clone(&val);
            handles.push(std::thread::spawn(move || {
                let mut val = val.write().unwrap();
                *val = 2;
                println!("写线程修改为: {val}");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(*val.read().unwrap(), 2);
    }

    /// mpsc::channel 的要点：
    ///
    /// - 多个生产者可以发送消息
    /// - 一个消费者接收消息
    #[test]
    fn test_concurrency_mpsc_channel() {
        let (tx, rx) = channel();

        // 多个生产者发送消息
        for i in 0..3 {
            let tx = tx.clone();
            std::thread::spawn(move || {
                tx.send(i).unwrap();
            });
        }

        drop(tx); // 主线程丢弃发送端，确保消费者循环能正常结束

        // 主线程接收消息
        for received in rx {
            println!("接收到消息：{}", received);
        }
    }

    #[test]
    fn test_concurrency_crossbeam_channel_unbounded() {
        let (tx, rx) = crossbeam::channel::unbounded::<String>(); // 无界通道

        for i in 0..5 {
            let tx1 = tx.clone();
            std::thread::spawn(move || {
                for j in 0..3 {
                    tx1.send(format!("event {i}_{j}")).unwrap();
                }
            });
        }

        drop(tx);

        for received in rx {
            println!("接收到消息：{received}");
        }

        println!("所有消息处理完成");
    }

    #[test]
    fn test_concurrency_crossbeam_channel_bounded() {
        let (tx, rx) = crossbeam::channel::bounded::<String>(3);

        for i in 0..5 {
            let tx = tx.clone();
            std::thread::spawn(move || {
                for j in 0..3 {
                    tx.send(format!("event {i}_{j}")).unwrap();
                }
            });
        }

        drop(tx);

        for received in rx {
            println!("接收到消息：{received}");
        }
        println!("消息处理完成");
    }

    /// 多生产者多消费者：消息分发消费模式（负载均衡）
    #[test]
    fn test_concurrency_crossbeam_channel_load_balancing() {
        let (tx, rx) = crossbeam::channel::bounded::<String>(3);

        for i in 0..3 {
            let tx = tx.clone();
            std::thread::spawn(move || {
                for j in 0..5 {
                    tx.send(format!("msg {i}_{j}")).unwrap();
                    std::thread::sleep(Duration::from_millis(20));
                }
            });
        }

        let handles: Vec<_> = (0..3)
            .map(|i| {
                let rx = rx.clone();
                std::thread::spawn(move || {
                    for msg in rx {
                        println!("消费者{i}收到：{msg}");
                    }
                })
            })
            .collect();

        drop(tx);

        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn test_concurrency_crossbeam_select() {
        let (tx, rx) = crossbeam::channel::unbounded::<String>();
        let (stop_tx, stop_rx) = crossbeam::channel::bounded::<()>(0);

        // 消费者：在子线程中接收消息
        let handle = std::thread::spawn(move || {
            loop {
                crossbeam::select! {
                    recv(rx) -> msg => match msg {
                        Ok(v) => println!("接收任务：{v}"),
                        Err(_) => {
                            println!("任务通道已关闭，退出");
                            break;
                        }
                    },
                    recv(stop_rx) -> _ => {
                        println!("收到停止信号");
                        break;
                    }
                }
            }
        });

        for i in 0..3 {
            tx.send(format!("任务 {i}")).unwrap();
            std::thread::sleep(Duration::from_millis(10));
        }

        stop_tx.send(()).unwrap();

        handle.join().unwrap();
        println!("主线程结束");
    }

    #[test]
    fn test_concurrency_mpsc_channel_drop_rx() {
        let (tx, rx) = channel();
        drop(rx);

        assert_eq!(tx.send(7), Err(SendError(7)));
    }

    #[test]
    fn test_concurrency_mpsc_channel_drop_tx() {
        let (tx, rx) = channel();

        tx.send(7).unwrap();
        drop(tx);

        assert_eq!(rx.recv(), Ok(7));
        assert_eq!(rx.recv(), Err(RecvError));
    }

    #[test]
    fn test_concurrency_mpsc_channel_try_recv() {
        let (tx, rx) = channel::<()>();
        drop(tx);

        assert_eq!(rx.try_recv(), Err(std::sync::mpsc::TryRecvError::Disconnected));
    }

    #[test]
    fn test_concurrency_spinlock() {
        let lock = Arc::new(SpinLock::new());
        let counter = Arc::new(AtomicUsize::new(0));

        let handles: Vec<_> = (0..4)
            .map(|_| {
                let lock = Arc::clone(&lock);
                let counter = Arc::clone(&counter);
                std::thread::spawn(move || {
                    for _ in 0..10000 {
                        lock.lock();
                        counter.fetch_add(1, Ordering::Relaxed); // 这里可以使用 Ordering::Relexed，因为已经上锁了。
                        lock.unlock();
                    }
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(counter.load(Ordering::Relaxed), 40000); // 这里可以使用 Ordering::Relexed，因为已经上锁了。
    }

    #[test]
    fn test_concurrency_counter() {
        let counter = Arc::new(Counter::new());

        let handles = (0..4).map(|_| {
            let counter = counter.clone();

            std::thread::spawn(move || {
                for _ in 0..10000 {
                    counter.increment();
                }
            })
        });

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(counter.get(), 40000);
    }

    /// 一个简单的 Future 执行器
    fn run_future<T: Future>(mut fut: T) -> T::Output {
        use std::task::{RawWaker, RawWakerVTable, Waker};

        // 一个假的 waker（仅供示例，不会真正唤醒）
        fn dummy_waker() -> Waker {
            fn clone(_: *const ()) -> RawWaker {
                RawWaker::new(std::ptr::null(), &VTABLE)
            }
            fn wake(_: *const ()) {}
            fn wake_by_ref(_: *const ()) {}
            fn drop(_: *const ()) {}

            static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);

            unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) }
        }

        let waker = dummy_waker();
        let mut ctx = Context::from_waker(&waker);
        let mut fut = unsafe { Pin::new_unchecked(&mut fut) };

        loop {
            match fut.as_mut().poll(&mut ctx) {
                Poll::Pending => continue,
                Poll::Ready(val) => return val,
            }
        }
    }

    /// Future 是异步编程原语，表示将来会产生一个结果的对象，
    /// 它不会立即执行，而是要等到执行器去”推动“它才会运行。
    /// 执行器推动时Future会返回任务的当前状态。
    ///
    /// Future 就是异步任务的状态机。
    ///
    /// 注意：之所以要使用 `Pin` 将  `future` 在内存中盯住（即不能移动它在内存中的位置），因为 Future 会有自引用现象，
    /// 执行器在推动过程中，如果发生了内存移动，那么就会出现UB错误。
    #[test]
    fn test_concurrency_future_hello() {
        /// 定义异步任务Future
        struct MyFuture {
            polled_times: u8,
        }

        impl Future for MyFuture {
            type Output = u8;

            fn poll(
                self: std::pin::Pin<&mut Self>,
                _: &mut std::task::Context<'_>,
            ) -> std::task::Poll<Self::Output> {
                let this = self.get_mut();
                if this.polled_times < 2 {
                    this.polled_times += 1;
                    println!("Not redy yet... (poll count = {})", this.polled_times);
                    Poll::Pending
                } else {
                    println!("Ready!");
                    Poll::Ready(42)
                }
            }
        }

        let fut = MyFuture { polled_times: 0 };

        assert_eq!(run_future(fut), 42);
    }

    /// async fn 被编译器自动转换为一个实现了 Future trait 的状态机结构体
    #[test]
    fn test_concurrency_future_async_fn() {
        async fn say_hello() -> u32 {
            println!("Hello");
            42
        }

        let fut = say_hello();

        assert_eq!(run_future(fut), 42);
    }

    /// .await 是当前状态机的”挂起点”。挂起当前 Future，让执行器去执行子 Future 任务
    #[test]
    fn test_concurrency_await() {
        async fn foo() -> u32 {
            println!("foo");
            1
        }

        async fn bar() -> u32 {
            let foo = foo().await; // 挂起 bar() Future，让执行器poll foo() Future，并等待 foo 完成后继续执行后续操作
            println!("bar");
            foo + 2
        }

        let fut = bar();
        assert_eq!(run_future(fut), 3);
    }

    /// 自引用结构体的悬垂引用问题
    #[test]
    fn test_concurrency_pin_1() {
        struct SelfReferential {
            value: String,
            pointer_to_value: Option<*const String>,
        }

        impl SelfReferential {
            fn new(value: &str) -> Self {
                let value = value.to_string();
                SelfReferential { value, pointer_to_value: None }
            }

            fn init(&mut self) {
                self.pointer_to_value = Some(&self.value);
            }
        }

        let mut thing1 = SelfReferential::new("hello");
        thing1.init();

        let p1 = thing1.pointer_to_value;
        let p2 = Some(&thing1.value as *const String);
        assert!(p1 == p2);
        println!("{:x}", thing1.value.as_ptr() as usize);

        let thing2 = thing1;

        let p1 = thing2.pointer_to_value;
        let p2 = Some(&thing2.value as *const String);
        assert!(p1 != p2); // pointer_to_value 存储的value内存地址已经与 value 内存地址不一致了，形成悬垂指针

        println!("{:x}", thing2.value.as_ptr() as usize);
    }

    #[tokio::test]
    async fn test_join_aync_tasks() {
        let t1 = tokio::spawn(async { 1 });
        let t2 = tokio::spawn(async { 2 });

        let (v1, v2) = tokio::join!(t1, t2);

        assert!(v1.is_ok_and(|v| v == 1));
        assert!(v2.is_ok_and(|v| v == 2));
    }
}
