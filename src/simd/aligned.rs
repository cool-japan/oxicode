//! Aligned buffer management for SIMD operations.
//!
//! This module provides memory-aligned buffers that are optimal for SIMD operations.
//! Proper alignment can significantly improve SIMD performance by enabling aligned loads/stores.

#[cfg(feature = "alloc")]
use core::alloc::Layout;
use core::mem::MaybeUninit;
#[cfg(feature = "alloc")]
use core::mem::{align_of, size_of};
use core::ops::{Deref, DerefMut};
#[cfg(feature = "alloc")]
use core::ptr::NonNull;
use core::slice;

/// The alignment used for SIMD operations (64 bytes for AVX-512 compatibility).
pub const SIMD_ALIGNMENT: usize = 64;

/// A buffer with guaranteed SIMD-friendly alignment.
///
/// This buffer is aligned to 64 bytes (AVX-512 requirements), which is also
/// compatible with AVX2 (32 bytes), SSE (16 bytes), and NEON (16 bytes).
///
/// # Example
///
/// ```rust,ignore
/// use oxicode::simd::AlignedVec;
///
/// let mut buffer = AlignedVec::<f32>::with_capacity(1024);
/// buffer.push(1.0);
/// buffer.push(2.0);
/// assert_eq!(buffer.len(), 2);
/// ```
#[cfg(feature = "alloc")]
pub struct AlignedVec<T> {
    ptr: NonNull<T>,
    len: usize,
    cap: usize,
}

#[cfg(feature = "alloc")]
impl<T> AlignedVec<T> {
    /// Create a new empty aligned vector.
    pub fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            len: 0,
            cap: 0,
        }
    }

    /// Create a new aligned vector with the specified capacity.
    ///
    /// # Panics
    ///
    /// Panics if allocation fails (via alloc::alloc::handle_alloc_error).
    pub fn with_capacity(capacity: usize) -> Self {
        if capacity == 0 {
            return Self::new();
        }

        let layout = Self::layout_for_capacity(capacity);
        // SAFETY: Layout is valid since capacity > 0 and T is sized
        let ptr = unsafe { alloc::alloc::alloc(layout) as *mut T };

        match NonNull::new(ptr) {
            Some(ptr) => Self {
                ptr,
                len: 0,
                cap: capacity,
            },
            None => alloc::alloc::handle_alloc_error(layout),
        }
    }

    /// Create an aligned vector from a slice, copying the data.
    pub fn from_slice(slice: &[T]) -> Self
    where
        T: Clone,
    {
        let mut vec = Self::with_capacity(slice.len());
        for item in slice {
            vec.push(item.clone());
        }
        vec
    }

    fn layout_for_capacity(capacity: usize) -> Layout {
        let size = capacity.saturating_mul(size_of::<T>());
        let align = SIMD_ALIGNMENT.max(align_of::<T>());
        // `align` is always a valid power of two (SIMD_ALIGNMENT is 64 and
        // align_of::<T>() is a power of two), so `from_size_align` only fails
        // when the alignment-rounded `size` exceeds isize::MAX. That is an
        // unsatisfiable allocation request, so route it through the standard
        // allocation-failure handler (which aborts) rather than panicking with
        // `expect`. This keeps the type free of production `unwrap`/`expect`.
        match Layout::from_size_align(size, align) {
            Ok(layout) => layout,
            Err(_) => {
                let reported = Layout::from_size_align(0, align).unwrap_or(Layout::new::<u8>());
                alloc::alloc::handle_alloc_error(reported)
            }
        }
    }

    /// Returns the number of elements in the vector.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the vector is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the capacity of the vector.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.cap
    }

    /// Push an element to the vector.
    ///
    /// # Panics
    ///
    /// Panics if reallocation fails.
    pub fn push(&mut self, value: T) {
        if self.len >= self.cap {
            self.grow();
        }

        // SAFETY: We just ensured len < cap
        unsafe {
            self.ptr.as_ptr().add(self.len).write(value);
        }
        self.len += 1;
    }

    /// Extend the vector with elements from an iterator.
    pub fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push(item);
        }
    }

    /// Clear all elements from the vector.
    pub fn clear(&mut self) {
        // Drop all elements
        for i in 0..self.len {
            // SAFETY: All elements from 0..len are valid
            unsafe {
                core::ptr::drop_in_place(self.ptr.as_ptr().add(i));
            }
        }
        self.len = 0;
    }

    /// Resize the vector to the given length, filling with the given value.
    pub fn resize(&mut self, new_len: usize, value: T)
    where
        T: Clone,
    {
        if new_len > self.len {
            self.reserve(new_len - self.len);
            for _ in self.len..new_len {
                self.push(value.clone());
            }
        } else {
            while self.len > new_len {
                self.pop();
            }
        }
    }

    /// Pop an element from the end of the vector.
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            // SAFETY: len was > 0, so this element is valid
            Some(unsafe { self.ptr.as_ptr().add(self.len).read() })
        }
    }

    /// Reserve additional capacity.
    pub fn reserve(&mut self, additional: usize) {
        let required = self.len.saturating_add(additional);
        if required > self.cap {
            let new_cap = required.max(self.cap.saturating_mul(2)).max(8);
            self.realloc(new_cap);
        }
    }

    /// Get the underlying slice.
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        if self.cap == 0 {
            &[]
        } else {
            // SAFETY: ptr is valid for len elements
            unsafe { slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
        }
    }

    /// Get the underlying mutable slice.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        if self.cap == 0 {
            &mut []
        } else {
            // SAFETY: ptr is valid for len elements
            unsafe { slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
        }
    }

    /// Get a raw pointer to the data.
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self.ptr.as_ptr()
    }

    /// Get a mutable raw pointer to the data.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.ptr.as_ptr()
    }

    /// Check if the buffer is properly aligned for SIMD operations.
    #[inline]
    pub fn is_aligned(&self) -> bool {
        if self.cap == 0 {
            true // Empty buffer is "aligned" vacuously
        } else {
            (self.ptr.as_ptr() as usize) % SIMD_ALIGNMENT == 0
        }
    }

    fn grow(&mut self) {
        let new_cap = if self.cap == 0 {
            8
        } else {
            self.cap.saturating_mul(2)
        };
        self.realloc(new_cap);
    }

    fn realloc(&mut self, new_cap: usize) {
        let new_layout = Self::layout_for_capacity(new_cap);

        let new_ptr = if self.cap == 0 {
            // First allocation
            unsafe { alloc::alloc::alloc(new_layout) as *mut T }
        } else {
            // Reallocate
            let old_layout = Self::layout_for_capacity(self.cap);
            unsafe {
                alloc::alloc::realloc(self.ptr.as_ptr() as *mut u8, old_layout, new_layout.size())
                    as *mut T
            }
        };

        match NonNull::new(new_ptr) {
            Some(ptr) => {
                self.ptr = ptr;
                self.cap = new_cap;
            }
            None => alloc::alloc::handle_alloc_error(new_layout),
        }
    }
}

