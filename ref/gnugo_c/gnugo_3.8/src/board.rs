//! 直接转写gnugo的board.c，保留原接口
use libc::{c_int, c_void};

#[repr(C)]
pub struct Board {
    pub size: c_int,
    // ...其他字段与gnugo的board.h完全一致
}

#[no_mangle]
pub extern "C" fn rs_board_new(size: c_int) -> *mut Board {
    Box::into_raw(Box::new(Board {
        size,
        // ...初始化
    }))
}