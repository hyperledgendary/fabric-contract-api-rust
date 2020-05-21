/*
 * SPDX-License-Identifier: Apache-2.0
 */
#![recursion_limit = "128"]
// #![feature(proc_macro_diagnostic,param_attrs)]
extern crate proc_macro;


// Bring in quite a lot of different crates, noteably the syn crate for handling the 
// AST.
// Suspect that a lot of these aren't actually needed
//use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, AttributeArgs,ItemFn,
};

// use syn::{
//     parse::Error, parse_macro_input, parse_quote, punctuated::Punctuated, spanned::Spanned, FnArg,
//     Generics, Ident, ImplItem, ImplItemMethod, ItemImpl, ReturnType, Token, Type, TypeTuple,
// };

//use quote::ToTokens;
//use quote::TokenStreamExt;


// #[proc_macro_attribute]
// pub fn transient(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     println!("attr: \"{}\"", attr.to_string());
//     println!("item: \"{}\"", item.to_string());
//     item
// }

/// Use this macro to mark the impl of the contract functions
/// 
/// 
/// # Example
/// 
/// ```
///  
/// #[contract_impl]
/// impl MyContract {
/// 
///     pub fn my_asset_fn() { 
///         OK(())
///     }
/// 
/// }
/// 
#[proc_macro_attribute]
pub fn contract_impl(_args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {

    // parse the incoming AST, we don't wish to modify the existing code so immediately
    // quote! this to ensure it's in the output.
    let ty = parse_macro_input!(input as syn::ItemImpl);
    let existing = quote!{#ty};

    // Need to navigate down to the indentifier of the struct
    // Honestly... not quite sure how this actually works as the self_ty structure is
    // quite a few structs/enums deep so how this can be directly accessed ??
    let type_name = match *ty.self_ty {
        syn::Type::Path(ref path) => &path.path,
        _ => panic!()
    };

    // we need to have the names of the methods, both as literal strings
    // and identifiers to call
    let mut method_names = Vec::new();
    let mut method_fns = Vec::new();
    for i in ty.items {
        match i {
            
            syn::ImplItem::Method(ref method) => {
                let method = method.clone();
                let name = &method.sig.ident;
                method_fns.push(syn::Ident::new(&format!("{}",name), name.span()));
                method_names.push(ident_to_litstr(name));

                // todo: sort out the arguments; left to another day
                // this code will be long and boring, but conceptually simple based
                // on the other contract implementations
                //
                // build up the list arguments to the function we're going to call
                // let mut call_args = Vec::new();
                // for input in method.sig.decl.inputs.iter().skip(1) {
                //     match input {
                //         syn::FnArg::Captured(arg) => { let pat = &arg.pat; call_args.push(quote!(#pat)) }
                //         _ => { panic!() }
                //     }
                // }
                // println!("{:?}",call_args); 
            }
            _ => {}
        }
    }

    // quote! the existing code, and also the new routing implementation 
    // lots more to add here.....
    let output = quote! {
       #existing

        impl Routing for #type_name {         

            fn route2(&self, ctx: Context, tx_fn: String, args: Vec<String>) -> Result<String,String>{
                ctx.log(format!("Inside the contract {} {:?}",tx_fn,args));
                let _r = match &tx_fn[..] {
          
                    #(#method_names => 
                        {
                            //let _r = self.#method_fns(ctx );
                            Ok(())
                        }
                        
                        , )*       
                    _ => Err(String::from("Unknown transaction fn "))
                };
                Ok(String::from("200"))
            }
        }

    };

    // Hand the output tokens back to the compiler.
    output.into()
}

/// Convert from syn::Ident to literal string
fn ident_to_litstr(ident: &syn::Ident) -> syn::LitStr {
    syn::LitStr::new(&ident.to_string()[..],proc_macro2::Span::call_site())
}

/// Use this to mark the functions that are considered to be transaction functions
/// 
/// Arguments to this provide the ability to indicate 
/// - that this function is intended to be evaluated or submitted
/// - which arguments are from the set of transient data
/// 
/// # Example
/// 
/// This shoulds a transaction function that will will be marked as to be 'submitted'
/// ```
/// #[transaction]
/// pub fn createAsset()...
/// 
/// /// This shoulds a transaction function that will will be marked as to be 'submitted'
/// ```
/// #[transaction]
/// pub fn createAsset()...
/// 
/// #[transaction(submit)]
/// pub fn createAsset()...
/// ```
/// 
/// ```
/// This shoulds a transaction function that will will be marked as to be 'evaluated'
/// ```
/// #[transaction(evaluate)]
/// pub fn createAsset()...
/// 
/// ```
/// 
/// #[transaction(tranisent = {price, owner} )]
/// pub fn createAsset(id: String, price: u32, owner: String ) ...alloc
/// ```
/// 
#[proc_macro_attribute]
pub fn transaction(_args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    input
}


///
/// Use this to mark the structs that serve as complex data types
/// 
/// # Example
/// 
/// ```
/// #[derive(DataType)]
/// pub struct MyAsset {
///     assetid: String
/// }
/// 
/// pub fn get_asset() -> Result<MyAsset,String> { ... }
/// 
/// ```
/// 
#[proc_macro_derive(DataType)]
pub fn data_type_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream  {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl DataType for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}