use core::{ffi, ptr};

use crate::{c_int, c_char, c_uchar, locale_t, size_t};

#[no_mangle]
pub extern "C" fn gi_memccpy(
  t: *mut ffi::c_void,
  f: *const ffi::c_void,
  c: c_int,
  n: size_t
) -> *mut ffi::c_void {
  let uc: c_uchar = c as c_uchar;
  let mut tp: *mut c_uchar = t as *mut c_uchar;
  let mut fp: *const c_uchar = f as *const c_uchar;
  let mut i = n;

  if i != 0 {
    loop {
      let save = i;
      i = i.wrapping_sub(1);
      if !(save != 0) {
        break;
      }

      let fp1 = fp;
      fp = fp.wrapping_offset(1);
      let tp1 = tp;
      tp = tp.wrapping_offset(1);
      unsafe {
        *tp1 = *fp1;

        if *tp1 == uc {
          return tp as *mut ffi::c_void;
        }
      }
    }
  }

  return 0 as *mut ffi::c_void;
}

#[no_mangle]
pub extern "C" fn gi_memchr(
  s: *const ffi::c_void,
  c: c_int,
  n: size_t
) -> *mut ffi::c_void {
  let mut i = n;

  if i != 0 {
    let mut p: *mut c_uchar = s as *mut c_uchar;
    loop {
      let save = i;
      i = i.wrapping_sub(1);
      if !(save != 0) {
        break;
      }

      let p1 = p;
      p = p.wrapping_offset(1);
      unsafe {
        if *p1 == c as c_uchar {
          return p.wrapping_sub(1) as *mut ffi::c_void;
        }
      }
    }
  }

  return ptr::null_mut();
}

#[no_mangle]
pub extern "C" fn gi_memcmp(
  s1: *const ffi::c_void,
  s2: *const ffi::c_void,
  n: size_t
) -> c_int {
  let mut i = n;

  if i != 0 {
    let mut p1: *const c_uchar = s1 as *const c_uchar;
    let mut p2: *const c_uchar = s2 as *const c_uchar;

    loop {
      let save = i;
      i = i.wrapping_sub(1);
      if !(save != 0) {
        break;
      }

      p1 = p1.wrapping_offset(1);
      p2 = p2.wrapping_offset(1);
      unsafe {
        if *p1 != *p2 {
          return p2 as c_int - p1 as c_int;
        }
      }
    }
  }

  return 0;
}

#[no_mangle]
pub extern "C" fn gi_memcpy(
  dst0: *mut ffi::c_void,
  src0: *const ffi::c_void,
  n: size_t
) -> *mut ffi::c_void {
  return gi_memmove(dst0, src0, n);
}

#[no_mangle]
pub extern "C" fn gi_memmove(
  dst0: *mut ffi::c_void,
  src0: *const ffi::c_void,
  n: size_t
) -> *mut ffi::c_void {
  let mut dst: *mut c_char = dst0 as *mut c_char;
  let mut src: *mut c_char = src0 as *mut c_char;
  let mut i = n;

  if dst < src {
    loop {
      let save = i;
      i = i.wrapping_sub(1);
      if !(save > 0) {
        break;
      }

      let dst1 = dst;
      dst = dst.wrapping_offset(1);
      let src1 = src;
      src = src.wrapping_offset(1);
      unsafe { *dst1 = *src1; }
    }
  } else if dst > src {
    dst = dst.wrapping_offset(n as isize);
    src = src.wrapping_offset(n as isize);

    loop {
      let save = i;
      i = i.wrapping_sub(1);
      if !(save > 0) {
        break;
      }

      dst = dst.wrapping_offset(-1);
      src = src.wrapping_offset(-1);
      unsafe { *dst = *src; }
    }
  }

  return dst0;
}

#[no_mangle]
pub extern "C" fn gi_memrchr(
  s: *const ffi::c_void,
  c: c_int,
  n: size_t
) -> *mut ffi::c_void {
  let mut i = n;

  if i != 0 {
    let mut p: *mut c_uchar = (s as *mut c_uchar).wrapping_offset(n as isize);
    loop {
      let save = i;
      i = i.wrapping_sub(1);
      if !(save != 0) {
        break;
      }

      p = p.wrapping_offset(-1);
      unsafe {
        if *p == c as c_uchar {
          return p as *mut ffi::c_void;
        }
      }
    }
  }

  return ptr::null_mut();
}

#[no_mangle]
pub extern "C" fn gi_memset(
  dst: *mut ffi::c_void,
  c: c_int,
  n: size_t
) -> *mut ffi::c_void {
  let mut i = n;

  if i != 0 {
    let mut d: *mut c_uchar = dst as *mut c_uchar;

    loop {
      let save = i;
      i = i.wrapping_sub(1);
      if !(save != 0) {
        break;
      }

      let d1 = d;
      d = d.wrapping_offset(1);
      unsafe { *d1 = c as c_uchar; }
    }
  }

  return dst;
}

#[no_mangle]
pub extern "C" fn gi_stpcpy(
  dst: *mut c_char,
  src: *const c_char
) -> *mut c_char {
  let slice = unsafe { ffi::CStr::from_ptr(src) };
  return gi_stpncpy(dst, src, slice.to_bytes().len() + 1);
}

