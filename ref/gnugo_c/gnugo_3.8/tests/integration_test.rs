#[test]
fn test_board_ffi() {
    unsafe {
        let board = rs_board_new(19);
        assert_eq!((*board).size, 19);
        libc::free(board as *mut libc::c_void);
    }
}