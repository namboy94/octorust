extern {
    #[link_name="printf"]
    fn __printf(s: *const u8, ...);
}

pub fn newline() {
    unsafe {
        __printf("\n\0".as_ptr())
    }
}

pub fn print_text(text: &str) {
    unsafe {
        __printf("%s\0".as_ptr(), text)
    }
}

pub fn print_u32(number: u32) {
    unsafe {
        __printf("%d\0".as_ptr(), number)
    }
}
