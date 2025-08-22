use std::fmt::{Debug, Display, Formatter};
use std::mem;

const _: () = assert!(mem::size_of::<Object>() == 4 || mem::size_of::<Object>() == 8);

#[derive(Clone, Copy, Eq)]
#[repr(C)]
pub union Object {
    pub common: *mut ObjCommon,
    pub string: *mut ObjString,
}

impl Object {
    pub fn type_(&self) -> ObjType {
        unsafe { (*self.common).type_ }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.type_() {
            ObjType::String => unsafe { write!(f, "{}", (*self.string).value) },
        }
    }
}

impl Debug for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{self}")
    }
}

impl From<*mut ObjString> for Object {
    fn from(string: *mut ObjString) -> Self {
        Self { string }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        if self.type_() == ObjType::String && other.type_() == ObjType::String {
            return unsafe { (*self.string) == (*other.string) };
        }
        // this should be enough when arenas and GC are implemented
        unsafe { self.common == other.common }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct ObjCommon {
    pub type_: ObjType,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum ObjType {
    String,
}

#[derive(Debug)]
#[repr(C)]
pub struct ObjString {
    pub common: ObjCommon,
    pub value: &'static str,
}

// this will get simplified once we have GC
impl PartialEq for ObjString {
    fn eq(&self, other: &Self) -> bool {
        (*self.value).to_string() == (*other.value).to_string()
    }
}

impl ObjString {
    pub fn new(value: &'static str) -> Self {
        let common = ObjCommon {
            type_: ObjType::String,
        };
        Self { common, value }
    }
}
