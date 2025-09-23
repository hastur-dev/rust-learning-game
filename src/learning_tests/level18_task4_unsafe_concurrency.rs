// Learning Tests for Level 18, Task 4: Unsafe Concurrency
// Implementing Send/Sync, atomic operations, and lock-free data structures

use std::sync::atomic::{AtomicU32, AtomicUsize, AtomicBool, AtomicPtr, Ordering};
use std::sync::Arc;
use std::thread;
use std::ptr;
use std::cell::UnsafeCell;
use std::marker::PhantomData;

// Custom mutex implementation
pub struct UnsafeMutex<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for UnsafeMutex<T> {}
unsafe impl<T: Send> Sync for UnsafeMutex<T> {}

impl<T> UnsafeMutex<T> {
    pub fn new(data: T) -> Self {
        UnsafeMutex {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock(&self) -> UnsafeMutexGuard<T> {
        while self.locked.compare_exchange_weak(
            false,
            true,
            Ordering::Acquire,
            Ordering::Relaxed
        ).is_err() {
            std::hint::spin_loop();
        }

        UnsafeMutexGuard {
            mutex: self,
            _phantom: PhantomData,
        }
    }

    pub fn try_lock(&self) -> Option<UnsafeMutexGuard<T>> {
        if self.locked.compare_exchange(
            false,
            true,
            Ordering::Acquire,
            Ordering::Relaxed
        ).is_ok() {
            Some(UnsafeMutexGuard {
                mutex: self,
                _phantom: PhantomData,
            })
        } else {
            None
        }
    }

    unsafe fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }
}

pub struct UnsafeMutexGuard<'a, T> {
    mutex: &'a UnsafeMutex<T>,
    _phantom: PhantomData<&'a mut T>,
}

impl<'a, T> Drop for UnsafeMutexGuard<'a, T> {
    fn drop(&mut self) {
        unsafe {
            self.mutex.unlock();
        }
    }
}

impl<'a, T> std::ops::Deref for UnsafeMutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.data.get() }
    }
}

impl<'a, T> std::ops::DerefMut for UnsafeMutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mutex.data.get() }
    }
}

// Lock-free stack implementation
pub struct LockFreeStack<T> {
    head: AtomicPtr<Node<T>>,
}

struct Node<T> {
    data: T,
    next: *mut Node<T>,
}

unsafe impl<T: Send> Send for LockFreeStack<T> {}
unsafe impl<T: Send> Sync for LockFreeStack<T> {}

impl<T> LockFreeStack<T> {
    pub fn new() -> Self {
        LockFreeStack {
            head: AtomicPtr::new(ptr::null_mut()),
        }
    }

    pub fn push(&self, data: T) {
        let new_node = Box::into_raw(Box::new(Node {
            data,
            next: ptr::null_mut(),
        }));

        loop {
            let head = self.head.load(Ordering::Acquire);
            unsafe {
                (*new_node).next = head;
            }

            if self.head.compare_exchange_weak(
                head,
                new_node,
                Ordering::Release,
                Ordering::Acquire
            ).is_ok() {
                break;
            }
        }
    }

