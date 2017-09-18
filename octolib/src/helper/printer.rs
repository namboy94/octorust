/// Author: Hermann Krumrey <hermann@krumreyh.com> 2017
/// Karlsruher Institut für Technologie, Matriculation number 1789804

extern { fn printf(s: *const u8, ...); }

pub fn print(text: &str) {
    unsafe {
        printf(text.as_ptr());
    }
}

pub fn print_one<T>(text: &str, value: T) {
    unsafe {
        printf(text.as_ptr(), value);
    }
}

pub fn print_two<T1, T2>(text: &str, value1: T1, value2: T2) {
    unsafe {
        printf(text.as_ptr(), value1, value2);
    }
}