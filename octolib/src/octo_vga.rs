/// VGA output on guest layer

extern {
    #[link_name="vga_set_pixel"]
    fn __vga_set_pixel(x: i32, y: i32, r: i32, g: i32, b: i32);

    #[link_name="vga_update"]
    fn __vga_update();

    #[link_name="vga_write_buffer"]
    fn __vga_write_buffer(buffer: *mut u32);

    #[link_name="vga_get_framebuffer"]
    fn __vga_get_framebuffer() -> *mut u32;

}

/// Set the color of a pixel in the virtual VGA framebuffer
/// Sets the color of a pixel at coordinate (\b x,\b y) to the RGB values
/// provided
pub fn vga_set_pixel(x: i32, y: i32, r: i32, g: i32, b: i32) {
    unsafe {
        __vga_set_pixel(x, y, r, g, b)
    }
}

/// Update the screen to show previous changes
pub fn vga_update() {
    unsafe {
        __vga_update()
    }
}

/// Do a bitblit into the buffer of the virtual VGA device
pub fn vga_write_buffer(buffer: *mut u32) {
    unsafe {
        __vga_write_buffer(buffer)
    }
}

pub fn vga_get_framebuffer() -> *mut u32 {
    unsafe {
        __vga_get_framebuffer()
    }
}
