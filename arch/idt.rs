#[cfg(target_arch = "x86_64")]

pub use self::x86_64::idt::*;

pub mod x86_64
{
    pub mod idt;
}

