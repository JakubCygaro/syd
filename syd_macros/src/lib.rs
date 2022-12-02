use std::{ops::Deref, fmt::Arguments};

use proc_macro::TokenStream;
use syn::{self, Attribute};
use quote::quote;

#[proc_macro_attribute]
pub fn command_module(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(item as syn::ItemImpl);

    impl_command_module(&ast)
}

fn impl_command_module(ast: &syn::ItemImpl) -> TokenStream {
    if let syn::Type::Path(a) = &*ast.self_ty {
        let mut methods = vec![];
        for i in ast.items
                .iter()
                .filter_map(|x| match x {
                    syn::ImplItem::Method(m) => Some(m),
                    _ => None,
                })
        {
            // get method args
            let inputs = &i.sig.inputs;
            
            // method must have only 2 args
            if inputs.len() != 1 { continue; }
            // first arg cannot be self
            if let Some(syn::FnArg::Receiver(_)) = &inputs.first() {
                continue;
            }
            //method must be public
            let syn::Visibility::Public(_) = &i.vis else { continue;};

            //first arg is a type
            let Some(syn::FnArg::Typed(arg)) = &inputs.first() else { continue; };
            //is a reference
            let syn::Type::Reference(a) = &*arg.ty else { continue; };
            //is a mut reference
            let Some(_) = a.mutability else { continue; };
            //of type CommandContext
            let syn::Type::Path(ty) = &*a.elem else { continue; };
            let Some(last) = ty.path.segments.last() else { continue; };
            if last.ident != "CommandContext" { continue; }

            //check if method returns Result<()>
            let output = &i.sig.output;
            let syn::ReturnType::Type(_, a) = output else { continue; };
            let syn::Type::Path(path) = &**a else { continue; };
            let Some(seg) = path.path.segments.last() else { continue;};
            if seg.ident != "Result" { continue; };

            let syn::PathArguments::AngleBracketed(bracketed) =
                &seg.arguments else { continue; };
            let Some(syn::GenericArgument::Type(gen_ty)) = 
                bracketed.args.first() else { continue; };
            if let syn::Type::Path(path) = gen_ty {
                continue;
            } 
            let mut args_count = None::<i32>;
            if !&i.attrs.is_empty(){
                let valid = i.attrs.iter()
                    .filter(|a| a.path.segments.last()
                        .unwrap().ident == "command_args")
                    .collect::<Vec<&Attribute>>();
                if let Some(first) = valid.first() {
                    let Ok(syn::Lit::Int(count)) 
                        = first.parse_args() else { panic!("todo 3")};
                    args_count = Some(count.base10_parse::<i32>().unwrap());
                }
            }
                    

            methods.push((i, args_count));
        }
        
        
        let mut init_method: syn::ImplItemMethod = syn::parse_quote!(
            fn init() -> Vec<Command> {
                let mut commands: Vec<Command> = vec![];
            }
        );
        
        let mut stmts = vec![];
        for (m, a) in methods {
            let path = &m.sig.ident;
            if let Some(count) = a {
                let count = count as usize;
                let stmt: syn::Stmt = syn::parse_quote!{
                    commands.push( Command {
                        name: stringify!(#path).into(),
                        args_num: Some(#count),
                        function: Box::new(Self::#path),
                    });
                };
                stmts.push(stmt);
            }
            else {
                let stmt: syn::Stmt = syn::parse_quote!{
                    commands.push( Command {
                        name: stringify!(#path).into(),
                        args_num: None,
                        function: Box::new(Self::#path),
                    });
                };
                stmts.push(stmt);
            }

        }
        init_method.block.stmts.extend(stmts);

        init_method.block.stmts.push(syn::parse_quote!{
            return commands;
        });

        //implement CommandModule for this struct
        let struct_name = &a.path.segments.last().unwrap().ident;
        let mut trait_impl: syn::ItemImpl = syn::parse_quote!(
            impl CommandModule for #struct_name {

            }
        );
        trait_impl.items.push(syn::ImplItem::Method(init_method));

        quote!{
            #ast
            #trait_impl
        }.into()
    }
    else {
        panic!("Failed to resolve struct name!")
        
    }
}

#[proc_macro_attribute]
pub fn command_args(args: TokenStream, item: TokenStream) -> TokenStream {
    let method_ast = syn::parse_macro_input!(item as syn::ItemFn);
    let args_ast = syn::parse_macro_input!(args as syn::AttributeArgs);

    impl_command_args(&method_ast, &args_ast)
}

fn impl_command_args(function: &syn::ItemFn, args: &Vec<syn::NestedMeta>) -> TokenStream {
    if args.len() != 1 {
        panic!("the `command_args` macro must contain only one argument of type usize");
    }
    let Some(syn::NestedMeta::Lit(nested)) = args.first() else 
        { panic!("failed parsing attribute argument") };
    let syn::Lit::Int(lit) = nested else 
        { panic!("failed parsing attribute argument") };
    lit.base10_parse::<usize>().expect("macro argument must be of type usize");

    //enforce that there is only one attribute of this type used
    let attrs = &function.attrs;
    if attrs.iter()
        .any(|a| a.path.segments.last().unwrap().ident == "command_args") {
            panic!("this attribute can only be used once.")
    }

    quote!{
        #function
    }.into()
}