use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_derive(Access)]
pub fn derive_access(input: TokenStream) -> TokenStream {
    let syn::DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let fields: Vec<proc_macro2::TokenStream> = match data {
        syn::Data::Struct(st) => match st.fields {
            syn::Fields::Named(syn::FieldsNamed { named, .. }) => {
                named.iter().filter(|field| {
                    field.ident.is_some()
                }).map(|field| {
                    let ident = &field.ident;
                    let mut ty = field.ty.clone();
                    // Yeet out path.segments.arguments (ie. type arguments), as these need to be
                    // absent in instantiations:
                    //   struct Foo<'a, T> {
                    //      bar: baz<'a, T>,
                    //   }
                    // but:
                    //   Foo {
                    //      bar: baz::new(),
                    //   }
                    // Ugh.
                    match ty {
                        syn::Type::Path(syn::TypePath { ref mut path, .. }) => {
                            let segments = path.segments.clone().iter().map(|seg| {
                                let mut seg = seg.clone();
                                seg.arguments = syn::PathArguments::None;
                                seg
                            }).collect();
                            path.segments = segments;
                        },
                        _ => (),
                    };
                    quote! {
                        #ident : #ty::fetch(world)
                    }
                }).collect()
            },
            _ => {
                panic!("can only derive Access on structs with named fields");
            },
        }
        _ => panic!("can only derive Access on structs"),
    };

    let output = quote! {
        impl <'a> ecs::system::Access<'a> for #ident<'a> {
            fn fetch(world: &'a ecs::World) -> Self {
                Self {
                    #( #fields, )*
                }
            }
        }
    };

    let tokens = output.into();
    //eprintln!("TOKENS: {}", tokens);
    tokens
}
