use x86_64::{
    structures::paging::{mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB},
    VirtAddr,
};
use linked_list_allocator::LockedHeap;

// A wrapper around bootloader's frame allocator
pub struct BootInfoFrameAllocator<I>
where
    I: Iterator<Item = x86_64::structures::paging::PhysFrame>,
{
    frames: I,
}

impl<I> BootInfoFrameAllocator<I>
where
    I: Iterator<Item = x86_64::structures::paging::PhysFrame>,
{
    pub unsafe fn new(frames: I) -> Self {
        BootInfoFrameAllocator { frames }
    }
}

unsafe impl<I> FrameAllocator<Size4KiB> for BootInfoFrameAllocator<I>
where
    I: Iterator<Item = x86_64::structures::paging::PhysFrame>,
{
    fn allocate_frame(&mut self) -> Option<x86_64::structures::paging::PhysFrame> {
        self.frames.next()
    }
}

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        unsafe {
            mapper.map_to(page, frame, flags, frame_allocator)?.flush();
        }
    }

    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }

    Ok(())
}
