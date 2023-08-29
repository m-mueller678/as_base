//! # as_base
//! This crate allows directly accessing fields within a trait object similar to C++ public base classes.
//! No virtual dispatch is involved, the base object always begins at the same address as the enclosing object.
//! ```
//! # use as_base::*;
//! struct BaseType {
//!     x: u64,
//! }
//!
//! trait MyTrait: AsBase<BaseType> {}
//!
//! #[derive(AsBase)]
//! #[repr(C)]
//! struct Implementor {
//!     pub base: BaseType,
//! }
//!
//! impl MyTrait for Implementor {}
//!
//! fn main() {
//!     let x = Implementor {
//!         base: BaseType { x: 42 },
//!     };
//!     let dyn_reference = &x as &dyn MyTrait;
//!     assert_eq!(dyn_reference.as_base().x, 42)
//! }
//! ```

pub use as_base_proc_macro::AsBase;

/// Marker trait for base casting.
///
/// Implementors guarantee that an object of type T resides at the start of Self and that it can be safely accessed.
/// This is what enables all the casting.
///
/// Usually you do not want to implement this manually, use the derive macro.
/// # Safety
/// It must be safe to cast immutable and mutable references to Self into references to `T` with same lifetime and mutability via slim pointers.
pub unsafe trait AsBase<T> {}

/// Extension methods for [AsBase].
///
/// This is automatically implemented for all types implementing [AsBase].
/// You are not supposed to implement it yourself.
pub trait AsBaseExt<B>: AsBase<B> {
    /// casts to the base type.
    fn as_base(&self) -> &B {
        unsafe { &*(self as *const Self as *const u8 as *const B) }
    }
    /// casts to the base type.
    fn as_base_mut(&mut self) -> &mut B {
        unsafe { &mut *(self as *mut Self as *mut u8 as *mut B) }
    }
}

impl<B, T: AsBase<B> + ?Sized> AsBaseExt<B> for T {}