#[no_mangle]
pub extern "C" fn gi_stpncpy(
  dst: *mut c_char,
  src: *const c_char,
  n: size_t
) -> *mut c_char {
  let mut i = n;
  let mut d = dst;
  let mut s = src;

  unsafe {
    while i > 0 && *s as c_int != '\0' as c_int {
      let d1 = d;
      d = d.wrapping_offset(1);
      let s1 = s;
      s = s.wrapping_offset(1);
      *d1 = *s1;
      i = i.wrapping_sub(1);
    }
  }

  let end = d;
  loop {
      let save = i;
      i = i.wrapping_sub(1);
      if !(save > 0) {
        break;
      }

      let d1 = d;
      d = d.wrapping_offset(1);
      unsafe { *d1 = '\0' as c_char; }
  }

  return end;
}

#[no_mangle]
pub extern "C" fn gi_strncat(
  dst: *mut c_char,
  src: *const c_char,
  n: size_t
) -> *mut c_char {
  let mut i = n;

  if n != 0 {
    let mut d = dst;
    let mut s = src;

    unsafe {
      while *d != 0 {
        d = d.wrapping_offset(1);
      }
    }

    loop {
      let save = i;
      i = i.wrapping_sub(1);
      if !(save != 0) {
        break;
      }

      let s1 = s;
      s = s.wrapping_offset(1);
      unsafe {
        *d = *s1;
        if *d == 0 {
          break;
        }
      }
      d = d.wrapping_offset(1);
    }

    unsafe { *d = 0 };
  }

  return dst;
}

#[no_mangle]
pub extern "C" fn gi_strcat(
  dst: *mut c_char,
  src: *const c_char
) -> *mut c_char {
  let slice = unsafe { ffi::CStr::from_ptr(src) };
  return gi_strncat(dst, src, slice.to_bytes().len() + 1);
}

#[no_mangle]
pub extern "C" fn gi_strchr(
  p: *const c_char,
  ch: c_int
) -> *mut c_char {
  let mut p1 = p;
  loop {
    unsafe {
      if *p1 == ch as c_char {
        return p1 as *mut c_char;
      }

      if *p1 == 0 {
        return ptr::null_mut();
      }
    }

    p1 = p1.wrapping_offset(1);
  }
}

#[no_mangle]
pub extern "C" fn gi_strrchr(
  p: *const c_char,
  ch: c_int
) -> *mut c_char {
  let mut p1 = p;
  let mut save: *mut c_char = 0 as *mut c_char;
  loop {
    unsafe {
      if *p1 == ch as c_char {
        save = p1 as *mut c_char;
      }

      if *p1 == 0 {
        return save;
      }
    }

    p1 = p1.wrapping_offset(1);
  }
}

#[no_mangle]
pub extern "C" fn gi_strncmp(
  dst: *const c_char,
  src: *const c_char,
  n: size_t
) -> c_int {
  let mut i = n;

  if n != 0 {
    let mut d = dst;
    let mut s = src;

    loop {
      let save = i;
      i = i.wrapping_sub(1);
      if !(save != 0) {
        break;
      }

      let s1 = s;
      s = s.wrapping_offset(1);
      unsafe {
        if *d != *s1 {
          return d as c_int - s1 as c_int;
        }
      }

      let d1 = d;
      d = d.wrapping_offset(1);

      unsafe {
        if *d1 == 0 {
          break;
        }
      }
    }
  }

  return 0;
}

#[no_mangle]
pub extern "C" fn gi_strcmp(
  dst: *const c_char,
  src: *const c_char
) -> c_int {
  let slice = unsafe { ffi::CStr::from_ptr(src) };
  return gi_strncmp(dst, src, slice.to_bytes().len() + 1);
}

#[no_mangle]
pub extern "C" fn gi_strncpy(
  dst: *mut c_char,
  src: *const c_char,
  n: size_t
) -> *mut c_char {
  let mut i = n;

  if n != 0 {
    let mut d = dst;
    let mut s = src;

    loop {
      let mut d1 = d;
      d = d.wrapping_offset(1);
      let s1 = s;
      s = s.wrapping_offset(1);
      unsafe {
        *d1 = *s1;
        if *d1 == 0 {
          loop {
            let save = i;
            i = i.wrapping_sub(1);
            if !(save != 0) {
              break;
            }
            let d2 = d1;
            d1 = d1.wrapping_offset(1);
            *d2 = 0;
          }
          break;
        } else {
          let save = i;
          i = i.wrapping_sub(1);
          if !(save != 0) {
            break;
          }
        }
      }
    }
  }

  return dst;
}

#[no_mangle]
pub extern "C" fn gi_strcpy(
  dst: *mut c_char,
  src: *const c_char
) -> *mut c_char {
  let slice = unsafe { ffi::CStr::from_ptr(src) };
  return gi_strncpy(dst, src, slice.to_bytes().len() + 1);
}

#[no_mangle]
pub extern "C" fn gi_strcoll(
  dst: *const c_char,
  src: *const c_char
) -> c_int {
  return gi_strcmp(dst, src);
}

#[no_mangle]
pub extern "C" fn gi_strcoll_l(
  dst: *const c_char,
  src: *const c_char,
  _locale: locale_t
) -> c_int {
  return gi_strcmp(dst, src);
}
