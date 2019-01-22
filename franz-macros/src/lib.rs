extern crate proc_macro;

// use proc_macro::TokenTree;
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Generics, Index,
};
use itertools::Itertools;

fn add_derives(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: TokenStream = input.into();
    let output = quote! {
        #[derive(Debug, Eq, PartialEq)]
        #input
    };
    output.into()
}

fn type_comment(typename: &str, input_str: &str) -> TokenStream{
    let typename = format!("{}\n", typename);
    let inputs = input_str.lines().map(|s| format!("{}", s));
    quote!{
        #[doc = #typename]
        #[doc = "Generated from [Kafka Message Spec](http://kafka.apache.org/protocol.html) by the [`kafka_message!`](../../../franz_macros/macro.kafka_message.html) macro.\n"]
        #[doc = "```ignore"]
        #(#[doc = #inputs])*
        #[doc = "```\n"]
    }
}

/// Generates rust structs, including `#[derive(Debug, Eq, PartialEq, FromBytes, ToBytes)]`
/// for (potentially nested) [Kafka Protocol Message Spec](http://kafka.apache.org/protocol.html).
///
/// Example:
///
///     ApiVersions Request (Version: 2) =>
///
/// generates
///
///     #[derive(Debug, Eq, PartialEq, FromBytes, ToBytes)]
///     pub struct ApiVersionsRequestV2{}
///
/// Example:
///
///     ApiVersions Response (Version: 2) => error_code [api_versions] throttle_time_ms
///       error_code => INT16
///       api_versions => api_key min_version max_version
///         api_key => INT16
///         min_version => INT16
///         max_version => INT16
///       throttle_time_ms => INT32
///
/// generates
///
///     #[derive(Debug, Eq, PartialEq, FromBytes, ToBytes)]
///     pub struct ApiVersionsResponseV2 {
///         pub error_code: i16,
///         pub api_versions: Option<Vec<ApiVersionsResponseV2_Versions>>,
///         pub throttle_time_ms: i32,
///     }
///
///     #[allow(non_camel_case_types)]
///     #[derive(Debug, Eq, PartialEq, FromBytes, ToBytes)]
///     pub struct ApiVersionsResponse2_Versions {
///         pub api_key: i16,
///         pub min_version: i16,
///         pub max_version: i16,
///     }
///
///
#[proc_macro]
pub fn kafka_message(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut tokens: Vec<proc_macro::TokenTree> = input.into_iter().collect();
    assert_eq!(tokens.len(), 1);
    let input_str = match tokens.remove(0) {
        proc_macro::TokenTree::Literal(s) => s,
        _ => panic!("Expected exactly one string literal"),
    };

    let input_str = input_str.to_string();
    let input_str = input_str[1..input_str.len() - 1].to_string();
    eprintln!("input: {:?}", input_str);
    let mut split = input_str.split(' ');

    let api_name = split.next().expect("api name");
    let reqresp = split.next().expect("request or response");
    assert_eq!("(Version:", split.next().expect("version literal"));
    let version = split.next().expect("version number");
    assert_eq!(b')', version.as_bytes()[version.len() - 1]);
    let version = &version[0..version.len() - 1];
    let typename_str = format!("{}{}V{}", api_name, reqresp, version);
    eprintln!("typename: {:?}", typename_str);
    let typename = syn::Ident::new(&typename_str, Span::call_site());

    assert_eq!("=>", split.next().unwrap());

    let type_comment = type_comment(&typename_str, &input_str);
    // eprintln!("type_comment: {:?}", type_comment);

    let expanded = quote! {
        #type_comment
        pub struct #typename{}
    };
    let mut ts = proc_macro::TokenStream::from(expanded);

    let d0 = derive_from_bytes(ts.clone());
    let d1 = derive_to_bytes(ts.clone());
    ts.extend(d0);
    ts.extend(d1);

    add_derives(ts)
}

/// Derives a default [`FromBytes`](../franz_base/trait.FromBytes.html) implementation, whose `read()` method that calls `FromBytes::read()`
/// for each struct field.
///
/// Example:
///
///     #[derive(FromBytes)]
///     pub struct RequestHeader {
///         pub api_key: i16,
///         pub api_version: i16,
///         pub correlation_id: i32,
///         pub client_id: Option<KafkaString>,
///     }
///
/// generates
///
///     impl ::franz_base::FromBytes for RequestHeader {
///         fn read(bytes: &mut ::std::io::Cursor<::bytes::Bytes>)
///             -> Result<Self, ::franz_base::FromBytesError> {
///
///             Ok(RequestHeader{
///                 api_key: ::franz_base::FromBytes::read(bytes)?,
///                 api_version: ::franz_base::FromBytes::read(bytes)?,
///                 correlation_id: ::franz_base::FromBytes::read(bytes)?,
///                 client_id: ::franz_base::FromBytes::read(bytes)?,
///             })
///         }
///     }
///
/// (Pretty simple, except for the "fully qualified" types which add some noise)
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
            type_param
                .bounds
                .push(parse_quote!(::franz_base::FromBytes));
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

/// Derives a default [`ToBytes`](../franz_base/trait.ToBytes.html) implementation. The `len_to_write()` method sums up `ToBytes::len_to_write()`
/// for each struct field; the `write()` calls `write()` on each struct field.
///
/// Example:
///
///     #[derive(ToBytes)]
///     pub struct RequestHeader {
///         pub api_key: i16,
///         pub api_version: i16,
///         pub correlation_id: i32,
///         pub client_id: Option<KafkaString>,
///     }
///
/// generates
///
///     impl ::franz_base::ToBytes for RequestHeader {
///
///         fn len_to_write(&self) -> usize {
///             0 +
///             ::franz_base::ToBytes::len_to_write(&self.api_key) +
///             ::franz_base::ToBytes::len_to_write(&self.api_version) +
///             ::franz_base::ToBytes::len_to_write(&self.correlation_id) +
///             ::franz_base::ToBytes::len_to_write(&self.client_id)
///         }
///
///         fn write(&self, bytes: &mut ::bytes::BufMut) {
///             ::franz_base::ToBytes::write(&self.api_key, bytes);
///             ::franz_base::ToBytes::write(&self.api_version, bytes);
///             ::franz_base::ToBytes::write(&self.correlation_id, bytes);
///             ::franz_base::ToBytes::write(&self.client_id, bytes);
///         }
///
///     }

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
