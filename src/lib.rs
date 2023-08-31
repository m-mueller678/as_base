#![doc = include_str!("../README.md")]

/// Implements [AsBase].
///
/// Works on structs and tuple structs.
/// The first field is used as base.
/// The type must have a `#[repr(C)]` attribute.
pub use as_base_proc_macro::AsBase;

pub use as_base_proc_macro::*;

/// Marker trait for base casting.
///
/// This indicates that it is safe to transmute references to `Self` into Refrences to `T`.
/// This trait on its own introduces no new requirements, it merely collects all the marker
/// traits for the different kinds of references for convenience.
///
/// You can use the derive macro to implement all the marker traits at once.
pub trait AsBase<T>: AsBaseRef<T> + AsBaseMut<T> + AsBasePin<T> + AsBasePinMut<T> {}

macro_rules! sub_trait {
    ($Trait:ident,$ExtTrait:ident,$function:ident,$In:ty,$Out:ty) => {
        #[doc = "Marker trait to indicate that transmuting `"]
        #[doc =std::stringify!($In)]
        #[doc = "` to `"]
        #[doc =std::stringify!($Out)]
        #[doc = "` is safe.\n\n["]
        #[doc =std::stringify!($ExtTrait)]
        #[doc = "] makes use of this."]
        pub unsafe trait $Trait<T> {}

        #[doc = "Extension method for ["]
        #[doc =std::stringify!($Trait)]
        #[doc = "]."]
        pub trait $ExtTrait<T>: $Trait<T> {
            fn $function(self: $In) -> $Out {
                #[allow(clippy::transmute_ptr_to_ref)]
                unsafe {
                    std::mem::transmute(&*self as *const Self as *const u8)
                }
            }
        }

        impl<B, T: $Trait<B> + ?Sized> $ExtTrait<B> for T {}
    };
}

sub_trait!(AsBaseRef, AsBaseRefExt, as_base, &Self, &T);
sub_trait!(AsBaseMut, AsBaseMutExt, as_base_mut, &mut Self, &mut T);
sub_trait!(
    AsBasePin,
    AsBasePinExt,
    as_base_pin,
    std::pin::Pin<&Self>,
    std::pin::Pin<&T>
);
sub_trait!(
    AsBasePinMut,
    AsBasePinMutExt,
    as_base_pin_mut,
    std::pin::Pin<&mut Self>,
    std::pin::Pin<&mut T>
);
