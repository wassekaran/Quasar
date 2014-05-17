#[lang = "add"]
pub trait Add<RHS, Result> {
    fn add(&self, rhs: &RHS) -> Result;
}

impl<T> Add<uint, *T> for *T {
    fn add(&self, offset: &uint) -> *T {
        let ptr = *self as uint;
        (ptr + *offset) as *T
    }
}

impl<T> Add<uint, *mut T> for *mut T {
    fn add(&self, offset: &uint) -> *mut T {
        let ptr = *self as uint;
        (ptr + *offset) as *mut T
    }
}
