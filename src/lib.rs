use proc_macro::TokenStream;
use quote::quote;

fn impl_expose(input_stream: TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let item_impl = syn::parse::<syn::ItemImpl>(input_stream)?;
    let mut implementations = Vec::new();

    let self_ty = &item_impl.self_ty;
    let trait_name = match item_impl.trait_ {
        Some((_, ref path, _)) => path,
        None => {
            return Err(syn::Error::new(
                item_impl.impl_token.span,
                "Impl must be for a trait",
            ))
        }
    };

    for item in item_impl.items.iter() {
        implementations.push(match *item {
            syn::ImplItem::Const(ref impl_const) => {
                let name = &impl_const.ident;
                let ty = &impl_const.ty;
                quote! {
                    pub const #name: #ty = <#self_ty as #trait_name>::#name;
                }
            }
            syn::ImplItem::Type(ref impl_type) => {
                let name = &impl_type.ident;
                quote! {
                    pub type #name = <#self_ty as #trait_name>::#name;
                }
            }
            syn::ImplItem::Method(ref impl_method) => {
                let signature = &impl_method.sig;
                let name = &signature.ident;
                quote! {
                    pub #signature {
                        #trait_name::#name(self)
                    }
                }
            }
            _ => continue,
        });
    }

    Ok(quote! {
        #item_impl
        impl #self_ty {
            #(#implementations)*
        }
    })
}

#[proc_macro_attribute]
pub fn expose(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let stream = match impl_expose(input) {
        Ok(tokens) => tokens,
        Err(err) => err.into_compile_error(),
    };
    stream.into()
}
