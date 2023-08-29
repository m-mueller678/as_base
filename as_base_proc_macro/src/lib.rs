use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, Meta, Path};

fn as_base_crate() -> Path {
    match crate_name("as_base").unwrap() {
        FoundCrate::Name(as_base_crate) => syn::parse_str(&as_base_crate).unwrap(),
        FoundCrate::Itself => syn::parse_str("crate").unwrap(),
    }
}

/// implements [AsBase] for a struct or tuple struct.
///
/// The first field is used as base.
/// The type must have a `#[repr(C)]` attribute.
#[proc_macro_derive(AsBase)]
pub fn derive_as_base(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let Data::Struct(DataStruct { fields, .. }) = &input.data else {
        panic!("type must be a struct");
    };
    let is_repr_c = input.attrs.iter().any(|x| {
        let Meta::List(meta)=&x.meta else{return false};
        meta.path.get_ident().map(|x| x.to_string()).as_deref() == Some("repr")
            && meta.tokens.to_string() == "C"
    });
    assert!(is_repr_c, "struct must be #[repr(C)]");
    let base = match &fields {
        Fields::Named(x) => x.named.first(),
        Fields::Unnamed(x) => x.unnamed.first(),
        Fields::Unit => None,
    };
    let Some(base) = base else {
        panic!("struct must not be empty");
    };
    let base_type = &base.ty;
    let target_type = input.ident;
    let as_base_crate = as_base_crate();
    quote! {
        unsafe impl #as_base_crate::AsBase<#base_type> for #target_type {}
    }
    .into()
}
