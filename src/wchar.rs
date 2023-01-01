use core::ptr;

use crate::{c_int, size_t, wchar_t};

#[no_mangle]
pub extern "C" fn gi_wmemchr(
  dst: *mut wchar_t,
  c: wchar_t,
  n: size_t
) -> *mut wchar_t {
  let mut s = dst;

  for _ in 0..n {
    unsafe {
      if *s == c {
        return s;
      }
    }
    s = s.wrapping_offset(1);
  }

  return ptr::null_mut();
}

#[no_mangle]
pub extern "C" fn gi_wmemcmp(
  s1: *mut wchar_t,
  s2: *mut wchar_t,
  n: size_t
) -> c_int {
  let mut d1 = s1;
  let mut d2 = s2;

  for _ in 0..n {
    unsafe {
      if *d1 != *d2 {
        return d2 as c_int - d1 as c_int;
      }
    }
    d1 = d1.wrapping_offset(1);
    d2 = d2.wrapping_offset(1);
  }

  return 0;
}

#[no_mangle]
pub extern "C" fn gi_wmemcpy(
  dst0: *mut wchar_t,
  src0: *const wchar_t,
  n: size_t
) -> *mut wchar_t {
  return gi_wmemmove(dst0, src0, n);
}

#[no_mangle]
pub extern "C" fn gi_wmemmove(
  ws1: *mut wchar_t,
  ws2: *const wchar_t,
  n: size_t
) -> *mut wchar_t {
  let mut wsb1 = ws1;
  let mut wsb2 = ws2;
  let mut i = n;

  if (wsb1 as *const wchar_t) < wsb2 {
    loop {
      let save = i;
      i = i.wrapping_sub(1);
      if !(save > 0) {
        break;
      }

      let wsd1 = wsb1;
      wsb1 = wsb1.wrapping_offset(1);
      let wsd2 = wsb2;
      wsb2 = wsb2.wrapping_offset(1);
      unsafe { *wsd1 = *wsd2; }
    }
  } else if (wsb1 as *const wchar_t) > wsb2 {
    wsb1 = wsb1.wrapping_offset(n as isize);
    wsb2 = wsb2.wrapping_offset(n as isize);

    loop {
      let save = i;
      i = i.wrapping_sub(1);
      if !(save > 0) {
        break;
      }

      wsb1 = wsb1.wrapping_offset(-1);
      wsb2 = wsb2.wrapping_offset(-1);
      unsafe { *wsb1 = *wsb2; }
    }
  }

  return ws1;
}

#[no_mangle]
pub extern "C" fn gi_wmemset(
  s: *mut wchar_t,
  c: wchar_t,
  n: size_t
) -> *mut wchar_t {
  let mut p = s;

  for _ in 0..n {
    unsafe { *p = c; }
    
    p = p.wrapping_offset(1);
  }

  return s;
}
