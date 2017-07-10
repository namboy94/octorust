extern {
    #[link_name="cas"]
    fn __cas(data: usize, oldVal: usize, newVal: usize) -> i32;
}

fn cas(data: usize, oldVal: usize, newVal: usize) -> i32 {
    unsafe {
        __cas(data, oldVal, newVal)
    }
}