use darling::FromDeriveInput;
use proc_macro2::TokenTree;
use quote::quote;
use syn::{Data, DeriveInput, Field, Fields, Ident, Meta, Type, ext::IdentExt, parse_macro_input};

#[derive(FromDeriveInput, Debug)]
#[darling(attributes(medici))]
struct InsertableOpts {
    pub table_struct: String,
}

#[proc_macro_derive(Insertable, attributes(medici))]
pub fn derive_insertable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let opts = match InsertableOpts::from_derive_input(&derive_input) {
        Ok(opts) => opts,
        Err(error) => return error.write_errors().into(),
    };

    let name = derive_input.ident;
    let field_idents = struct_field_idents(derive_input.data);
    let field_idents_to_stringify = field_idents.iter().map(|field| field.unraw());
    let number_of_fields = field_idents.len();

    let table_struct = parse_table_struct(opts.table_struct);

    quote! {
        #[::async_trait::async_trait]
        #[automatically_derived]
        impl Insertable<#number_of_fields> for #name {
            type T = #table_struct;

            const COLUMNS: [&'static str; #number_of_fields] =
                [#(stringify!(#field_idents_to_stringify)),*];

            fn bind(
                self,
                separated: &mut ::sqlx::query_builder::Separated<'_, '_, ::sqlx::Postgres, &'static str>
            ) {
                #(separated.push_bind(self.#field_idents);)*
            }
        }
    }.into()
}

#[derive(FromDeriveInput, Debug)]
#[darling(attributes(medici))]
struct ChangesetOpts {
    pub table_struct: String,
}

#[proc_macro_derive(Changeset, attributes(medici))]
pub fn derive_changeset(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let opts = match ChangesetOpts::from_derive_input(&derive_input) {
        Ok(opts) => opts,
        Err(error) => return error.write_errors().into(),
    };

    let name = derive_input.ident;
    let fields = struct_field_idents(derive_input.data);
    let fields_to_stringify = fields.iter().map(|field| field.unraw());
    let number_of_fields = fields.len();

    let table_struct = parse_table_struct(opts.table_struct);

    quote! {
        #[::async_trait::async_trait]
        #[automatically_derived]
        impl Changeset<#number_of_fields> for #name {
            type T = #table_struct;

            const COLUMNS: [&'static str; #number_of_fields] =
                [#(stringify!(#fields_to_stringify)),*];

            fn bind(
                self,
                separated: &mut ::sqlx::query_builder::Separated<'_, '_, ::sqlx::Postgres, &'static str>
            ) {
                #(if let ::std::option::Option::Some(value) = self.#fields {
                    separated.push(format!("\"{}\" = ", stringify!(#fields)));
                    separated.push_bind_unseparated(value);
                })*
            }
        }


        #[automatically_derived]
        impl ::std::cmp::PartialEq<#table_struct> for #name {
            fn eq(&self, other: &#table_struct) -> bool {
                #(if let ::std::option::Option::Some(ref value) = self.#fields && value != &other.#fields {
                    return false;
                })*

                true
            }
        }

        #[automatically_derived]
        impl ::std::cmp::PartialEq<#name> for #table_struct {
            fn eq(&self, other: &#name) -> bool {
                other == self
            }
        }
    }.into()
}

fn filtered_struct_fields<T>(
    derive_input_data: Data,
    filter_map_f: impl Fn(Field) -> Option<T>,
) -> Vec<T> {
    match derive_input_data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields_named) => fields_named
                .named
                .into_iter()
                .filter_map(filter_map_f)
                .collect(),
            _ => panic!("macro supported only on structs with named fields"),
        },
        _ => panic!("macro supported only on structs with named fields"),
    }
}

fn struct_field_idents(derive_input_data: Data) -> Vec<Ident> {
    filtered_struct_fields(derive_input_data, |field| {
        let ident = field.ident.unwrap();

        if field.attrs.iter().any(|attr| {
            let Meta::List(meta_list) = &attr.meta else {
                return false;
            };

            meta_list.tokens.clone().into_iter().any(|token| {
                let TokenTree::Ident(ident) = token else {
                    return false;
                };

                ident == "skip"
            })
        }) {
            return None;
        }

        Some(ident)
    })
}

