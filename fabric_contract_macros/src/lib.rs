/*
 * SPDX-License-Identifier: Apache-2.0
 */
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![recursion_limit = "128"]

//! Definitions of the macros used for the Contracts.
//!  
//! There are macros for use with the Contract Structs, and the Transaction
//! Functions
// DevNote: did experiment with macros that can added on parameters but these are experimental nightly only
// moving to arguments in the transaction decorator
// #![feature(proc_macro_diagnostic,param_attrs)]
extern crate proc_macro;

// Bring in quite a lot of different crates, noteably the syn crate for handling the
// AST.
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse_macro_input, punctuated::Punctuated, AttributeArgs, FnArg, ItemFn, Pat, Type, TypePath,
    TypeReference,
};

// DevNote: Most basic attribute procedural macro, keep here for reference and debug

// #[proc_macro_attribute]
// pub fn transient(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     println!("attr: \"{}\"", attr.to_string());
//     println!("item: \"{}\"", item.to_string());
//     item
// }

/// Use this macro to mark the implementation of the your contract structure
///
/// # Example
/// ```ignore
/// // Use the Fabric Contract modules
 /// struct MyContract {}
///
/// #[Contract_Impl]
/// impl MyContract {
///
/// #[Transaction(evaluate)]
/// pub fn my_query_style_fn() {  }
///
/// #[Transaction(submit)]
/// pub fn my_submit_asset_fn() {  }
///
/// // this is NOT callable as transaction function
/// fn helper() { }
/// }
/// ```
///
/// This macro's purpose is to implement the 'routing' trait for the contract. This permits
/// the message from the peer to correctly routed to the transaction function required.
///
/// This trait relies on the transaction functions also being marked with the `#[transaction]`
/// macro.
///
/// If you wished you could provide the implementations of these macros yourself, but it is 
/// recommended to not do this unless you really have to.
#[proc_macro_attribute]
pub fn contract_impl(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // parse the incoming AST, we don't wish to modify the existing code so immediately
    // quote! this to ensure it's in the output.
    let ty = parse_macro_input!(input as syn::ItemImpl);
    let existing = quote! {#ty};

    // Need to navigate down to the indentifier of the struct
    // Honestly... not quite sure how this actually works as the self_ty structure is
    // quite a few structs/enums deep so how this can be directly accessed ??
    let type_name = match *ty.self_ty {
        syn::Type::Path(ref path) => &path.path,
        _ => panic!(),
    };

    // we need to have the names of the methods, both as literal strings
    // and identifiers to call
    let mut method_names = Vec::new();
    let mut method_fns = Vec::new();
    let mut method_md = Vec::new();
    for i in ty.items {
        if let syn::ImplItem::Method(ref method) = i {
            let method = method.clone();
            let name = &method.sig.ident;

            // ignore the new method
            // TODO this in a better way! i.e. only the fns marked #[transaction]

            if name != "new"
                && !name.to_string().starts_with("invoke")
                && !name.to_string().starts_with("md")
            {
                method_fns.push(syn::Ident::new(&format!("invoke_{}", name), name.span()));
                method_md.push(syn::Ident::new(&format!("md_{}", name), name.span()));
                method_names.push(ident_to_litstr(name));
            }
            // build up the list arguments to the function we're going to call
            let call_args = extract_arg_idents(method.sig.inputs);
        }
    }

    // quote! the existing code, and also the new routing implementation
    let output = quote! {
       #existing

        impl Metadata for #type_name {

           fn get_fn_metadata(&self) -> std::vec::Vec<fabric_contract::prelude::TransactionFn> {
                 let mut fns = Vec::new();
                 #(fns.push(self.#method_md()); )*
                fns
             }
        }

         impl Routing for #type_name {

                fn route3(&self, tx_fn: String, args: Vec<WireBuffer>, return_wb: TypeSchema) -> Result<WireBuffer,ContractError> {
                    log::debug!("Inside the contract (route3) {} {:?}",tx_fn,args);
                    match &tx_fn[..] {

                         #(#method_names =>
                             {
                                log::debug!("calling");
                                self.#method_fns(args,return_wb)
                             }

                             , )*
                        _ => Err(ContractError::from(String::from("Unknown transaction fn ")))
                    }

                }
         }

    };

    // Hand the output tokens back to the compiler.
    output.into()
}

