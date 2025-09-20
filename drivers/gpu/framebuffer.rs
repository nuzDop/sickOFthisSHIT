use bootloader::bootinfo::{FrameBuffer, FrameBufferInfo, PixelFormat};
use spin::{Mutex, Once};

/// A global static instance of our framebuffer writer.
pub static FRAMEBUFFER_WRITER: Once<Mutex<FrameBufferWriter>> = Once::new();

/// Provides a safe interface for writing to the framebuffer.
pub struct FrameBufferWriter {
    info: FrameBufferInfo,
    framebuffer: &'static mut [u8],
}

impl FrameBufferWriter {
    /// Creates a new writer for the given framebuffer.
    /// This function is unsafe because the caller must guarantee that the framebuffer
    /// memory is valid and correctly mapped.
    pub unsafe fn new(framebuffer: &'static mut FrameBuffer) -> Self {
        Self {
            info: framebuffer.info(),
            framebuffer: framebuffer.buffer_mut(),
        }
    }

    /// Writes a single pixel to the screen.
    pub fn write_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x >= self.info.width || y >= self.info.height {
            return; // Out of bounds
        }

        let pixel_offset = y * self.info.stride + x;
        let color_bytes = color.to_le_bytes();
        let byte_offset = pixel_offset * self.info.bytes_per_pixel;

        // Write the pixel based on its format.
        match self.info.pixel_format {
            PixelFormat::Rgb => {
                self.framebuffer[byte_offset] = color_bytes[2];
                self.framebuffer[byte_offset + 1] = color_bytes[1];
                self.framebuffer[byte_offset + 2] = color_bytes[0];
            }
            PixelFormat::Bgr => {
                self.framebuffer[byte_offset] = color_bytes[0];
                self.framebuffer[byte_offset + 1] = color_bytes[1];
                self.framebuffer[byte_offset + 2] = color_bytes[2];
            }
            PixelFormat::U8 => {
                self.framebuffer[byte_offset] = color_bytes[0];
            }
            _ => {} // Unknown format, do nothing.
        }
    }

    /// Clears the entire screen to a single color.
    pub fn clear(&mut self, color: u32) {
        for y in 0..self.info.height {
            for x in 0..self.info.width {
                self.write_pixel(x, y, color);
            }
        }
    }

    /// Draws a filled rectangle.
    pub fn fill_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        for dy in 0..height {
            for dx in 0..width {
                self.write_pixel(x + dx, y + dy, color);
            }
        }
    }
}

/// Initializes the global framebuffer writer.
pub fn init(framebuffer: &'static mut FrameBuffer) {
    let writer = unsafe { FrameBufferWriter::new(framebuffer) };
    FRAMEBUFFER_WRITER.call_once(|| Mutex::new(writer));
}