fn parse_table_struct(table_struct: String) -> Type {
    syn::parse_str::<syn::Type>(&table_struct).unwrap()
}

#[derive(FromDeriveInput, Debug)]
#[darling(attributes(medici))]
struct TableOpts {
    pub table_name: String,
}

#[proc_macro_derive(Table, attributes(medici))]
pub fn derive_table(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let opts = match TableOpts::from_derive_input(&derive_input) {
        Ok(opts) => opts,
        Err(error) => return error.write_errors().into(),
    };

    let name = derive_input.ident;
    let fields = filtered_struct_fields(derive_input.data, Some);
    let primary_key_field = fields
        .iter()
        .find(|field| {
            field.attrs.iter().any(|attr| {
                let Meta::List(meta_list) = &attr.meta else {
                    return false;
                };

                meta_list.tokens.clone().into_iter().any(|token| {
                    let TokenTree::Ident(ident) = token else {
                        return false;
                    };

                    ident == "primary_key"
                })
            })
        })
        .expect("table struct should have a primary key field");

    let table_name = opts.table_name;
    let primary_key_type = primary_key_field.ty.clone();
    let primary_key_ident = primary_key_field.ident.clone().unwrap();

    quote! {
        #[automatically_derived]
        impl Table for #name {
            type PrimaryKey = #primary_key_type;

            const TABLE_NAME: &'static str = #table_name;
            const PRIMARY_KEY_COLUMN: &'static str = stringify!(#primary_key_ident);

            fn primary_key(&self) -> &Self::PrimaryKey {
                &self.#primary_key_ident
            }
        }
    }
    .into()
}

#[proc_macro_derive(ValkeyString, attributes(medici))]
pub fn derive_valkey_string(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let name = derive_input.ident;

    let expanded = quote! {
        #[automatically_derived]
        impl ::fred::types::FromValue for #name {
            fn from_value(
                value: ::fred::types::Value
            ) -> ::std::result::Result<Self, ::fred::error::Error> {
                let json_value = value.into_json().unwrap();

                ::std::result::Result::Ok(::serde_json::from_value(json_value).unwrap())
            }
        }

        #[automatically_derived]
        impl ::std::convert::From<#name> for ::fred::types::Value {
            fn from(value: #name) -> Self {
                ::serde_json::to_string(&value).unwrap().into()
            }
        }
    };

    expanded.into()
}

#[proc_macro_derive(Hashable, attributes(medici))]
pub fn derive_hashable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();

    let hash_field_ident = syn::parse_str::<Ident>("hash").unwrap();

    let name = derive_input.ident;
    let fields = filtered_struct_fields(derive_input.data, |field| {
        let ident = field.ident.unwrap();

        if ident == hash_field_ident
            || field.attrs.iter().any(|attr| {
                let Meta::List(meta_list) = &attr.meta else {
                    return false;
                };

                meta_list.tokens.clone().into_iter().any(|token| {
                    let TokenTree::Ident(ident) = token else {
                        return false;
                    };

                    ident == "skip_hash"
                })
            })
        {
            return None;
        }

        Some(ident)
    });

    quote! {
        #[automatically_derived]
        impl #impl_generics Hashable for #name #ty_generics #where_clause {
            fn to_bytes(&self) -> ::std::vec::Vec<::std::primitive::u8> {
                let mut bytes = ::std::vec![];

                #(
                    ::std::iter::Extend::extend(
                        &mut bytes,
                        ::core::primitive::str::as_bytes(stringify!(#fields))
                    );
                    ::std::iter::Extend::extend(
                        &mut bytes,
                        Hashable::to_bytes(&self.#fields)
                    );
                )*

                bytes
            }

            fn stored_hash(&self) -> ::core::option::Option<&::std::primitive::str> {
                if ::std::string::String::is_empty(&self.#hash_field_ident) {
                    ::core::option::Option::None
                } else {
                    ::core::option::Option::Some(&self.#hash_field_ident)
                }
            }

            fn set_hash(&mut self, hash: ::std::string::String) {
                self.#hash_field_ident = hash;
            }
        }
    }
    .into()
}
