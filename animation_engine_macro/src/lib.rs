mod body;

use proc_macro::TokenStream;

#[proc_macro]
pub fn anim_components(item: TokenStream) -> TokenStream {
    body::anim_components(item.into()).into()
}
