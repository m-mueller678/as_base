use proc_macro_crate::{crate_name, FoundCrate};
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, Meta, Path, Visibility};

fn as_base_crate() -> Path {
    match crate_name("as_base").unwrap() {
        FoundCrate::Name(as_base_crate) => syn::parse_str(&as_base_crate).unwrap(),
        FoundCrate::Itself => syn::parse_str("crate").unwrap(),
    }
}

#[proc_macro_derive(AsBase)]
pub fn derive_as_base(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_as_base_impl(
        input,
        &[
            (true, "AsBaseRef"),
            (true, "AsBaseMut"),
            (true, "AsBasePin"),
            (true, "AsBasePinMut"),
            (false, "AsBase"),
        ],
    )
}

/// More specific version of [AsBase].
#[proc_macro_derive(AsBaseRef)]
pub fn derive_as_base_ref(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_as_base_impl(input, &[(true, "AsBaseRef")])
}

/// More specific version of [AsBase].
#[proc_macro_derive(AsBaseMut)]
pub fn derive_as_base_mut(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_as_base_impl(input, &[(true, "AsBaseMut")])
}

/// More specific version of [AsBase].
#[proc_macro_derive(AsBasePin)]
pub fn derive_as_base_pin(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_as_base_impl(input, &[(true, "AsBasePin")])
}

/// More specific version of [AsBase].
#[proc_macro_derive(AsBasePinMut)]
pub fn derive_as_base_pin_mut(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_as_base_impl(input, &[(true, "AsBasePinMut")])
}

fn derive_as_base_impl(
    input: proc_macro::TokenStream,
    traits: &[(bool, &str)],
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let Data::Struct(DataStruct { fields, .. }) = &input.data else {
        panic!("type must be a struct");
    };
    let is_repr_c = input.attrs.iter().any(|x| {
        let Meta::List(meta) = &x.meta else { return false; };
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
    assert!(
        matches!(base.vis, Visibility::Public(_)),
        "base field must be public"
    );
    let base_type = &base.ty;
    let target_type = input.ident;
    let as_base_crate = as_base_crate();
    traits
        .iter()
        .map(|(u, t)| {
            let unsafe_impl = if *u { quote!(unsafe) } else { quote!() };
            let trait_ident = format_ident!("{}", t);
            let tokens: proc_macro::TokenStream = quote! {
                #unsafe_impl impl #as_base_crate::#trait_ident<#base_type> for #target_type {}
            }
            .into();
            tokens
        })
        .collect()
}
