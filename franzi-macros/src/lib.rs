#![forbid(unsafe_code)]
#![warn(clippy::all)]
//TODO: Re-enable once you got the time to fix this
//#![warn(clippy::pedantic)]
//#![warn(clippy::cargo)]

extern crate proc_macro;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, quote_spanned};
use std::collections::HashMap;
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Generics, Index,
};

fn add_derives(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: TokenStream = input.into();
    let output = quote! {
        #[derive(Debug, Eq, PartialEq)]
        #input
    };
    output.into()
}

fn type_comment(typename: &str, input_str: &str) -> TokenStream {
    let typename = format!("{}\n", typename);
    let inputs = input_str.lines().map(ToString::to_string);
    quote! {
        #[doc = #typename]
        #[doc = "Generated from [Kafka Message Spec](http://kafka.apache.org/protocol.html) by the [`kafka_message!`](../../../franzi_macros/macro.kafka_message.html) macro.\n"]
        #[doc = "```ignore"]
        #(#[doc = #inputs])*
        #[doc = "```\n"]
    }
}

fn primitive_type(kafka_name: &str) -> Option<&'static str> {
    match kafka_name {
        "BOOLEAN" => Some("bool"),
        "INT8" => Some("i8"),
        "INT16" => Some("i16"),
        "INT32" => Some("i32"),
        "INT64" => Some("i64"),
        "UINT32" => Some("u32"),
        "VARINT" => Some("::franzi_base::types::vi32"),
        "VARLONG" => Some("::franzi_base::types::vi64"),
        "STRING" => Some("::franzi_base::types::KafkaString"),
        "NULLABLE_STRING" => Some("Option<::franzi_base::types::KafkaString>"),
        "BYTES" => Some("::bytes::Bytes"),
        "NULLABLE_BYTES" => Some("Option<::bytes::Bytes>"),
        // TODO: This does not work for invocations outside of franzi-proto crate...
        "RECORDS" => Some("franzi_proto::record::Records"),
        "ARRAY" => Some("Option<Vec>"),
        _ => None,
    }
}

fn parse_field(field_name: &str) -> (bool, &str) {
    if field_name.starts_with('[') {
        assert!(field_name.ends_with(']'));
        let field_name = &field_name[1..field_name.len() - 1];
        (true, field_name)
    } else {
        (false, field_name)
    }
}

fn to_rust_type(field2is_array: &HashMap<&str, bool>, name: &str, field_type: &str) -> String {
    let is_array = field2is_array
        .get(name)
        .unwrap_or_else(|| panic!("field2is_array[{:?}]", name));
    if *is_array {
        format!("Option<Vec<{}>>", field_type)
    } else {
        field_type.to_string()
    }
}

#[derive(Default)]
struct ParserState<'a> {
    field2comment: HashMap<&'a str, String>,
    field2is_array: HashMap<&'a str, bool>,
    last_field: Option<&'a str>,
}

impl<'a> ParserState<'a> {
    fn parse_comment_line(&mut self, line: &'a str) {
        let mut field_and_comment = line.trim().splitn(2, char::is_whitespace);
        let field = field_and_comment.next().expect("field name").trim();
        let comment = field_and_comment.next().expect("field comment").trim();
        if field == "Field" && comment == "Description" {
            return;
        }
        if !self.field2is_array.contains_key(&field) {
            // eprintln!("last field: {:?}", last_field);
            if let Some(last_field) = self.last_field {
                let last_value = self
                    .field2comment
                    .remove(&last_field)
                    .expect("expected last field to exist");
                self.field2comment
                    .insert(last_field, format!("{} {}", last_value, line));
            } else {
                panic!(
                    "Unexpected comment for field: {:?} comment: {:?}",
                    field, comment
                );
            }
        }
        self.field2comment.insert(field, comment.to_string());
        self.last_field = Some(field);
    }
}

