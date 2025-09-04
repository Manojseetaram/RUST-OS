use alloc::collections::linked_list::LinkedList;

struct ListNode {
    size: usize,
    next: Option<&'static mut ListNode>,
}
pub struct LinkedListAllocator{
    head : ListNode,
}
impl ListNode {
    const fn new(size: usize) -> Self {
        ListNode { size, next: None }
    }

    fn start_addr(&self) -> usize {
        self as *const Self as usize
    }

    fn end_addr(&self) -> usize {
        self.start_addr() + self.size
    }
}
impl LinkedListAllocator {
     pub const fn new() -> Self {
        Self {
            head: ListNode::new(0),
        }
    }

     pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        unsafe {
            self.add_free_region(heap_start, heap_size);
        }
    }

    /// Adds the given memory region to the front of the list.
    unsafe fn add_free_region(&mut self, addr: usize, size: usize) {
        todo!();
    }
}