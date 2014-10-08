#[lang = "add"]
pub trait Add<RHS, Result> {
    fn add(&self, rhs: &RHS) -> Result;
}

impl<T> Add<uint, *const T> for *const T {
    fn add(&self, offset: &uint) -> *const T {
        let ptr = *self as uint;
        (ptr + *offset) as *const T
    }
}

impl<T> Add<uint, *mut T> for *mut T {
    fn add(&self, offset: &uint) -> *mut T {
        let ptr = *self as uint;
        (ptr + *offset) as *mut T
    }
}
