// contains a reimplementation of Rust lang items and intrinsics
// used inside Quasar

pub mod intrinsics {
    extern "rust-intrinsic" {
        pub fn copy_nonoverlapping_memory<T>(src: *mut T, dst: *const T, count: uint);
        pub fn transmute<T, U>(e: T) -> U;
        pub fn uninit<T>() -> T;
    }
}

mod kinds {
    #[lang = "sized"] trait Sized {}
    #[lang = "copy" ] trait Copy for Sized? {}
}

mod failure {
    #[lang = "fail_bounds_check"]
    fn fail_bounds_check(_: &(&'static str, uint), _: uint, _: uint) -> ! {
        ::runtime::io::println("Bound checking failed");
        loop {}
    }
}

pub mod ptr {
    use super::intrinsics;

    pub unsafe fn read<T>(src: *const T) -> T {
        let mut ret = intrinsics::uninit();
        intrinsics::copy_nonoverlapping_memory(&mut ret, src, 1);
        ret
    }
}

pub mod repr {
    use super::transmute_copy;

    pub struct Slice<T> {
        pub data: *const T,
        pub len: uint
    }

    pub trait Repr<T> {
        #[inline(always)]
        fn repr(&self) -> T {
            unsafe { transmute_copy(self) }
        }
    }

    impl<'a> Repr<Slice<u8>> for &'a str {}
}

pub unsafe fn transmute_copy<T, U>(t: &T) -> U {
    let ptr: *const U = intrinsics::transmute(t);
    ptr::read(ptr)
}
