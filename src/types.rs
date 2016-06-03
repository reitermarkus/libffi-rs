//! Representations of C types and arrays thereof.

use std::{mem, ptr};
use libc;

use low;

type Type_      = *mut low::ffi_type;
type TypeArray_ = *mut Type_;
type Owned<T>      = T;

#[derive(Debug)]
pub struct Type(Type_);

#[derive(Debug)]
pub struct TypeArray(TypeArray_);

/// Computes the length of a raw `TypeArray_` by searching for the
/// null terminator.
unsafe fn ffi_type_array_len(mut array: TypeArray_) -> usize {
    let mut count   = 0;
    while !(*array).is_null() {
        count += 1;
        array = array.offset(1);
    }
    count
}

/// Creates an empty `TypeArray_` with null terminator.
unsafe fn ffi_type_array_create_empty(len: usize) -> Owned<TypeArray_> {
    let array = libc::malloc((len + 1) * mem::size_of::<Type_>())
                    as TypeArray_;
    assert!(!array.is_null());
    *array.offset(len as isize) = ptr::null::<low::ffi_type>() as Type_;
    array
}

/// Creates a null-terminated array of Type_. Takes ownership of
/// the elements.
unsafe fn ffi_type_array_create(elements: Vec<Type>)
    -> Owned<TypeArray_>
{
    let size = elements.len();
    let new  = ffi_type_array_create_empty(size);
    for i in 0 .. size {
        *new.offset(i as isize) = elements[i].0;
    }

    for t in elements {
        mem::forget(t);
    }

    new
}

/// Creates a struct type from a raw array of element types.
unsafe fn ffi_type_struct_create_raw(elements: Owned<TypeArray_>)
    -> Owned<Type_>
{
    let new = libc::malloc(mem::size_of::<low::ffi_type>()) as Type_;
    assert!(!new.is_null());

    (*new).size      = 0;
    (*new).alignment = 0;
    (*new).type_     = low::type_tag::STRUCT;
    (*new).elements  = elements;

    new
}

/// Creates a struct ffi_type with the given elements. Takes ownership
/// of the elements.
unsafe fn ffi_type_struct_create(elements: Vec<Type>) -> Owned<Type_> {
    ffi_type_struct_create_raw(ffi_type_array_create(elements))
}

/// Makes a copy of a type array.
unsafe fn ffi_type_array_clone(old: TypeArray_) -> Owned<TypeArray_> {
    let size = ffi_type_array_len(old);
    let new  = ffi_type_array_create_empty(size);

    for i in 0 .. size {
        *new.offset(i as isize) = ffi_type_clone(*old.offset(i as isize));
    }

    new
}

/// Makes a copy of a type.
unsafe fn ffi_type_clone(old: Type_) -> Owned<Type_> {
    if (*old).type_ == low::type_tag::STRUCT {
        ffi_type_struct_create_raw(ffi_type_array_clone((*old).elements))
    } else {
        old
    }
}

/// Destroys an array of Type_ and all of its elements.
unsafe fn ffi_type_array_destroy(victim: Owned<TypeArray_>) {
    let mut current = victim;
    while !(*current).is_null() {
        ffi_type_destroy(*current);
        current = current.offset(1);
    }

    libc::free(victim as *mut libc::c_void);
}

/// Destroys an Type_ if it was dynamically allocated.
unsafe fn ffi_type_destroy(victim: Owned<Type_>) {
    if (*victim).type_ == low::type_tag::STRUCT {
        ffi_type_array_destroy((*victim).elements);
        libc::free(victim as *mut libc::c_void);
    }
}

impl Drop for Type {
    fn drop(&mut self) {
        unsafe { ffi_type_destroy(self.0) }
    }
}

impl Drop for TypeArray {
    fn drop(&mut self) {
        unsafe { ffi_type_array_destroy(self.0) }
    }
}

impl Clone for Type {
    fn clone(&self) -> Self {
        unsafe { Type(ffi_type_clone(self.0)) }
    }
}

impl Clone for TypeArray {
    fn clone(&self) -> Self {
        unsafe { TypeArray(ffi_type_array_clone(self.0)) }
    }
}

impl Type {
    pub fn void() -> Self {
        Type(unsafe { &mut low::ffi_type_void })
    }

    pub fn uint8() -> Self {
        Type(unsafe { &mut low::ffi_type_uint8 })
    }

    pub fn sint8() -> Self {
        Type(unsafe { &mut low::ffi_type_sint8 })
    }

    pub fn uint16() -> Self {
        Type(unsafe { &mut low::ffi_type_uint16 })
    }

    pub fn sint16() -> Self {
        Type(unsafe { &mut low::ffi_type_sint16 })
    }

    pub fn uint32() -> Self {
        Type(unsafe { &mut low::ffi_type_uint32 })
    }

    pub fn sint32() -> Self {
        Type(unsafe { &mut low::ffi_type_sint32 })
    }

    pub fn uint64() -> Self {
        Type(unsafe { &mut low::ffi_type_uint64 })
    }

    pub fn sint64() -> Self {
        Type(unsafe { &mut low::ffi_type_sint64 })
    }

    pub fn float() -> Self {
        Type(unsafe { &mut low::ffi_type_float })
    }

    pub fn double() -> Self {
        Type(unsafe { &mut low::ffi_type_double })
    }

    pub fn pointer() -> Self {
        Type(unsafe { &mut low::ffi_type_pointer })
    }

    pub fn longdouble() -> Self {
        Type(unsafe { &mut low::ffi_type_longdouble })
    }

    pub fn complex_float() -> Self {
        Type(unsafe { &mut low::ffi_type_complex_float })
    }

    pub fn complex_double() -> Self {
        Type(unsafe { &mut low::ffi_type_complex_double })
    }

    pub fn complex_longdouble() -> Self {
        Type(unsafe { &mut low::ffi_type_complex_longdouble })
    }

    pub fn structure(fields: Vec<Type>) -> Self {
        unsafe {
            Type(ffi_type_struct_create(fields))
        }
    }

    pub fn structure_from_array(fields: TypeArray) -> Self {
        unsafe {
            Type(ffi_type_struct_create_raw(fields.0))
        }
    }

    pub fn as_raw_ptr(&self) -> *mut low::ffi_type {
        self.0
    }
}

impl TypeArray {
    pub fn new(elements: Vec<Type>) -> Self {
        unsafe { TypeArray(ffi_type_array_create(elements)) }
    }

    pub fn len(&self) -> usize {
        unsafe { ffi_type_array_len(self.0) }
    }

    pub fn as_raw_ptr(&self) -> *mut *mut low::ffi_type {
        self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_uint64() {
        Type::uint64();
    }

    #[test]
    fn clone_uint64() {
        Type::uint64().clone().clone();
    }

    #[test]
    fn create_struct() {
        Type::structure(vec![Type::sint64(),
                             Type::sint64(),
                             Type::uint64()]);
    }

    #[test]
    fn clone_struct() {
        Type::structure(vec![Type::sint64(),
                             Type::sint64(),
                             Type::uint64()]).clone().clone();
    }

}
