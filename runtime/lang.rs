// contains a reimplementation of Rust lang items and intrinsics
// used inside Quasar

pub mod intrinsics {
    extern "rust-intrinsic" {
        pub fn copy_nonoverlapping_memory<T>(src: *mut T, dst: *T, count: uint);
        pub fn transmute<T, U>(e: T) -> U;
        pub fn uninit<T>() -> T;
    }
}

mod kinds {
    #[lang = "sized"]
    trait Size {}
}

mod failure {
    #[lang = "fail_bounds_check"]
    fn fail_bounds_check(_: *u8, _: uint, _: uint, _: uint) -> ! {
        ::runtime::io::println("Bound checking failed");
        loop {}
    }
}

pub mod ptr {
    use super::intrinsics;

    pub unsafe fn read<T>(src: *T) -> T {
        let mut ret = intrinsics::uninit();
        intrinsics::copy_nonoverlapping_memory(&mut ret, src, 1);
        ret
    }
}

pub mod repr {
    use super::transmute_copy;

    pub struct Slice<T> {
        pub data: *T,
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
    let ptr: *U = intrinsics::transmute(t);
    ptr::read(ptr)
}
