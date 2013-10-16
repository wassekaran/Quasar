pub mod intrinsics
{
    extern "rust-intrinsic" 
    {
        pub fn transmute<T,U>(e: T) -> U;
        pub fn memcpy64<T>(dst: *mut T, src: *T, count: u64);
        pub fn memcpy32<T>(dst: *mut T, src: *T, count: u32);
        pub fn uninit<T>() -> T;
        pub fn size_of<T>() -> uint;
        pub fn offset<T>(dst: *T, offset: int) -> *T;

    }
}

pub mod cast
{
    #[cfg(target_word_size = "32")]
    #[inline]
    pub unsafe fn transmute_copy<T, U>(src: &T) -> U 
    {
        let mut dest: U = super::intrinsics::uninit();
        let dest_ptr: *mut u8 = transmute(&mut dest);
        let src_ptr: *u8 = transmute(src);
        super::intrinsics::memcpy32(dest_ptr, src_ptr, super::sys::size_of::<U>() as u32);
        dest
    }

    /// Casts the value at `src` to U. The two types must have the same length.
    #[cfg(target_word_size = "64")]
    #[inline]
    pub unsafe fn transmute_copy<T, U>(src: &T) -> U 
    {
        let mut dest: U = super::intrinsics::uninit();
        let dest_ptr: *mut u8 = transmute(&mut dest);
        let src_ptr: *u8 = transmute(src);
        super::intrinsics::memcpy64(dest_ptr, src_ptr, super::sys::size_of::<U>() as u64);
        dest
    }

    #[inline]
    pub unsafe fn transmute<L, G>(thing: L) -> G 
    {
        super::intrinsics::transmute(thing)
    }
}

pub mod sys
{
    #[inline]
    pub fn size_of<T>() -> uint 
    {
        unsafe { super::intrinsics::size_of::<T>() }
    }
}

pub struct Slice<T> 
{
    data: *T,
    len: uint
}

pub trait Repr<T> 
{
    /// This function "unwraps" a rust value (without consuming it) into its raw
    /// struct representation. This can be used to read/write different values
    /// for the struct. This is a safe method because by default it does not
    /// give write-access to the struct returned.
    #[inline]
    fn repr(&self) -> T { unsafe { cast::transmute_copy(self) } }
}

