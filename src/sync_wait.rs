#![allow(unused)]

use std::thread::sleep;
use std::time::Duration;
use crate::COUNT;

pub struct SyncWait {
    pub(crate) count: i32,
}

impl SyncWait {
    // pub fn add(&self, &mut count: i32) {
    //     &self.count += *count
    // }
    pub fn done(&mut self) {
        println!("before done: {}", self.count);
        if self.count > 0 {
            self.count -= 1;
        } else {
            panic!("done error!")
        }
        println!("after done: {}", self.count)
    }
    pub unsafe fn wait(&mut self) {
        loop {
            // sleep(Duration::from_secs(1));
            if self.count == 0 {
                break println!("wait over, continue!");
            } else {
                // println!("waitting……current:{}",self.count)
            }
        }
    }
}
