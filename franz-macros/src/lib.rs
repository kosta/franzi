extern crate proc_macro;

// use proc_macro::TokenTree;
use proc_macro2::{TokenStream, Span};
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Generics, Index};

fn add_derives(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: TokenStream = input.into();
    let output = quote! {
        #[derive(Debug, Eq, PartialEq)]
        #input
    };
    output.into()
}

/// Unimplemented at the moment
#[proc_macro]
pub fn message(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut tokens: Vec<proc_macro::TokenTree> = input.into_iter().collect();
    assert_eq!(tokens.len(), 1);
    let input_str = match tokens.remove(0) {
        proc_macro::TokenTree::Literal(s) => s,
        _ => panic!("Expected exactly one string literal"),
    };

    let input_str = input_str.to_string();
    let input_str = input_str[1..input_str.len()-1].to_string();

    // eprintln!("input: {:?}", input_str);

    let name = syn::Ident::new(&input_str, Span::call_site());

    let expanded = quote! {
        pub struct #name{}
    };
    let mut ts = proc_macro::TokenStream::from(expanded);

    let d0 = derive_from_bytes(ts.clone());
    let d1 = derive_to_bytes(ts.clone());
    ts.extend(d0);
    ts.extend(d1);

    add_derives(ts)
}

#[proc_macro_derive(FromBytes)]
pub fn derive_from_bytes(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;

    // Add a bound `T: HeapSize` to every type parameter T.
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Generate an expression to generate impl using each field.
    let imp = from_bytes_impl(&input.data);

    let expanded = quote! {
        // The generated impl.
        impl #impl_generics ::franz_base::FromBytes for #name #ty_generics #where_clause {
            fn read(bytes: &mut ::std::io::Cursor<::bytes::Bytes>) -> Result<Self, ::franz_base::FromBytesError> {
                Ok(#name
                    #imp
                )
            }
        }
    };

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}

// Add a bound `T: HeapSize` to every type parameter T.
fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(::franz_base::FromBytes));
        }
    }
    generics
}

// Generate an expression to sum up the heap size of each field.
fn from_bytes_impl(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    // Expands to an expression like
                    //
                    // {
                    //     throttle_time_ms: FromBytes::read(bytes)?,
                    //     responses: FromBytes::read(bytes)?,
                    // }
                    //
                    // We take some care to use the span of each `syn::Field` as
                    // the span of the corresponding `read`
                    // call. This way if one of the field types does not
                    // implement `FromBytes` then the compiler's error message
                    // underlines which field it is. (Neat trick from HeapSize example)
                    let recurse = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        quote_spanned! {f.span()=>
                            #name: ::franz_base::FromBytes::read(bytes)?,
                        }
                    });
                    quote! {
                        {
                            #(
                                #recurse
                            )*
                        }
                    }
                }
                Fields::Unnamed(ref fields) => {
                    // Expands to an expression like
                    //
                    // (
                    //      FromBytes::read(bytes)?,
                    //      FromBytes::read(bytes)?
                    // )
                    let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                        let index = Index::from(i);
                        quote_spanned! {f.span()=>
                            ::franz_base::FromBytes::read(&self.#index),
                        }
                    });
                    quote! {
                        (
                            #(#recurse)*
                        )
                    }
                }
                Fields::Unit => {
                    // Unit structs cannot own more than 0 bytes of heap memory.
                    quote!()
                }
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

#[proc_macro_derive(ToBytes)]
pub fn derive_to_bytes(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;

    // Add a bound `T: HeapSize` to every type parameter T.
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Generate an expression to generate impl using each field.
    let len_impl = to_bytes_len_impl(&input.data);
    let write_impl = to_bytes_write_impl(&input.data);

    let expanded = quote! {
        // The generated impl.
        impl #impl_generics ::franz_base::ToBytes for #name #ty_generics #where_clause {
                fn len_to_write(&self) -> usize {
                    #len_impl
                }

                fn write(&self, bytes: &mut ::bytes::BufMut) {
                    #write_impl
                }
        }
    };

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}

// Generate an expression to sum up the heap size of each field.
fn to_bytes_len_impl(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    // Expands to an expression like
                    //
                    // {
                    //     throttle_time_ms: FromBytes::read(bytes)?,
                    //     responses: FromBytes::read(bytes)?,
                    // }
                    //
                    // We take some care to use the span of each `syn::Field` as
                    // the span of the corresponding `read`
                    // call. This way if one of the field types does not
                    // implement `FromBytes` then the compiler's error message
                    // underlines which field it is. (Neat trick from HeapSize example)
                    let recurse = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        quote_spanned! {f.span()=>
                            ::franz_base::ToBytes::len_to_write(&self.#name)
                        }
                    });
                    quote! {
                        0 #(+ #recurse)*
                    }
                }
                Fields::Unnamed(ref fields) => {
                    // Expands to an expression like
                    //
                    // (
                    //      FromBytes::read(bytes)?,
                    //      FromBytes::read(bytes)?
                    // )
                    let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                        let index = Index::from(i);
                        quote_spanned! {f.span()=>
                            ::franz_base::FromBytes::read(&self.#index)
                        }
                    });
                    quote! {
                        0 #(+ #recurse)*
                    }
                }
                Fields::Unit => {
                    // Unit structs cannot own more than 0 bytes of heap memory.
                    quote!(0)
                }
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

// Generate an expression to sum up the heap size of each field.
fn to_bytes_write_impl(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    // Expands to an expression like
                    //
                    // {
                    //     throttle_time_ms: FromBytes::read(bytes)?,
                    //     responses: FromBytes::read(bytes)?,
                    // }
                    //
                    // We take some care to use the span of each `syn::Field` as
                    // the span of the corresponding `read`
                    // call. This way if one of the field types does not
                    // implement `FromBytes` then the compiler's error message
                    // underlines which field it is. (Neat trick from HeapSize example)
                    let recurse = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        quote_spanned! {f.span()=>
                            ::franz_base::ToBytes::write(&self.#name, bytes);
                        }
                    });
                    quote! {
                        #(
                            #recurse
                        )*
                    }
                }
                Fields::Unnamed(ref fields) => {
                    // Expands to an expression like
                    //
                    // (
                    //      FromBytes::read(bytes)?,
                    //      FromBytes::read(bytes)?
                    // )
                    let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                        let index = Index::from(i);
                        quote_spanned! {f.span()=>
                            ::franz_base::ToBytes::write(&self.#index, bytes);
                        }
                    });
                    quote! {
                        #(
                            #recurse
                        )*
                    }
                }
                Fields::Unit => {
                    // Unit structs cannot own more than 0 bytes of heap memory.
                    quote!()
                }
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}