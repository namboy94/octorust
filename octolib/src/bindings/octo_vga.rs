/// Author: Hermann Krumrey <hermann@krumreyh.com> 2017
/// Karlsruher Institut für Technologie, Matriculation number 1789804
/// This fle is based on the IRTSS octo_vga.h file
///
/// VGA output on guest layer

extern {

    /// Set the color of a pixel in the virtual VGA framebuffer
    /// Sets the color of a pixel at coordinate (\b x,\b y) to the RGB values
    /// provided
    pub fn vga_set_pixel(x: i32, y: i32, r: i32, g: i32, b: i32);

    /// Update the screen to show previous changes
    pub fn vga_update();

    /// Do a bitblit into the buffer of the virtual VGA device
    pub fn vga_write_buffer(buffer: *mut u32);

    pub fn vga_get_framebuffer() -> *mut u32;

}
