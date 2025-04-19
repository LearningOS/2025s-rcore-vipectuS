//! Process management syscalls
use core::mem::size_of;

use crate::task::try_allocate_phy_mm;
use crate::task::try_deallocate_phy_mm;
use crate::{
    mm::{PageTable, VirtAddr},
    task::{
        change_program_brk, current_user_token, exit_current_and_run_next, find_syscall_count,
        suspend_current_and_run_next,
    },
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let size = size_of::<usize>();
    let sec_ptr = _ts as usize;
    let usec_ptr = sec_ptr + size;
    let us = get_time_us();
    let page_table = PageTable::from_token(current_user_token());

    let sec_va = VirtAddr::from(sec_ptr);
    let offset = sec_va.page_offset();
    let sec_ppn = page_table.translate(sec_va.floor()).unwrap().ppn();
    let bytes = sec_ppn.get_bytes_array();
    bytes[offset..offset + size].copy_from_slice(&(us / 1_000_000).to_ne_bytes());

    let usec_va = VirtAddr::from(usec_ptr);
    let offset = usec_va.page_offset();
    let usec_ppn = page_table.translate(usec_va.floor()).unwrap().ppn();
    let bytes = usec_ppn.get_bytes_array();
    bytes[offset..offset + size].copy_from_slice(&(us % 1_000_000).to_ne_bytes());
    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {
    trace!("kernel: sys_trace");
    match _trace_request {
        0 => {
            let page_table = PageTable::from_token(current_user_token());
            let va = VirtAddr::from(_id);
            let pte = match page_table.translate(va.floor()) {
                Some(pte) => pte,
                None => return -1,
            };

            if pte.readable() && pte.user_accessible() {
                let bytes = pte.ppn().get_bytes_array();
                bytes[va.page_offset()].into()
            } else {
                -1
            }
        }
        1 => {
            let page_table = PageTable::from_token(current_user_token());
            let va = VirtAddr::from(_id);
            let pte = match page_table.translate(va.floor()) {
                Some(pte) => pte,
                None => return -1,
            };

            if pte.writable() && pte.user_accessible() {
                let bytes = pte.ppn().get_bytes_array();
                bytes[va.page_offset()] = _data as u8;
                0
            } else {
                -1
            }
        }
        2 => find_syscall_count(_id).into(),
        _ => -1,
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap");
    match try_allocate_phy_mm(_start, _len, _port) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap");
    match try_deallocate_phy_mm(_start, _len) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
