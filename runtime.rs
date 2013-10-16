/*
 * Runtime.rs
 *
 * Contains re-implemented parts of Rust's libstd that are needed here, for
 * convenience or because the compiler needs some of them to use some features
 * of the language.
 *
 * Contains some parts copied from Rust's official libstd. See
 * http://github.com/mozilla/rust/
 */

use self::libc::*;

pub mod intrinsics
{
    /*
     * Intrinsics are functions implemented in-compiler by directly inserting
     * LLVM code snippets.
     * One just need to declare their signature in order to be able to use
     * them properly.
     */

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

/*
 * TODO: implement range in the runtime
 * Iterators and options needs to be implemented first.
 */

/*
pub struct Range<A>
{
    priv state: A,
    priv stop: A,
    priv one: A
}

#[inline]
pub fn range<A: Add<A, A> + Ord + Clone + One>(start: A, stop: A) -> Range<A> {
    Range{state: start, stop: stop, one: One::one()}
}

impl<A: Add<A, A> + Ord + Clone> Iterator<A> for Range<A>
{
    #[inline]
    fn next(&mut self) -> Option<A>
    {
        if self.state < self.stop {
            let result = self.state.clone();
            self.state = self.state + self.one;
            Some(result)
        } else {
            None
        }
    }

    // FIXME: #8606 Implement size_hint() on Range
    // Blocked on #8605 Need numeric trait for converting to `Option<uint>`
}
*/

/* some libc types that are needed */

pub mod libc
{
    pub type c_char = u8;

    #[cfg(target_word_size = "32")]
    pub type size_t = u32;

    #[cfg(target_word_size = "64")]
    pub type size_t = u64;
}

#[lang="fail_"]
#[inline]
pub fn fail_(expr: *c_char, file: *c_char, line: size_t) -> !
{
    loop {}
}

#[lang="fail_bounds_check"]
#[inline]
pub fn fail_bounds_check(file: *c_char, line: size_t, index: size_t, len: size_t)
{
    use util::kprint;
    use util::kprintln;

    kprint("Index out of bounds at ");
//    kprint(file);
    kprintln("");

    /*
     * TODO: implement format, or print_int or whatever, to print the line no
     * and the index and size of the array
     */

    kprintln("Index is of size ?? but the array is of size ??");

    /* TODO: implement a panic() function */

    loop {}
}

pub mod cast
{
    use super::intrinsics;
    use super::sys;

    #[cfg(target_word_size = "32")]
    #[inline]
    pub unsafe fn transmute_copy<T, U>(src: &T) -> U 
    {
        let mut dest: U = intrinsics::uninit();
        let dest_ptr: *mut u8 = transmute(&mut dest);
        let src_ptr: *u8 = transmute(src);
        intrinsics::memcpy32(dest_ptr, src_ptr, sys::size_of::<U>() as u32);
        dest
    }

    #[cfg(target_word_size = "64")]
    #[inline]
    pub unsafe fn transmute_copy<T, U>(src: &T) -> U 
    {
        let mut dest: U = intrinsics::uninit();
        let dest_ptr: *mut u8 = transmute(&mut dest);
        let src_ptr: *u8 = transmute(src);
        intrinsics::memcpy64(dest_ptr, src_ptr, sys::size_of::<U>() as u64);
        dest
    }

    #[inline]
    pub unsafe fn transmute<L, G>(thing: L) -> G 
    {
        intrinsics::transmute(thing)
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
    #[inline]
    fn repr(&self) -> T { unsafe { cast::transmute_copy(self) } }
}

