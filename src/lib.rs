use proc_macro::TokenStream;
use syn::*;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ..
        }) => named,
        _ => unimplemented!(),
    };

    let field_names = fields.iter().map(|f| &f.ident);
    let field_types = fields.iter().map(|f| &f.ty);

    let expanded = quote::quote! {
        impl #name {
            fn builder() -> #name {
                #name::default()
            }

            #(
                fn #field_names(&mut self, #field_names: #field_types) -> &mut Self {
                    self.#field_names = #field_names;
                    self
                }
            )*
        }
    };

    TokenStream::from(expanded)
}
