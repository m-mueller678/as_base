//! # as_base
//! This crate allows directly accessing fields within a trait object similar to C++ public base classes.
//! No virtual dispatch is involved, the base object always begins at the same address as the enclosing object.
//! ```
//! # use as_base::*;
//! struct BaseType(u64);
//!
//! trait SayHello: AsBase<BaseType> {
//!     fn say_hello(&self) {
//!         println!("hello from {}!", self.as_base().0);
//!     }
//! }
//!
//! #[derive(AsBase)]
//! #[repr(C)]
//! struct Implementor {
//!     base: BaseType,
//! }
//!
//! impl SayHello for Implementor {}
//!
//! fn main() {
//!     let mut x = Implementor { base: BaseType(4) };
//!     let dyn_reference: &mut dyn SayHello = &mut x;
//!     let base: &mut BaseType = dyn_reference.as_base_mut();
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
