//! Implementation of [`TaskManager`]
//!
//! It is only used to manage processes and schedule process based on ready queue.
//! Other CPU process monitoring functions are in Processor.

use super::{TaskControlBlock, schedule, Schedule};
use crate::sync::UPSafeCell;
use alloc::collections::{BTreeMap, VecDeque};
use alloc::sync::Arc;
use lazy_static::*;

pub struct TaskManager {
    stride_mark: usize, //= BTreeMap<pid, bool>
    ready_queue: VecDeque<Arc<TaskControlBlock>>,
}

// YOUR JOB: FIFO->Stride
/// A simple FIFO scheduler.
impl TaskManager {
    /// 返回一个空的TaskManager
    pub fn new() -> Self {
        Self {
            stride_mark: 0,
            ready_queue: VecDeque::new(),
        }
    }
    /// Add process back to ready queue
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push_back(task);
    }
    /// Take a process out of the ready queue
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.ready_queue.pop_front()
    }

    /// Take a process out of the ready queue
    pub fn stride_scheduling(&mut self) -> Option<Arc<TaskControlBlock>> {
        if self.ready_queue.len() == 0 {
            return None;
        }
        let result_id = (0..self.ready_queue.len())
            .min_by_key(|id| self.ready_queue[*id].inner_exclusive_access().schedule.pass);

        // if self.ready_queue[result_id.unwrap()].inner_exclusive_access().pass == usize::MAX {
        //     self.ready_queue.iter_mut().for_each(|item| item.inner_exclusive_access().pass = 0);
        // }

        let result = self.ready_queue.remove(result_id.unwrap());
        // let inner = &mut result.as_mut().unwrap().inner_exclusive_access().schedule;
        // inner.update_pass();
        // let stride = result.as_mut().unwrap().inner_exclusive_access();
        // let stride = &mut (stride.stride);
        // let inner =  result.as_mut().unwrap().inner_exclusive_access();
        // let pass = &mut inner.pass;
        // *pass += stride;
        // drop(inner);
        result
    }
    fn update_pass(&mut self) {

    }
}

lazy_static! {
    /// TASK_MANAGER instance through lazy_static!
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> =
        unsafe { UPSafeCell::new(TaskManager::new()) };
}

/// 与fetch_task相反，它将一个task重新放到TaskManager中
pub fn add_task(task: Arc<TaskControlBlock>) {
    TASK_MANAGER.exclusive_access().add(task);
}

/// 从TaskManager中pop出一个task
pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    TASK_MANAGER.exclusive_access().fetch()
}

/// 根据stride scheduling从TaskManager中pop出一个task
pub fn stride_scheduling_task() -> Option<Arc<TaskControlBlock>> {
    TASK_MANAGER.exclusive_access().stride_scheduling()
}
