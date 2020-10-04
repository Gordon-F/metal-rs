
use objc::runtime::{Object, NO, BOOL};

const UTF8_ENCODING: usize = 4;

#[allow(non_camel_case_types)]
pub type id = *mut Object;
#[allow(non_upper_case_globals)]
pub const nil: id = 0 as id;

#[cfg(target_pointer_width = "32")]
pub type NSInteger = i32;
#[cfg(target_pointer_width = "32")]
pub type NSUInteger = u32;

#[cfg(target_pointer_width = "64")]
pub type NSInteger = i64;
#[cfg(target_pointer_width = "64")]
pub type NSUInteger = u64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NSRange {
    pub location: NSUInteger,
    pub length: NSUInteger,
}

impl NSRange {
    #[inline]
    pub fn new(location: NSUInteger, length: NSUInteger) -> NSRange {
        NSRange {
            location,
            length,
        }
    }
}

pub trait NSString: Sized {
    unsafe fn alloc(_: Self) -> id {
        msg_send![class!(NSString), alloc]
    }

    unsafe fn stringByAppendingString_(self, other: id) -> id;
    unsafe fn init_str(self, string: &str) -> Self;
    unsafe fn UTF8String(self) -> *const i8;
    unsafe fn len(self) -> usize;
    unsafe fn isEqualToString(self, other: &str) -> bool;
    unsafe fn substringWithRange(self, range: NSRange) -> id;
}

impl NSString for id {
    unsafe fn isEqualToString(self, other: &str) -> bool {
        let other = NSString::alloc(nil).init_str(other);
        let rv: BOOL = msg_send![self, isEqualToString:other];
        rv != NO
    }

    unsafe fn stringByAppendingString_(self, other: id) -> id {
        msg_send![self, stringByAppendingString:other]
    }

    unsafe fn init_str(self, string: &str) -> id {
        return msg_send![self,
                         initWithBytes:string.as_ptr()
                             length:string.len()
                             encoding:UTF8_ENCODING as id];
    }

    unsafe fn len(self) -> usize {
        msg_send![self, lengthOfBytesUsingEncoding:UTF8_ENCODING]
    }

    unsafe fn UTF8String(self) -> *const i8 {
        msg_send![self, UTF8String]
    }

    unsafe fn substringWithRange(self, range: NSRange) -> id {
        msg_send![self, substringWithRange:range]
    }
}
