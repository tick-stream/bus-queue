extern crate arc_swap;

use arc_swap::ArcSwapOption;
use std::fmt::Display;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
pub struct BusReader<T: Display> {
    buffer: Arc<Box<[ArcSwapOption<T>]>>,
    wi: Arc<AtomicUsize>,
    ri: usize,
    size: usize,
}

impl<T: Display> BusReader<T> {
    pub fn recv(&mut self) -> Option<Arc<T>> {
        if self.ri == self.wi.load(Ordering::Relaxed) {
            return None;
        }
        loop {
            match self.buffer[self.ri % self.size].load() {
                None => return None,
                Some(some) => {
                    if self.wi.load(Ordering::Relaxed) > self.ri + self.size {
                        self.ri = self.wi.load(Ordering::Relaxed) - self.size;
                    } else {
                        self.ri += 1;
                        return Some(some);
                    }
                }
            }
        }
    }
}

pub struct Bus<T: Display> {
    // atp to an array of atps of option<arc<t>>
    buffer: Arc<Box<[ArcSwapOption<T>]>>,
    wi: Arc<AtomicUsize>,
    size: usize,
}

impl<T: Display> Bus<T> {
    pub fn new(size: usize) -> Self {
        let mut temp: Vec<ArcSwapOption<T>> = Vec::new();
        temp.resize(size, ArcSwapOption::new(None));

        Self {
            buffer: Arc::new(temp.into_boxed_slice()),
            wi: Arc::new(AtomicUsize::new(0)),
            size: size,
        }
    }
    pub fn add_sub(&self) -> BusReader<T> {
        BusReader {
            buffer: self.buffer.clone(),
            wi: self.wi.clone(),
            ri: 0,
            size: self.size,
        }
    }
    pub fn push(&self, object: T) {
        self.buffer[self.wi.load(Ordering::Relaxed) % self.size].store(Some(Arc::new(object)));
        self.wi.fetch_add(1, Ordering::Relaxed);
    }
    pub fn print(&self) {
        let temp = &self.buffer;
        println!("******print********{}", temp.len());
        for (index, object) in temp.into_iter().enumerate() {
            match object.load() {
                None => println!("{} : None", index),
                Some(some) => println!("{} : Some({})", index, some),
            }
        }
        println!("******print********");
    }
}