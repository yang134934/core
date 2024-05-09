//! Process management syscalls

use crate::task::{change_program_brk, exit_current_and_run_next, suspend_current_and_run_next};
use crate::timer::get_time_ms;
use crate::mm::{mmap, munmap};



/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

/// get current time
pub fn sys_get_time() -> isize {
    get_time_ms() as isize
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
pub fn sys_mmap(start: usize, len: usize, prot: usize) -> isize
{
mmap(start,len,prot);
0
}
pub fn sys_munmap(start: usize, len: usize) -> isize
{
munmap(start,len);
0
}
