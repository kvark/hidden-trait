use proc_macro::TokenStream;
use quote::quote;

fn impl_expose(input_stream: TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let item_impl = syn::parse::<syn::ItemImpl>(input_stream)?;
    let mut implementations = Vec::new();

    for item_orig in item_impl.items.iter() {
        let mut item = item_orig.clone();
        let vis = match item {
            syn::ImplItem::Const(ref mut impl_const) => &mut impl_const.vis,
            syn::ImplItem::Method(ref mut impl_method) => &mut impl_method.vis,
            syn::ImplItem::Type(ref mut impl_type) => &mut impl_type.vis,
            _ => continue,
        };
        *vis = syn::Visibility::Public(syn::VisPublic {
            pub_token: syn::token::Pub {
                span: proc_macro2::Span::call_site(),
            },
        });
        implementations.push(item);
    }

    let self_ty = &item_impl.self_ty;
    Ok(quote! {
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
