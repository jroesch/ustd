use alloc::boxed::Box;
use core::mem::transmute;
use collections::Vec;

use super::c_char;

pub struct CString {
    inner: Box<[u8]>
}

impl CString {
    pub fn new(bytes: Vec<u8>) -> Result<CString, ()> {
        // let bytes = t.into();
        match bytes.iter().position(|x| *x == 0) {
            Some(i) => Err(()), // (NulError(i, bytes)),
            None => Ok(unsafe { CString::from_vec_unchecked(bytes) }),
        }
    }

    pub unsafe fn from_vec_unchecked(mut v: Vec<u8>) -> CString {
       v.push(0);
       CString { inner: v.into_boxed_slice() }
    }

    pub fn as_ptr(&self) -> *const c_char {
        unsafe { transmute(self.inner.as_ptr()) }
    }
}
