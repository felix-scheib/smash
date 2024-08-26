use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput, Fields};
use quote::quote;

#[proc_macro_derive(Implement)]
pub fn derive_answer_fn(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);

    let name = &input.ident;


    let init = implement_init(&input);
    let map = implement_get_slot(&input);
    let getter = implement_getter(&input);

    let expanded = quote! {
        impl #name {
            #init
            #map
            #getter
        }
    };

    TokenStream::from(expanded)
}

fn implement_init(input: &DeriveInput) -> proc_macro2::TokenStream {
    let fields = if let Data::Struct(ref data_struct) = input.data {
        if let Fields::Named(ref fields) = data_struct.fields {
            fields.named.iter().enumerate().map(|(i, f)| {
                let field_name = &f.ident;
                let handle = i + 1;
                quote! {
                    #field_name: std::sync::Arc::new(super::slot::Slot::new(
                        Default::default(),
                        #handle,
                        std::sync::Weak::clone(&shared_memory),
                    )),
                }
            }).collect::<Vec<_>>()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    quote! {
        pub fn init(shared_memory: &std::sync::Weak<super::SharedMemory>) -> Self {
            Self {
                #(#fields)*
            }
        }
    }
}

fn implement_get_slot(input: &DeriveInput) -> proc_macro2::TokenStream {
    let fields = if let Data::Struct(ref data_struct) = input.data {
        if let Fields::Named(ref fields) = data_struct.fields {
            fields.named.iter().enumerate().map(|(i, f)| {
                let field_name = &f.ident;
                let handle = i + 1;
                quote! {
                    #handle => Some(super::as_trait(std::sync::Arc::clone(&self.#field_name))),
                }
            }).collect::<Vec<_>>()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };


    quote! {
        pub fn get_slot(&self, handle: usize) -> Option<std::sync::Arc<dyn super::IncommingObserver>> {
            match handle {
                #(#fields)*
                _ => None,
            }
        }
    }
}

fn implement_getter(input: &DeriveInput) -> proc_macro2::TokenStream {
    let getters = if let Data::Struct(ref data_struct) = input.data {
        if let Fields::Named(ref fields) = data_struct.fields {
            fields.named.iter().map(|f| {
                let field_name = &f.ident;
                let field_type = &f.ty;

                quote! {
                    pub fn #field_name(&self) -> #field_type {
                        std::sync::Arc::clone(&self.#field_name)
                    }
                }
            }).collect::<Vec<_>>()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    quote! {
        #(#getters)*
    }
}