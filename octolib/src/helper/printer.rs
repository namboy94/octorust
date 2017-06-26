extern {
    fn printf(s: *const u8, ...);
}

pub fn newline() {
    unsafe {
        printf("\n\0".as_ptr())
    }
}

pub fn print_text(text: &str) {
    unsafe {
        printf("%s\0".as_ptr(), text);
    }
}

pub fn print_u32(number: u32) {
    unsafe {
        printf("%d\0".as_ptr(), number)
    }
}