#[cfg(feature = "alloc")]
impl<T> Drop for AlignedVec<T> {
    fn drop(&mut self) {
        if self.cap > 0 {
            // Drop all elements
            for i in 0..self.len {
                // SAFETY: All elements from 0..len are valid
                unsafe {
                    core::ptr::drop_in_place(self.ptr.as_ptr().add(i));
                }
            }

            // Deallocate
            let layout = Self::layout_for_capacity(self.cap);
            unsafe {
                alloc::alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}

#[cfg(feature = "alloc")]
impl<T> Default for AlignedVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "alloc")]
impl<T> Deref for AlignedVec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

#[cfg(feature = "alloc")]
impl<T> DerefMut for AlignedVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

#[cfg(feature = "alloc")]
impl<T: Clone> Clone for AlignedVec<T> {
    fn clone(&self) -> Self {
        Self::from_slice(self.as_slice())
    }
}

#[cfg(feature = "alloc")]
impl<T: core::fmt::Debug> core::fmt::Debug for AlignedVec<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(self.as_slice().iter()).finish()
    }
}

#[cfg(feature = "alloc")]
// SAFETY: AlignedVec is Send if T is Send
unsafe impl<T: Send> Send for AlignedVec<T> {}

#[cfg(feature = "alloc")]
// SAFETY: AlignedVec is Sync if T is Sync
unsafe impl<T: Sync> Sync for AlignedVec<T> {}

/// A fixed-size aligned buffer on the stack.
///
/// This is useful for small temporary buffers that need SIMD alignment.
#[repr(C, align(64))]
pub struct AlignedBuffer<T, const N: usize> {
    data: [MaybeUninit<T>; N],
    len: usize,
}

impl<T, const N: usize> AlignedBuffer<T, N> {
    /// Create a new empty aligned buffer.
    pub const fn new() -> Self {
        Self {
            // SAFETY: MaybeUninit array doesn't require initialization
            data: unsafe { MaybeUninit::uninit().assume_init() },
            len: 0,
        }
    }

    /// Returns the capacity of the buffer.
    #[inline]
    pub const fn capacity(&self) -> usize {
        N
    }

