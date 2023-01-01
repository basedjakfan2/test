cfg_if! {
  if #[cfg(any(target_arch = "s390x"))] {
    mod s390x;
    pub use self::s390x::*;
  } else if #[cfg(any(target_arch = "x86_64"))] {
    mod x86_64;
    pub use self::x86_64::*;
  }
}
