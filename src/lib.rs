use crossbeam_channel::{Receiver, Sender};
use std::collections::HashMap;
use std::thread::{self, JoinHandle};
use thiserror::Error;

pub type ThreadId = usize;

#[derive(Debug)]

pub struct Threadpool {
    pub max_threads: usize,
    pub id: u8,
    used_threads: Vec<Thread>,
    thread_send: Vec<(ThreadId, JoinHandle<()>)>,
    used_ids: Vec<u128>,
    done: Vec<ThreadId>,
}

#[derive(Debug)]
pub struct Thread {
    id: ThreadId,
    done: bool,
    origin: ThreadId,
}

#[derive(Error, Debug)]
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
}

fn manage(bx: &Sender<u128>, rx: &Receiver<(u128, JoinHandle<()>)>) {
    let mut threads: HashMap<u128, JoinHandle<()>> = HashMap::new();
    if let Ok(t) = rx.try_recv() {
        let (id, handle) = t;
        threads.insert(id, handle);
    }
    threads.retain(|id, handle| {
        if handle.is_finished() {
            if let Ok(_) = bx.send(*id) {
                return false;
            }
            return true;
        }
        return true;
    });
}

pub fn new(max_threads: usize, id: u8) -> Threadpool {
    let mut pool = Threadpool::new(max_threads, id);
    thread::spawn(|| {
        pool.manage();
    });
    return pool;
}

impl Threadpool {
    fn manage(&mut self) {
        let mut threads: HashMap<usize, &JoinHandle<()>> = HashMap::new();
        for (id, handle) in self.thread_send.iter() {
            threads.insert(*id, handle);
        }
        threads.retain(|id, handle| {
            if handle.is_finished() {
                self.done.push(*id);
                return true;
            }
            return true;
        });
    }
    pub fn new(max_threads: usize, id: u8) -> Self {
        let tp = Threadpool {
            max_threads,
            id,
            used_threads: Vec::new(),
            thread_send: Vec::new(),
            used_ids: Vec::new(),
            done: Vec::new(),
        };
        return tp;
    }

    pub fn assing<F: Fn() + Send + 'static>(&mut self, f: F) -> Result<Thread, ThreadingError> {
        if self.used_threads.len() >= self.max_threads {
            return Err(ThreadingError::NoAvaliableThreads {
                max_threads: self.max_threads,
            });
        }
        let handle = thread::spawn(f);
        self.used_threads += 1;
        let mut id_try = self.used_threads.len();
        let id: usize;
        while self.used_ids.contains(&id_try) {
            id_try += 1;
        }
        id = id_try;
        self.thread_send.push((id, handle));
        let thread = Thread {
            id: id,
            done: false,
            origin: self.id,
        };
        return Ok(thread);
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}*/