    /// Returns the number of elements in the buffer.
    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the buffer is empty.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns true if the buffer is full.
    #[inline]
    pub const fn is_full(&self) -> bool {
        self.len >= N
    }

    /// Push an element to the buffer.
    ///
    /// Returns `Err(value)` if the buffer is full.
    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.len >= N {
            return Err(value);
        }
        self.data[self.len] = MaybeUninit::new(value);
        self.len += 1;
        Ok(())
    }

    /// Pop an element from the buffer.
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            // SAFETY: len was > 0, element at len is initialized
            Some(unsafe { self.data[self.len].assume_init_read() })
        }
    }

    /// Clear all elements from the buffer.
    pub fn clear(&mut self) {
        for i in 0..self.len {
            // SAFETY: Elements from 0..len are initialized
            unsafe {
                self.data[i].assume_init_drop();
            }
        }
        self.len = 0;
    }

    /// Get the buffer as a slice.
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        // SAFETY: First len elements are initialized
        unsafe { slice::from_raw_parts(self.data.as_ptr() as *const T, self.len) }
    }

    /// Get the buffer as a mutable slice.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        // SAFETY: First len elements are initialized
        unsafe { slice::from_raw_parts_mut(self.data.as_mut_ptr() as *mut T, self.len) }
    }

    /// Get the raw pointer to the buffer.
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self.data.as_ptr() as *const T
    }

    /// Get the raw mutable pointer to the buffer.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.data.as_mut_ptr() as *mut T
    }
}

impl<T, const N: usize> Default for AlignedBuffer<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> Drop for AlignedBuffer<T, N> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl<T, const N: usize> Deref for AlignedBuffer<T, N> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T, const N: usize> DerefMut for AlignedBuffer<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

impl<T: Clone, const N: usize> Clone for AlignedBuffer<T, N> {
    fn clone(&self) -> Self {
        let mut result = Self::new();
        for item in self.as_slice() {
            let _ = result.push(item.clone());
        }
        result
    }
}

impl<T: core::fmt::Debug, const N: usize> core::fmt::Debug for AlignedBuffer<T, N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(self.as_slice().iter()).finish()
    }
}

// SAFETY: AlignedBuffer is Send if T is Send
unsafe impl<T: Send, const N: usize> Send for AlignedBuffer<T, N> {}

// SAFETY: AlignedBuffer is Sync if T is Sync
unsafe impl<T: Sync, const N: usize> Sync for AlignedBuffer<T, N> {}

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alignment_constant() {
        assert_eq!(SIMD_ALIGNMENT, 64);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_aligned_vec_new() {
        let vec: AlignedVec<f32> = AlignedVec::new();
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.capacity(), 0);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_aligned_vec_with_capacity() {
        let vec: AlignedVec<f32> = AlignedVec::with_capacity(1024);
        assert_eq!(vec.len(), 0);
        assert!(vec.capacity() >= 1024);
        assert!(vec.is_aligned());
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_aligned_vec_push() {
        let mut vec: AlignedVec<f32> = AlignedVec::new();
        vec.push(1.0);
        vec.push(2.0);
        vec.push(3.0);
        assert_eq!(vec.len(), 3);
        assert_eq!(vec.as_slice(), &[1.0, 2.0, 3.0]);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_aligned_vec_from_slice() {
        let data = [1.0f32, 2.0, 3.0, 4.0];
        let vec = AlignedVec::from_slice(&data);
        assert_eq!(vec.as_slice(), &data);
        assert!(vec.is_aligned());
    }

    #[test]
    fn test_aligned_buffer_stack() {
        let mut buffer: AlignedBuffer<f32, 16> = AlignedBuffer::new();
        assert_eq!(buffer.len(), 0);
        assert_eq!(buffer.capacity(), 16);

        buffer.push(1.0).expect("push should succeed");
        buffer.push(2.0).expect("push should succeed");
        assert_eq!(buffer.len(), 2);
        assert_eq!(buffer.as_slice(), &[1.0, 2.0]);
    }

    #[test]
    fn test_aligned_buffer_full() {
        let mut buffer: AlignedBuffer<u8, 4> = AlignedBuffer::new();
        buffer.push(1).expect("push should succeed");
        buffer.push(2).expect("push should succeed");
        buffer.push(3).expect("push should succeed");
        buffer.push(4).expect("push should succeed");
        assert!(buffer.push(5).is_err());
    }

    #[test]
    fn test_stack_buffer_alignment() {
        let buffer: AlignedBuffer<f32, 16> = AlignedBuffer::new();
        let ptr = buffer.as_ptr() as usize;
        // Should be aligned to at least 64 bytes
        assert_eq!(ptr % 64, 0, "Buffer should be 64-byte aligned");
    }
}