/// Convert from syn::Ident to literal string
fn ident_to_litstr(ident: &syn::Ident) -> syn::LitStr {
    syn::LitStr::new(&ident.to_string()[..], proc_macro2::Span::call_site())
}

/// Use this to mark the functions that are considered to be transaction functions
///
/// Arguments to this provide the ability to indicate
/// - that this function is intended to be evaluated or submitted
/// - which arguments are from the set of transient data (use variable names)
///
/// # Example
///
/// ```ignore
/// use fabric_contract::contract::*;
/// #[Transaction]
/// pub fn createAsset() { }
///
/// #[Transaction(submit)]
/// pub fn createAnotherAsset() { }

/// #[Transaction(evaluate)]
/// pub fn readAsset() { }
///
/// #[Transaction(tranisent = {price, owner} )]
/// pub fn createDetailedAsset(id: String, price: u32, owner: String ) { }
/// ```
///
#[proc_macro_attribute]
pub fn transaction(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let psitem = parse_macro_input!(input as ItemFn);

    let name = psitem.sig.ident.clone();
    let classname = syn::Ident::new(&format!("{}{}", "invoke_", name), psitem.sig.ident.span());
    let metadata = syn::Ident::new(&format!("{}{}", "md_", name), psitem.sig.ident.span());
    let name_as_literal = ident_to_litstr(&name);

    let ret_type = match psitem.sig.output.clone() {
        syn::ReturnType::Default => Box::new(syn::parse_quote!(())),
        syn::ReturnType::Type(_, ret_type) => ret_type,
    };

    let mut arg_names = Vec::new();
    let mut aargs = Vec::new();

    let mut metadata_args = Vec::new();

    // the overall algorthim here should be consiered candidate for optimization
    // It iterates over the signature, skipping the self reference
    // then gets the name of the argument
    for input in psitem.sig.inputs.iter().skip(1) {
        match input {
            FnArg::Typed(arg) => {
                let stringify = arg.to_token_stream().to_string();
                let pat = &arg.pat;
                let ty = &arg.ty;
                let comment = format!("{:?}", ty);
                aargs.push(quote! {

                   let #pat = #ty::from(&args[i]);
                   i+=1; // and convert convert_from( );

                });
                arg_names.push(quote! { # pat });

                metadata_args.push(quote! {
                    tx.add_arg(#stringify);
                });
            }
            _ => panic!(),
        }
    }

    let output = quote! {

        #psitem

        // hello
       //pub fn #classname(&self, args: Vec<WireBuffer>) -> #ret_type {
        pub fn #classname(&self, args: Vec<WireBuffer>, return_wb: TypeSchema) -> Result<WireBuffer,ContractError> {
            let mut i=0;
            #(#aargs)*

            match self.#name(#(#arg_names),*) {
                Ok(r) => {
                    let mut wb = WireBuffer::new_unfilled(return_wb);
                    wb.from_rt(r);
                    Ok(wb)
                }, Err(e) => Err(e)
            }
        }

        pub fn #metadata(&self) -> fabric_contract::prelude::TransactionFn {
            let mut tx = fabric_contract::prelude::TransactionFnBuilder::new();
            tx.name(#name_as_literal);

            #(#metadata_args)*

            tx.build()
        }
    };
    output.into()
}

/// Define the properties of the datatype
#[proc_macro_attribute]
pub fn property(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    input
}

///
/// Use this to mark the structs that serve as complex data types
/// Need to provide example
///
#[proc_macro_derive(DataTypeMacro)]
pub fn data_type_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
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

fn extract_arg_idents(fn_args: Punctuated<FnArg, syn::token::Comma>) -> Vec<Pat> {
    fn_args
        .into_iter()
        .skip(1)
        .map(extract_arg_pat)
        .collect::<Vec<_>>()
}

fn extract_arg_pat(a: FnArg) -> Pat {
    match a {
        FnArg::Typed(p) => *p.pat,
        _ => panic!(),
    }
}
