#![feature(thread_local)]

#[thread_local]
static mut VAR: u64 = 0;

pub fn get_var() -> u64 {
    unsafe { VAR }
}

pub fn set_var(val: u64) {
    unsafe {
        VAR = val;
    }
}
