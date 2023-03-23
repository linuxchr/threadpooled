use crossbeam_channel::{unbounded, Receiver, Sender};
use std::collections::HashMap;
use std::thread::{self, JoinHandle};
use thiserror::Error;

pub struct Threadpool {
    pub max_threads: u128,
    pub id: u8,
    used_threads: u128,
    thread_send: Sender<(u128, JoinHandle<()>)>,
    used_ids: Vec<u128>,
    done: Vec<u128>,
}

pub struct Thread<'a> {
    id: u128,
    done: bool,
    origin: &'a mut Threadpool,
}

#[derive(Error, Debug)]
pub enum ThreadingError {
    #[error("No threads available (max is {max_threads:?})")]
    NoAvaliableThreads { max_threads: u128 },
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

pub fn manager(pool: &Threadpool) {
    let (tx, rx) = crossbeam_channel::unbounded();
    let (bx, brx) = unbounded();
    thread::spawn(move || {
        let sender = bx;
        let receiver = rx;
        loop {
            Threadpool::manage(&sender, &receiver);
        }
    });
}

fn new(max_threads: u128, id: u128) -> Self {}

pub impl Threadpool {
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
    fn new(max_threads: u128, id: u8) -> Self {
        let (tx, rx) = crossbeam_channel::unbounded();
        let (bx, brx) = unbounded();
        thread::spawn(move || {
            let sender = bx;
            let receiver = rx;
            loop {
                manager(&sender, &receiver);
            }
        });
        let tp = Threadpool {
            max_threads,
            id,
            used_threads: 0,
            thread_send: tx,
            used_ids: Vec::new(),
            done: Vec::new(),
        };
        return tp;
    }

    pub fn assing<F: Fn() + Send + 'static>(&mut self, f: F) -> Result<Thread, ThreadingError> {
        if self.used_threads >= self.max_threads {
            return Err(ThreadingError::NoAvaliableThreads {
                max_threads: self.max_threads,
            });
        }
        let handle = thread::spawn(f);
        self.used_threads += 1;
        let mut id_try = self.used_threads;
        let id: u128;
        while self.used_ids.contains(&id_try) {
            id_try += 1;
        }
        id = id_try;
        if let Err(_) = self.thread_send.send((id, handle)) {
            self.used_threads -= 1;
            return Err(ThreadingError::ManagerCommunicationFailed);
        }
        let thread = Thread {
            id: id,
            done: false,
            origin: self,
        };
        return Ok(thread);
    }
}

impl Thread<'_> {
    pub fn is_finished(&mut self) -> Result<bool, ThreadingError> {
        if !self.done {
            return Ok(self.done);
        }
        Ok(self.done)
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