/// Generates rust structs, including `#[derive(Debug, Eq, PartialEq, FromKafkaBytes, ToKafkaBytes)]`
/// for (potentially nested) [Kafka Protocol Message Spec](http://kafka.apache.org/protocol.html).
///
/// For kafka requests, the [`KafkaRequest`](../franzi_base/trait.KafkaRequest.html) trait is implemented as well.
///
/// Example:
/// ```ignore
///     ApiVersions Request (Version: 2) =>
/// ```
/// generates
/// ```
/// # use ::franzi_base::KafkaRequest;
/// # use ::franzi_macros::{FromKafkaBytes, ToKafkaBytes};
/// # #[derive(Debug, Eq, PartialEq, FromKafkaBytes, ToKafkaBytes)] pub struct ApiVersionsResponseV2{}
///     #[derive(Debug, Eq, PartialEq, FromKafkaBytes, ToKafkaBytes)]
///     pub struct ApiVersionsRequestV2{}
///
///     impl ::franzi_base::KafkaRequest for ApiVersionsRequestV2 {
///
///        type Response = ApiVersionsResponseV2;
///
///        fn api_key(&self) -> i16 {
///            ::franzi_base::api_keys::ApiKey::ApiVersions as i16
///        }
///
///        fn api_version(&self) -> i16 {
///            2i16
///        }
///    }
/// ```
/// Example:
/// ```ignore
///     ApiVersions Response (Version: 2) => error_code [api_versions] throttle_time_ms
///       error_code => INT16
///       api_versions => api_key min_version max_version
///         api_key => INT16
///         min_version => INT16
///         max_version => INT16
///       throttle_time_ms => INT32
/// ```
/// generates
/// ```
/// # use ::franzi_macros::{FromKafkaBytes, ToKafkaBytes};
///     #[derive(Debug, Eq, PartialEq, FromKafkaBytes, ToKafkaBytes)]
///     pub struct ApiVersionsResponseV2 {
///         pub error_code: i16,
///         pub api_versions: Option<Vec<ApiVersionsResponseV2_Versions>>,
///         pub throttle_time_ms: i32,
///     }
///
///     #[allow(non_camel_case_types)]
///     #[derive(Debug, Eq, PartialEq, FromKafkaBytes, ToKafkaBytes)]
///     pub struct ApiVersionsResponseV2_Versions {
///         pub api_key: i16,
///         pub min_version: i16,
///         pub max_version: i16,
///     }
/// ```
///
#[proc_macro]
pub fn kafka_message(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Note for the fellow developer: I'm really not proud of this code, this is
    // my first "real" proc_macro. If you know how to do this better, send a PR :)

    let mut tokens: Vec<proc_macro::TokenTree> = input.into_iter().collect();
    assert_eq!(tokens.len(), 1);
    let input_str = match tokens.remove(0) {
        proc_macro::TokenTree::Literal(s) => s,
        _ => panic!("Expected exactly one string literal"),
    };

    let input_str = input_str.to_string();
    let input_str = input_str[1..input_str.len() - 1].to_string();
    // eprintln!("input: {:?}", input_str);
    let mut input_lines = input_str.trim().split('\n');
    let current_line = input_lines.next().expect("First line");
    // eprintln!("current_line: {:?}", current_line);
    let mut split = current_line.split_whitespace();

    let api_name = split.next().expect("api name");
    let reqresp = split.next().expect("request or response");
    assert_eq!("(Version:", split.next().expect("version literal"));
    let version = split.next().expect("version number");
    assert_eq!(b')', version.as_bytes()[version.len() - 1]);
    let version = &version[0..version.len() - 1];
    let typename_str = format!("{}{}V{}", api_name, reqresp, version);
    // eprintln!("typename: {:?}", typename_str);

    assert_eq!("=>", split.next().unwrap());

    let mut state = ParserState::default();
    // remaining items on line are field names
    let mut field2type = HashMap::new();
    let mut type2fields = HashMap::new();
    {
        let mut fields = Vec::new();
        for field_name in split {
            let (is_arr, field_name) = parse_field(field_name);
            state.field2is_array.insert(field_name, is_arr);
            fields.push(field_name);
        }
        // eprintln!("fields: {:?}", fields);
        type2fields.insert(typename_str.clone(), fields);
    }

    let mut spec_lines = 1;
    let mut parsing_comments = false;
    for line in input_lines {
        if line.trim().is_empty() {
            // eprintln!("Got an empty line");
            parsing_comments = true;
        } else if parsing_comments {
            state.parse_comment_line(line);
        } else {
            spec_lines += 1;
            // remaining lines are in the form of fieldname => type or typename => fields
            let mut split = line.trim().split_whitespace();
            let name = split.next().expect("non-first line field name").trim();
            // eprintln!("Got field or type name {:?}", name);
            assert_eq!("=>", split.next().expect("non-first line arrow"));
            let fields_or_type: Vec<_> = split.collect();
            if fields_or_type.len() == 1 {
                // single field
                if let Some(primitive_type) = primitive_type(fields_or_type[0]) {
                    // eprintln!("field {:?} has primitive type {:?}", name, primitive_type);
                    // let rust_type = dbg!(to_rust_type(&state.field2is_array, name, primitive_type));
                    // Note: whether or not it is an array is handled further below...
                    let existing = field2type.insert(name.to_string(), primitive_type.to_string());
                    if let Some(existing) = existing {
                        assert_eq!(
                            existing, primitive_type,
                            "Expected identical type of field {:?} that appears twice",
                            name
                        );
                    }

                    // done for this lines
                    continue;
                }
            }

            let subtype_name = format!("{}_{}", typename_str, name);
            // not a primitive type -> Must be a list of fields
            let mut fields = Vec::new();
            for field_name in fields_or_type {
                let (is_arr, field_name) = parse_field(field_name);
                state.field2is_array.insert(field_name, is_arr);
                fields.push(field_name);
            }
            type2fields.insert(subtype_name.clone(), fields);
            let existing = field2type.insert(name.to_string(), subtype_name.clone());
            if let Some(existing) = existing {
                assert_eq!(
                    existing, subtype_name,
                    "Expected identical type of field {:?} that appears twice",
                    name
                );
            }
        }
    }

    let mut field_lines: HashMap<String, Vec<TokenStream>> = HashMap::new();
    for (typ, fields) in &type2fields {
        let mut lines = Vec::new();
        for field in fields {
            let field_type = &field2type
                .get(*field)
                .unwrap_or_else(|| panic!("field2type[{:?}]", field));
            let field_type = to_rust_type(&state.field2is_array, field, field_type);
            let comment = state
                .field2comment
                .get(*field)
                .unwrap_or_else(|| panic!("field2comment[{:?}]", field));
            lines.push(
                syn::parse_str(&format!(
                    "#[doc = {:?}] pub {}: {},",
                    comment, field, field_type
                ))
                .unwrap(),
            );
        }
        field_lines.insert(typ.clone(), lines);
    }

    let mut full_stream = proc_macro::TokenStream::new();

    // only use the "spec" lines (not the comment lines) for the type comment
    // otherwise, it's too ugly...
    let spec_str = input_str
        .lines()
        .take(spec_lines)
        .collect::<Vec<_>>()
        .join("\n");

    for (typename, lines) in field_lines {
        // eprintln!("Writing TokenStream for type {:?}", typename);
        let type_comment = type_comment(&typename, &spec_str);
        let typ = Ident::new(&typename, Span::call_site());
        let expanded = quote! {
            #type_comment
            pub struct #typ{
                #(#lines)*
            }
        };

        let ts = proc_macro::TokenStream::from(expanded);
        let d0 = derive_from_bytes(ts.clone());
        let d1 = derive_to_bytes(ts.clone());
        let ts = add_derives(ts);

        full_stream.extend(ts);
        full_stream.extend(d0);
        full_stream.extend(d1);
    }

    if reqresp == "Request" {
        // eprintln!("Is Request Version {} api_name {:?}", version, api_name);
        let typename = Ident::new(&typename_str, Span::call_site());
        let response_type = Ident::new(
            &format!("{}ResponseV{}", api_name, version),
            Span::call_site(),
        );
        let version: i16 = version.parse().expect("numberic version");
        let api_key = Ident::new(api_name, Span::call_site());

        let expanded = quote! {
            impl ::franzi_base::KafkaRequest for #typename {
                type Response = #response_type;

                fn api_key(&self) -> i16 {
                    ::franzi_base::api_keys::ApiKey::#api_key as i16
                }

                fn api_version(&self) -> i16 {
                    #version
                }
            }
        };
        full_stream.extend(proc_macro::TokenStream::from(expanded));
    }

    full_stream
}

