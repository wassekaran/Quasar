#[lang = "add"]
pub trait Add<RHS, Result> {
    fn add(&self, rhs: &RHS) -> Result;
}

impl<T> Add<usize, *const T> for *const T {
    fn add(&self, offset: &usize) -> *const T {
        let ptr = *self as usize;
        (ptr + *offset) as *const T
    }
}

impl<T> Add<usize, *mut T> for *mut T {
    fn add(&self, offset: &usize) -> *mut T {
        let ptr = *self as usize;
        (ptr + *offset) as *mut T
    }
}

pub trait PartialOrd<Rhs: ?Sized = Self> {
    fn partial_cmp(&Self, other: &Rhs) -> Option<Ordering>;
}

impl<T> PartialOrd<usize, *mut T> for *mut T {
    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>
    fn add(&self, offset: &usize) -> *mut T {
        let ptr = *self as usize;
        (ptr + *offset) as *mut T
    }
}
