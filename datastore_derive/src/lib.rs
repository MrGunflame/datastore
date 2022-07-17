mod storedata;

use proc_macro::TokenStream;

#[proc_macro_derive(StoreData, attributes(datastore))]
pub fn storedata(input: TokenStream) -> TokenStream {
    storedata::expand_macro(input)
}
