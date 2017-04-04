use libc::c_char;
use std::borrow::Borrow;
use std::ffi::{ CStr, CString };
use ::*;
pub use ::dynamic::CaseType;

#[no_mangle]
pub unsafe extern "C" fn caseconv_convert_case(src: *const c_char, src_case: CaseType, dst_case: CaseType) -> *mut c_char {
    let src = CStr::from_ptr(src).to_string_lossy();
    let src = src.borrow();

    let ret_owned = CString::from_vec_unchecked(convert(src, &src_case, &dst_case).into_bytes());
    ret_owned.into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn caseconv_unjumble(src: *const c_char, dst_case: CaseType) -> *mut c_char {
    let src = CStr::from_ptr(src).to_string_lossy();
    let src = src.borrow();

    let ret_owned = CString::from_vec_unchecked(convert(src, ::dynamic::JUMBLED, &dst_case).into_bytes());
    ret_owned.into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn caseconv_guess_case(src: *const c_char) -> CaseType {
    let src = CStr::from_ptr(src).to_string_lossy();
    let src = src.borrow();

    CaseType::guess(src)
}

#[no_mangle]
pub unsafe extern "C" fn caseconv_guess_and_convert(src: *const c_char, dst_case: CaseType) -> *mut c_char {
    let src = CStr::from_ptr(src).to_string_lossy();
    let src = src.borrow();

    let ret_owned = guess_and_convert(src, &dst_case);
    CString::from_vec_unchecked(ret_owned.into_bytes()).into_raw()
}
