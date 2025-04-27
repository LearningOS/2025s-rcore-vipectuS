//! Mutex (spin-like and blocking(sleep))

use core::any::Any;

use super::UPSafeCell;
use crate::task::{block_current_and_run_next, suspend_current_and_run_next};
use crate::task::TaskControlBlock;
use crate::task::{current_task, wakeup_task};
use alloc::vec::Vec;
use alloc::{collections::VecDeque, sync::Arc};

/// Mutex trait
pub trait Mutex: Sync + Send + Any {
    /// Lock the mutex
    fn lock(&self);
    /// Unlock the mutex
    fn unlock(&self);
    /// Downcast
    fn as_any(&self) -> &dyn Any;
}

/// Spinlock Mutex struct
pub struct MutexSpin {
    locked: UPSafeCell<bool>,
}

impl MutexSpin {
    /// Create a new spinlock mutex
    pub fn new() -> Self {
        Self {
            locked: unsafe { UPSafeCell::new(false) },
        }
    }
}

impl Mutex for MutexSpin {
    /// Lock the spinlock mutex
    fn lock(&self) {
        trace!("kernel: MutexSpin::lock");
        loop {
            let mut locked = self.locked.exclusive_access();
            if *locked {
                drop(locked);
                suspend_current_and_run_next();
                continue;
            } else {
                *locked = true;
                return;
            }
        }
    }

    fn unlock(&self) {
        trace!("kernel: MutexSpin::unlock");
        let mut locked = self.locked.exclusive_access();
        *locked = false;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Blocking Mutex struct
pub struct MutexBlocking {
    inner: UPSafeCell<MutexBlockingInner>,
}

pub struct MutexBlockingInner {
    locked: bool,
    onwer_tid: Option<usize>,
    wait_queue: VecDeque<Arc<TaskControlBlock>>,
}

impl MutexBlocking {
    /// Create a new blocking mutex
    pub fn new() -> Self {
        trace!("kernel: MutexBlocking::new");
        Self {
            inner: unsafe {
                UPSafeCell::new(MutexBlockingInner {
                    locked: false,
                    onwer_tid: None,
                    wait_queue: VecDeque::new(),
                })
            },
        }
    }
}

impl Mutex for MutexBlocking {
    /// lock the blocking mutex
    fn lock(&self) {
        trace!("kernel: MutexBlocking::lock");
        let mut mutex_inner = self.inner.exclusive_access();
        if mutex_inner.locked {
            mutex_inner.wait_queue.push_back(current_task().unwrap());
            drop(mutex_inner);
            block_current_and_run_next();
        } else {
            mutex_inner.locked = true;
            mutex_inner.onwer_tid = Some(current_task().unwrap().inner_exclusive_access().res.as_ref().unwrap().tid);
        }
    }

    /// unlock the blocking mutex
    fn unlock(&self) {
        trace!("kernel: MutexBlocking::unlock");
        let mut mutex_inner = self.inner.exclusive_access();
        assert!(mutex_inner.locked);
        if let Some(waking_task) = mutex_inner.wait_queue.pop_front() {
            mutex_inner.onwer_tid = Some(waking_task.inner_exclusive_access().res.as_ref().unwrap().tid);
            wakeup_task(waking_task);
        } else {
            mutex_inner.onwer_tid = None;
            mutex_inner.locked = false;
        }
    }

    /// Downcast
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl MutexBlocking {
    /// return if the mutex is locked
    pub fn is_locked(&self) -> bool {
        self.inner.exclusive_access().locked
    }

    /// return the owner of the mutex
    pub fn get_onwer(&self) -> Option<usize> {
        self.inner.exclusive_access().onwer_tid
    }

    /// return the waiter tid
    pub fn get_waiter(&self) -> Vec<usize> {
        let mut res = Vec::new();
        let inner = self.inner.exclusive_access();

        for task in &inner.wait_queue {
            res.push(task.inner_exclusive_access().res.as_ref().unwrap().tid);
        }

        res
    }
}
