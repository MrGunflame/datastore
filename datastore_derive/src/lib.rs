mod storedata;

use proc_macro::TokenStream;
use proc_macro2::Ident;

#[proc_macro_derive(StoreData)]
pub fn storedata(input: TokenStream) -> TokenStream {
    storedata::expand_macro(input)
}