    pub fn pop(&self) -> Option<T> {
        loop {
            let head = self.head.load(Ordering::Acquire);

            if head.is_null() {
                return None;
            }

            let next = unsafe { (*head).next };

            if self.head.compare_exchange_weak(
                head,
                next,
                Ordering::Release,
                Ordering::Acquire
            ).is_ok() {
                let node = unsafe { Box::from_raw(head) };
                return Some(node.data);
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.load(Ordering::Acquire).is_null()
    }
}

impl<T> Drop for LockFreeStack<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

// Atomic counter with relaxed memory ordering options
pub struct AtomicCounter {
    count: AtomicUsize,
    max_value: usize,
}

impl AtomicCounter {
    pub fn new(max: usize) -> Self {
        AtomicCounter {
            count: AtomicUsize::new(0),
            max_value: max,
        }
    }

    pub fn increment(&self) -> Result<usize, String> {
        let mut current = self.count.load(Ordering::Relaxed);

        loop {
            if current >= self.max_value {
                return Err("Counter at maximum".to_string());
            }

            match self.count.compare_exchange_weak(
                current,
                current + 1,
                Ordering::SeqCst,
                Ordering::Relaxed
            ) {
                Ok(val) => return Ok(val + 1),
                Err(actual) => current = actual,
            }
        }
    }

    pub fn decrement(&self) -> Result<usize, String> {
        let mut current = self.count.load(Ordering::Relaxed);

        loop {
            if current == 0 {
                return Err("Counter at minimum".to_string());
            }

            match self.count.compare_exchange_weak(
                current,
                current - 1,
                Ordering::SeqCst,
                Ordering::Relaxed
            ) {
                Ok(val) => return Ok(val - 1),
                Err(actual) => current = actual,
            }
        }
    }

    pub fn get(&self) -> usize {
        self.count.load(Ordering::SeqCst)
    }

    pub fn reset(&self) {
        self.count.store(0, Ordering::SeqCst);
    }
}

// SeqLock for high-performance reads
pub struct SeqLock<T: Copy> {
    sequence: AtomicUsize,
    data: UnsafeCell<T>,
}

unsafe impl<T: Copy + Send> Send for SeqLock<T> {}
unsafe impl<T: Copy + Send> Sync for SeqLock<T> {}

impl<T: Copy> SeqLock<T> {
    pub fn new(data: T) -> Self {
        SeqLock {
            sequence: AtomicUsize::new(0),
            data: UnsafeCell::new(data),
        }
    }

    pub fn write(&self, new_data: T) {
        let seq = self.sequence.fetch_add(1, Ordering::AcqRel);

        // Ensure seq is odd during write
        assert_eq!(seq % 2, 0);

        unsafe {
            ptr::write_volatile(self.data.get(), new_data);
        }

        self.sequence.fetch_add(1, Ordering::AcqRel);
    }

    pub fn read(&self) -> T {
        loop {
            let seq1 = self.sequence.load(Ordering::Acquire);

            // If odd, a write is in progress
            if seq1 % 2 != 0 {
                std::hint::spin_loop();
                continue;
            }

            let data = unsafe { ptr::read_volatile(self.data.get()) };

            let seq2 = self.sequence.load(Ordering::Acquire);

            // If sequence hasn't changed, read is valid
            if seq1 == seq2 {
                return data;
            }

            std::hint::spin_loop();
        }
    }
}

// Custom Arc implementation
pub struct UnsafeArc<T> {
    ptr: *mut ArcInner<T>,
    _phantom: PhantomData<T>,
}

struct ArcInner<T> {
    strong: AtomicUsize,
    weak: AtomicUsize,
    data: T,
}

unsafe impl<T: Send + Sync> Send for UnsafeArc<T> {}
unsafe impl<T: Send + Sync> Sync for UnsafeArc<T> {}

impl<T> UnsafeArc<T> {
    pub fn new(data: T) -> Self {
        let inner = Box::new(ArcInner {
            strong: AtomicUsize::new(1),
            weak: AtomicUsize::new(1),
            data,
        });

        UnsafeArc {
            ptr: Box::into_raw(inner),
            _phantom: PhantomData,
        }
    }

    pub fn strong_count(&self) -> usize {
        unsafe { (*self.ptr).strong.load(Ordering::SeqCst) }
    }

    pub fn get(&self) -> &T {
        unsafe { &(*self.ptr).data }
    }
}

impl<T> Clone for UnsafeArc<T> {
    fn clone(&self) -> Self {
        unsafe {
            (*self.ptr).strong.fetch_add(1, Ordering::Relaxed);
        }

        UnsafeArc {
            ptr: self.ptr,
            _phantom: PhantomData,
        }
    }
}

impl<T> Drop for UnsafeArc<T> {
    fn drop(&mut self) {
        unsafe {
            if (*self.ptr).strong.fetch_sub(1, Ordering::Release) == 1 {
                std::sync::atomic::fence(Ordering::Acquire);

                // Last strong reference, drop data
                ptr::drop_in_place(&mut (*self.ptr).data);

                if (*self.ptr).weak.fetch_sub(1, Ordering::Release) == 1 {
                    // Last weak reference, deallocate
                    let _ = Box::from_raw(self.ptr);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsafe_mutex() {
        let mutex = Arc::new(UnsafeMutex::new(0u32));
        let mut handles = vec![];

        for _ in 0..10 {
            let mutex_clone = mutex.clone();
            let handle = thread::spawn(move || {
                for _ in 0..1000 {
                    let mut guard = mutex_clone.lock();
                    *guard += 1;
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let final_value = *mutex.lock();
        assert_eq!(final_value, 10000);
    }

    #[test]
    fn test_lock_free_stack() {
        let stack = Arc::new(LockFreeStack::new());
        let mut handles = vec![];

        // Push from multiple threads
        for i in 0..10 {
            let stack_clone = stack.clone();
            let handle = thread::spawn(move || {
                for j in 0..100 {
                    stack_clone.push(i * 100 + j);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        // Pop all values
        let mut count = 0;
        while stack.pop().is_some() {
            count += 1;
        }

        assert_eq!(count, 1000);
    }

    #[test]
    fn test_atomic_counter() {
        let counter = Arc::new(AtomicCounter::new(1000));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter_clone = counter.clone();
            let handle = thread::spawn(move || {
                for _ in 0..50 {
                    let _ = counter_clone.increment();
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(counter.get(), 500);
    }

    #[test]
    fn test_seqlock() {
        let lock = Arc::new(SeqLock::new((0, 0)));
        let lock_read = lock.clone();

        let writer = thread::spawn(move || {
            for i in 0..100 {
                lock.write((i, i * 2));
                thread::sleep(std::time::Duration::from_micros(10));
            }
        });

        let reader = thread::spawn(move || {
            for _ in 0..100 {
                let (a, b) = lock_read.read();
                assert_eq!(b, a * 2);
                thread::sleep(std::time::Duration::from_micros(5));
            }
        });

        writer.join().unwrap();
        reader.join().unwrap();
    }
}

// Student exercises
pub mod exercises {
    use super::*;
    use std::sync::atomic::AtomicI32;

    // Exercise 1: Implement a lock-free queue
    pub struct LockFreeQueue<T> {
        // TODO: Implement a Michael-Scott queue
        // head: AtomicPtr<Node<T>>,
        // tail: AtomicPtr<Node<T>>,
    }

    impl<T> LockFreeQueue<T> {
        pub fn new() -> Self {
            // TODO: Initialize queue with dummy node
            unimplemented!("Initialize lock-free queue")
        }

        pub fn enqueue(&self, data: T) {
            // TODO: Add to tail
            unimplemented!("Enqueue operation")
        }

        pub fn dequeue(&self) -> Option<T> {
            // TODO: Remove from head
            unimplemented!("Dequeue operation")
        }
    }

    // Exercise 2: Implement a reader-writer lock
    pub struct RwLock<T> {
        readers: AtomicI32,
        writer: AtomicBool,
        data: UnsafeCell<T>,
    }

    impl<T> RwLock<T> {
        pub fn new(data: T) -> Self {
            // TODO: Initialize RwLock
            unimplemented!("Initialize RwLock")
        }

        pub fn read(&self) -> ReadGuard<T> {
            // TODO: Acquire read lock
            unimplemented!("Acquire read lock")
        }

        pub fn write(&self) -> WriteGuard<T> {
            // TODO: Acquire write lock
            unimplemented!("Acquire write lock")
        }
    }

    pub struct ReadGuard<'a, T> {
        lock: &'a RwLock<T>,
    }

    pub struct WriteGuard<'a, T> {
        lock: &'a RwLock<T>,
    }

    // Exercise 3: Implement hazard pointers
    pub struct HazardPointer<T> {
        // TODO: Implement hazard pointer for safe memory reclamation
    }

    impl<T> HazardPointer<T> {
        pub fn protect(&self, ptr: *mut T) {
            // TODO: Mark pointer as protected
            unimplemented!("Protect pointer")
        }

        pub fn release(&self) {
            // TODO: Release protection
            unimplemented!("Release protection")
        }

        pub fn retire(&self, ptr: *mut T) {
            // TODO: Retire pointer for later reclamation
            unimplemented!("Retire pointer")
        }
    }
}