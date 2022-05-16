use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident, Type};

pub fn expand_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let mut types = Vec::new();
    let mut idents = Vec::new();

    match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                for field in fields.named.iter() {
                    types.push(field.ty.clone());
                    idents.push(field.ident.clone().unwrap());
                }
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }

    let storedata = expand_storedata_impl(&input.ident, &idents, &types);
    let descriptor = expand_datadescriptor_impl(&input.ident, &idents, &types);
    let query = expand_dataquery_impl(&input.ident, &idents, &types);

    let expanded = quote! {
        #storedata
        #descriptor
        #query
    };

    proc_macro::TokenStream::from(expanded)
}

fn expand_storedata_impl(ident: &Ident, idents: &[Ident], types: &[Type]) -> TokenStream {
    let trait_bounds = expand_trait_bounds(types);

    let write_impl = idents.iter().map(|ident| {
        let name = ident.to_string();

        quote! {
            writer.write_field(#name, &self.#ident)?;
        }
    });

    let read_impl = idents.iter().map(|ident| {
        let name = ident.to_string();

        quote! {
            let #ident = reader.read_field(#name)?;
        }
    });

    let descriptor_ident = Ident::new(&format!("{}Descriptor", ident), Span::call_site());
    let query_ident = Ident::new(&format!("{}Query", ident), Span::call_site());

    quote! {
        impl<T> ::datastore::StoreData<T> for #ident
        where
            T: ::datastore::Store,
            #trait_bounds
        {
            type Descriptor = #descriptor_ident;
            type Query = #query_ident;

            fn write<W>(&self, writer: &mut W) -> ::std::result::Result<(), W::Error>
            where
                W: ::datastore::Writer<T>,
            {
                #(#write_impl)*

                ::std::result::Result::Ok(())
            }

            fn read<R>(reader: &mut R) -> ::std::result::Result<Self, R::Error>
            where
                R: ::datastore::Reader<T>
            {
                #(#read_impl)*

                Ok(Self {
                    #(#idents,)*
                })
            }
        }
    }
}

fn expand_datadescriptor_impl(ident: &Ident, idents: &[Ident], types: &[Type]) -> TokenStream {
    let trait_bounds = expand_trait_bounds(types);

    let datadescriptor_ident = Ident::new(&format!("{}Descriptor", ident), Span::call_site());

    let write_impl = idents.iter().zip(types).map(|(ident, ty)| {
        let name = ident.to_string();
        let ty = ty.clone();

        quote! {
            writer.write_field::<#ty>(#name)?;
        }
    });

    let name = ident.to_string();

    quote! {
        #[derive(Copy, Clone, Debug, Default)]
        pub struct #datadescriptor_ident;

        impl<T> ::datastore::DataDescriptor<#ident, T> for #datadescriptor_ident
        where
            T: ::datastore::Store,
            #trait_bounds
        {
            fn name(&self) -> &str {
                #name
            }

            fn write<W>(&self, writer: &mut W) -> ::std::result::Result<(), W::Error>
            where
                W: ::datastore::TypeWriter<T>
            {
                #(#write_impl)*

                ::std::result::Result::Ok(())
            }
        }
    }
}

fn expand_dataquery_impl(ident: &Ident, idents: &[Ident], types: &[Type]) -> TokenStream {
    let trait_bounds = expand_trait_bounds(types);

    let dataquery_ident = Ident::new(&format!("{}Query", ident), Span::call_site());

    let dataquery_fields = idents.iter().zip(types.iter()).map(|(ident, ty)| {
        quote! {
            #ident: Option<#ty>,
        }
    });

    let dataquery_methods = idents.iter().zip(types.iter()).map(|(ident, ty)| {
        quote! {
            pub fn #ident(mut self, t: #ty) -> Self {
                self.#ident = ::std::option::Option::Some(t);
                self
            }
        }
    });

    let write_impl = idents.iter().map(|ident| {
        let name = ident.to_string();

        quote! {
            if let Some(value) = self.#ident.as_ref() {
                writer.write_field(#name, value)?;
            }
        }
    });

    quote! {
        #[derive(Clone, Default)]
        pub struct #dataquery_ident {
            #(#dataquery_fields)*
        }

        impl #dataquery_ident {
            #(#dataquery_methods)*
        }

        impl<T> ::datastore::DataQuery<#ident, T> for #dataquery_ident
        where
            T: ::datastore::Store,
            #trait_bounds
        {
            fn write<W>(&self, writer: &mut W) -> ::std::result::Result<(), W::Error>
            where
                W: ::datastore::Writer<T>,
            {
                #(#write_impl)*

                ::std::result::Result::Ok(())
            }
        }
    }
}

fn expand_trait_bounds(types: &[Type]) -> TokenStream {
    let mut bounds = Vec::new();
    for ty in types {
        if !bounds.contains(ty) {
            bounds.push(ty.clone());
        }
    }

    quote! {
        #(
            #bounds: ::datastore::Write<T> + ::datastore::Read<T>,
        )*
    }
}
