use darling::{ast, util, FromDeriveInput, FromField};
use proc_macro::TokenStream;
use quote::quote;
use syn::*;

#[derive(Debug, FromField)]
#[darling(attributes(bbgun))]
struct BuilderField {
    ident: Option<Ident>,
    ty: Type,
    #[darling(default)]
    transform: Option<Path>,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(bbgun), supports(struct_named))]
struct Builder {
    ident: Ident,
    data: ast::Data<util::Ignored, BuilderField>,
    #[darling(default)]
    build_func: Option<Path>,
    #[darling(default)]
    builds_to: Option<Path>,
}

#[proc_macro_derive(Builder, attributes(bbgun))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let builder = Builder::from_derive_input(&input).unwrap();
    let name = &builder.ident;

    let fields = match builder.data {
        ast::Data::Struct(fields) => fields,
        _ => panic!("Only structs are supported"),
    };

    let field_setters = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        let subsetter = if let Some(transform) = &field.transform {
            quote! {
                #transform(#field_name)
            }
        } else {
            quote! {
                #field_name
            }
        };

        quote! {
            fn #field_name(&mut self, #field_name: #field_type) -> &mut Self {
                self.#field_name = #subsetter;
                self
            }
        }
    });

    let build_func = if builder.build_func.is_some() && builder.builds_to.is_some() {
        let bf = builder.build_func.as_ref().unwrap();
        let bt = builder.builds_to.as_ref().unwrap();
        quote! {
            fn build(&self) -> #bt {
                    #bf(self)
            }
        }
    } else {
        quote! {}
    };

    TokenStream::from(quote! {
        #[automatically_derived]
        impl #name {
            fn builder() -> #name {
                #name::default()
            }

            #build_func

            #(#field_setters)*
        }
    })
}
