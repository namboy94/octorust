/// Author: Hermann Krumrey <hermann@krumreyh.com> 2017
/// Karlsruher Institut für Technologie, Matriculation number 1789804
/// This fle is based on the IRTSS octo_cas.h file

extern {
    pub fn cas(data: usize, oldVal: usize, newVal: usize) -> i32;
}
