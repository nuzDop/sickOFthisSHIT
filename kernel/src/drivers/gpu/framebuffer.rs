use bootloader_api::info::{FrameBuffer, FrameBufferInfo, PixelFormat};
use spin::{Mutex, Once};

pub static FRAMEBUFFER_WRITER: Once<Mutex<FrameBufferWriter>> = Once::new();

pub struct FrameBufferWriter {
    info: FrameBufferInfo,
    framebuffer: &'static mut [u8],
}

impl FrameBufferWriter {
    pub unsafe fn new(framebuffer: &'static mut FrameBuffer) -> Self {
        Self {
            info: framebuffer.info(),
            framebuffer: framebuffer.buffer_mut(),
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x >= self.info.width || y >= self.info.height {
            return;
        }
        let pixel_offset = y * self.info.stride + x;
        let color_bytes = color.to_le_bytes();
        let byte_offset = pixel_offset * self.info.bytes_per_pixel;

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
            _ => {}
        }
    }

    pub fn clear(&mut self, color: u32) {
        let width = self.info.width;
        let height = self.info.height;
        for y in 0..height {
            for x in 0..width {
                self.write_pixel(x, y, color);
            }
        }
    }

    pub fn fill_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        for dy in 0..height {
            for dx in 0..width {
                self.write_pixel(x + dx, y + dy, color);
            }
        }
    }
}

pub fn init(framebuffer: &'static mut FrameBuffer) {
    let writer = unsafe { FrameBufferWriter::new(framebuffer) };
    FRAMEBUFFER_WRITER.call_once(|| Mutex::new(writer));
}
