/// Author: Hermann Krumrey <hermann@krumreyh.com> 2017
/// Karlsruher Institut für Technologie, Matriculation number 1789804
/// This fle is based on the IRTSS octo_cas.h file

extern {
    #[link_name="cas"]
    fn __cas(data: usize, oldVal: usize, newVal: usize) -> i32;
}

fn cas(data: usize, oldVal: usize, newVal: usize) -> i32 {
    unsafe {
        __cas(data, oldVal, newVal)
    }
}