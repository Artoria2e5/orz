/// Trait wrapper for unchecked_index.
pub trait UncheckedSliceExt<T> {
    unsafe fn nc<'a>(&'a self) -> unchecked_index::UncheckedIndex<&'a Self>;
    unsafe fn nc_mut<'a>(&'a mut self) -> unchecked_index::UncheckedIndex<&'a mut Self>;
}

impl<T> UncheckedSliceExt<T> for [T] {
    unsafe fn nc<'a>(&'a self) -> unchecked_index::UncheckedIndex<&'a Self> {
        unchecked_index::unchecked_index(self)
    }

    unsafe fn nc_mut<'a>(&'a mut self) -> unchecked_index::UncheckedIndex<&'a mut Self> {
        unchecked_index::unchecked_index(self)
    }
}

/// Unchecked byte operations.
pub trait ByteSliceExt {
    /// Read unaligned offset, disregarding type. Endian-unsafe.
    unsafe fn read<T>(&self, offset: usize) -> T;
    /// Write unaligned offset, disregarding type. Endian-unsafe.
    unsafe fn write<T>(&mut self, offset: usize, value: T);
    /// Like read, but mutates offset to `sizeof(T) + offset`.
    unsafe fn read_forward<T>(&self, offset: &mut usize) -> T;
    /// Like write, but mutates offset to `sizeof(T) + offset`.
    unsafe fn write_forward<T>(&mut self, offset: &mut usize, value: T);
}

impl ByteSliceExt for [u8] {
    unsafe fn read<T>(&self, offset: usize) -> T {
        return std::ptr::read_unaligned(self.as_ptr().add(offset) as *const T);
    }

    unsafe fn write<T>(&mut self, offset: usize, value: T) {
        std::ptr::write_unaligned(self.as_mut_ptr().add(offset) as *mut T, value);
    }

    unsafe fn read_forward<T>(&self, offset: &mut usize) -> T {
        let offset = {let old_offset = *offset; *offset += std::mem::size_of::<T>(); old_offset};
        return std::ptr::read_unaligned(self.as_ptr().add(offset) as *const T);
    }

    unsafe fn write_forward<T>(&mut self, offset: &mut usize, value: T) {
        let offset = {let old_offset = *offset; *offset += std::mem::size_of::<T>(); old_offset};
        std::ptr::write_unaligned(self.as_mut_ptr().add(offset) as *mut T, value);
    }
}
