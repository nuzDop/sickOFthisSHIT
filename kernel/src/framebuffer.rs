use bootloader_api::info::{FrameBuffer, FrameBufferInfo, PixelFormat};
use spin::{Mutex, Once};

/// A global static instance of our framebuffer writer, initialized once.
pub static WRITER: Once<Mutex<FrameBufferWriter>> = Once::new();

/// Provides a safe interface for writing to the framebuffer memory.
pub struct FrameBufferWriter {
    framebuffer: &'static mut [u8],
    info: FrameBufferInfo,
}

impl FrameBufferWriter {
    /// Creates a new writer for the given framebuffer.
    pub fn new(framebuffer: &'static mut FrameBuffer) -> Self {
        FrameBufferWriter {
            info: framebuffer.info(),
            framebuffer: framebuffer.buffer_mut(),
        }
    }

    /// Draws a filled rectangle.
    pub fn fill_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        let color_bytes = color.to_le_bytes();
        for row in y..(y + height) {
            for col in x..(x + width) {
                if col < self.info.width && row < self.info.height {
                    let pixel_offset = row * self.info.stride + col;
                    let byte_offset = pixel_offset * self.info.bytes_per_pixel;
                    
                    self.framebuffer[byte_offset..byte_offset + 3]
                        .copy_from_slice(&color_bytes[0..3]);
                }
            }
        }
    }

    /// Clears the entire screen to a single color.
    pub fn clear(&mut self, color: u32) {
        self.fill_rect(0, 0, self.info.width, self.info.height, color);
    }
}
