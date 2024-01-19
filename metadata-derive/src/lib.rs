use std::collections::HashMap;

use syn::DeriveInput;

#[proc_macro_derive(MetaData, attributes(metadata))]
pub fn metadata_derive_macro(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    metadata_derive_macro2(item.into()).unwrap().into()
}

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(metadata))]
struct MetaDataStructAttributes {
    author: String,
    #[deluxe(default = 0)]
    serial_version: usize,
}

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(metadata))]
struct MetaDataFieldAttributes {
    author: String,
}

fn metadata_derive_macro2(
    item: proc_macro2::TokenStream,
) -> deluxe::Result<proc_macro2::TokenStream> {
    // parse
    let mut ast: DeriveInput = syn::parse2(item)?;

    // extract struct attributes
    let MetaDataStructAttributes {
        author,
        serial_version,
    } = deluxe::extract_attributes(&mut ast)?;

    // extract field attributes
    let field_attrs = extract_metadata_field_attrs(&mut ast)?;
    let (field_names, field_authors): (Vec<String>, Vec<String>) = field_attrs
        .into_iter()
        .map(|(field, attrs)| (field, attrs.author))
        .unzip();

    // define impl variables
    let ident = &ast.ident;
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();

    // generate
    Ok(quote::quote! {
        impl #impl_generics MetaData for #ident #type_generics #where_clause {
            fn author(&self) -> &'static str {
                #author
            }

            fn serial_version(&self) -> usize {
                #serial_version
            }

            fn fields_authors(&self) -> std::collections::HashMap<&'static str, &'static str> {
                let fields = [#(#field_names),*];
                let authors = [#(#field_authors),*];

                fields.into_iter().zip(authors.into_iter()).collect()
            }
        }
    })
}

fn extract_metadata_field_attrs(
    ast: &mut DeriveInput,
) -> deluxe::Result<HashMap<String, MetaDataFieldAttributes>> {
    let mut field_attrs = HashMap::new();

    if let syn::Data::Struct(s) = &mut ast.data {
        for field in s.fields.iter_mut() {
            if let Ok(attrs) = deluxe::extract_attributes(field) {
                let field_name = field.ident.as_ref().map(ToString::to_string).unwrap();
                field_attrs.insert(field_name, attrs);
            };
        }
    }

    Ok(field_attrs)
}