/// Derives a default [`FromKafkaBytes`](../franzi_base/trait.FromKafkaBytes.html) implementation, whose `read()` method that calls `FromKafkaBytes::read()`
/// for each struct field.
///
/// Example:
/// ```
/// # use franzi_base::types::KafkaString;
/// # use ::franzi_macros::{FromKafkaBytes};
///     #[derive(FromKafkaBytes)]
///     pub struct RequestHeader {
///         pub api_key: i16,
///         pub api_version: i16,
///         pub correlation_id: i32,
///         pub client_id: Option<KafkaString>,
///     }
/// ```
/// generates
/// ```ignore
///     impl ::franzi_base::FromKafkaBytes for RequestHeader {
///         fn read(bytes: &mut ::std::io::Cursor<::bytes::Bytes>)
///             -> Result<Self, ::franzi_base::FromBytesError> {
///
///             Ok(RequestHeader{
///                 api_key: ::franzi_base::FromKafkaBytes::read(bytes)?,
///                 api_version: ::franzi_base::FromKafkaBytes::read(bytes)?,
///                 correlation_id: ::franzi_base::FromKafkaBytes::read(bytes)?,
///                 client_id: ::franzi_base::FromKafkaBytes::read(bytes)?,
///             })
///         }
///     }
/// ```
/// (Pretty simple, except for the "fully qualified" types which add some noise)
#[proc_macro_derive(FromKafkaBytes)]
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
        impl #impl_generics ::franzi_base::FromKafkaBytes for #name #ty_generics #where_clause {
            fn read(bytes: &mut ::bytes::Bytes) -> Result<Self, ::franzi_base::FromBytesError> {
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
                .push(parse_quote!(::franzi_base::FromKafkaBytes));
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
                    // ```ignore
                    // {
                    //     throttle_time_ms: FromKafkaBytes::read(bytes)?,
                    //     responses: FromKafkaBytes::read(bytes)?,
                    // }
                    // ```
                    // We take some care to use the span of each `syn::Field` as
                    // the span of the corresponding `read`
                    // call. This way if one of the field types does not
                    // implement `FromKafkaBytes` then the compiler's error message
                    // underlines which field it is. (Neat trick from HeapSize example)
                    let recurse = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        quote_spanned! {f.span()=>
                            #name: ::franzi_base::FromKafkaBytes::read(bytes)?,
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
                    //      FromKafkaBytes::read(bytes)?,
                    //      FromKafkaBytes::read(bytes)?
                    // )
                    let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                        let index = Index::from(i);
                        quote_spanned! {f.span()=>
                            ::franzi_base::FromKafkaBytes::read(&self.#index),
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

/// Derives a default [`ToKafkaBytes`](../franzi_base/trait.ToKafkaBytes.html) implementation. The `len_to_write()` method sums up `ToKafkaBytes::len_to_write()`
/// for each struct field; the `write()` calls `write()` on each struct field.
///
/// Example:
/// ```
/// # use ::franzi_base::types::KafkaString;
/// # use ::franzi_macros::ToKafkaBytes;
///     #[derive(ToKafkaBytes)]
///     pub struct RequestHeader {
///         pub api_key: i16,
///         pub api_version: i16,
///         pub correlation_id: i32,
///         pub client_id: Option<KafkaString>,
///     }
/// ```
/// generates
/// ```ignore
///     impl ::franzi_base::ToKafkaBytes for RequestHeader {
///
///         fn len_to_write(&self) -> usize {
///             0 +
///             ::franzi_base::ToKafkaBytes::len_to_write(&self.api_key) +
///             ::franzi_base::ToKafkaBytes::len_to_write(&self.api_version) +
///             ::franzi_base::ToKafkaBytes::len_to_write(&self.correlation_id) +
///             ::franzi_base::ToKafkaBytes::len_to_write(&self.client_id)
///         }
///
///         fn write(&self, bytes: &mut ::bytes::BufMut) {
///             ::franzi_base::ToKafkaBytes::write(&self.api_key, bytes);
///             ::franzi_base::ToKafkaBytes::write(&self.api_version, bytes);
///             ::franzi_base::ToKafkaBytes::write(&self.correlation_id, bytes);
///             ::franzi_base::ToKafkaBytes::write(&self.client_id, bytes);
///         }
///
///     }
/// ```
#[proc_macro_derive(ToKafkaBytes)]
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
        impl #impl_generics ::franzi_base::ToKafkaBytes for #name #ty_generics #where_clause {
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
                    //     throttle_time_ms: FromKafkaBytes::read(bytes)?,
                    //     responses: FromKafkaBytes::read(bytes)?,
                    // }
                    //
                    // We take some care to use the span of each `syn::Field` as
                    // the span of the corresponding `read`
                    // call. This way if one of the field types does not
                    // implement `FromKafkaBytes` then the compiler's error message
                    // underlines which field it is. (Neat trick from HeapSize example)
                    let recurse = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        quote_spanned! {f.span()=>
                            ::franzi_base::ToKafkaBytes::len_to_write(&self.#name)
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
                    //      FromKafkaBytes::read(bytes)?,
                    //      FromKafkaBytes::read(bytes)?
                    // )
                    let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                        let index = Index::from(i);
                        quote_spanned! {f.span()=>
                            ::franzi_base::FromKafkaBytes::read(&self.#index)
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
                    //     throttle_time_ms: FromKafkaBytes::read(bytes)?,
                    //     responses: FromKafkaBytes::read(bytes)?,
                    // }
                    //
                    // We take some care to use the span of each `syn::Field` as
                    // the span of the corresponding `read`
                    // call. This way if one of the field types does not
                    // implement `FromKafkaBytes` then the compiler's error message
                    // underlines which field it is. (Neat trick from HeapSize example)
                    let recurse = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        quote_spanned! {f.span()=>
                            ::franzi_base::ToKafkaBytes::write(&self.#name, bytes);
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
                    //      FromKafkaBytes::read(bytes)?,
                    //      FromKafkaBytes::read(bytes)?
                    // )
                    let recurse = fields.unnamed.iter().enumerate().map(|(i, f)| {
                        let index = Index::from(i);
                        quote_spanned! {f.span()=>
                            ::franzi_base::ToKafkaBytes::write(&self.#index, bytes);
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
