use std::collections::HashMap;
use std::thread::{self, JoinHandle};
use thiserror::Error;

pub type ThreadId = usize;

#[derive(Debug)]

pub struct Threadpool {
    max_threads: usize,
    thread_handles: HashMap<ThreadId, (JoinHandle<()>, i32)>,
    used_ids: Vec<usize>,
    used_threads: usize,
}

#[derive(Debug, Copy, Clone, Error)]
pub enum ThreadingError {
    #[error("No threads available (max is {max_threads:?})")]
    NoAvaliableThreads { max_threads: usize },
    #[error("Manager communication failed")]
    ManagerCommunicationFailed,
    #[error("Used thread check failed")]
    UsedThreadCheckFailed,
    #[error("Thread freeing failed")]
    ThreadFreeingFailed,
    #[error("Thread state broadcasting failed")]
    ThreadStateBroadcastingFailed,
    #[error("Thread away giving failed")]
    ThreadAwayFailed,
    #[error("Thread might attempt to join itself or created a deadlock")]
    ThreadJoinFailed,
    #[error("Thread dont exist")]
    ThreadNotFound,
}

impl Threadpool {
    pub fn new(max_threads: usize) -> Self {
        Threadpool {
            max_threads,
            thread_handles: HashMap::new(),
            used_ids: Vec::new(),
            used_threads: 0,
        }
    }

    pub fn assing<F: Fn() + Send + 'static>(&mut self, f: F) -> Result<ThreadId, ThreadingError> {
        for (_, (handle, used)) in self.thread_handles.iter_mut() {
            if used == &0 && handle.is_finished() {
                *used = 1;
                self.used_threads -= 1;
            }
        }
        if self.used_threads >= self.max_threads {
            return Err(ThreadingError::NoAvaliableThreads {
                max_threads: self.max_threads,
            });
        }
        let handle = thread::spawn(f);
        self.used_threads += 1;
        let mut id_try: ThreadId = self.used_threads;
        while self.used_ids.contains(&id_try) {
            id_try += 1;
        }
        let id: ThreadId = id_try;
        self.thread_handles.insert(id, (handle, 0));
        Ok(id)
    }
    pub fn is_finished(&mut self, th: ThreadId) -> Result<bool, ThreadingError> {
        if let Some((handle, used)) = self.thread_handles.get_mut(&th) {
            if handle.is_finished() {
                if used == &0 {
                    *used = 1;
                    self.used_threads -= 1;
                }
                return Ok(true);
            }
        } else {
            return Err(ThreadingError::ThreadNotFound);
        }
        Ok(false)
    }
    /// !!! Consumes the Input Thread, dont use it after that !!!
    pub fn join(&mut self, th: ThreadId) -> Result<(), ThreadingError> {
        match self.thread_handles.remove(&th) {
            None => Err(ThreadingError::ThreadNotFound),
            Some((handle, _)) => {
                if handle.join().is_err() {
                    return Err(ThreadingError::ThreadJoinFailed);
                }
                self.used_threads -= 1;
                Ok(())
            }
        }
    }
}
