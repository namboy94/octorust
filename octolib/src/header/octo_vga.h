#ifndef _OCTO_VGA_H_
#define _OCTO_VGA_H_

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \file octo_vga.h
 *
 * \brief VGA output on guest layer
 */

/**
 * \brief Set the color of a pixel in the virtual VGA framebuffer
 * Sets the color of a pixel at coordinate (\b x,\b y) to the RGB values
 * provided
 */
void vga_set_pixel(int x, int y, int r, int g, int b);

/** 
 * \brief Update the screen to show previous changes
 */
void vga_update(void);

/**
 * \brief Do a bitblit into the buffer of the virtual VGA device
 */
void vga_write_buffer(uint32_t* buffer);

uint32_t* vga_get_framebuffer(void);

#ifdef __cplusplus
}
#endif

#endif
