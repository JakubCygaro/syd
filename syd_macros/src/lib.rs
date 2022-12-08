use std::{ops::Deref, fmt::Arguments, thread::panicking};

use proc_macro::TokenStream;
use syn::{self, Attribute, token::Token};
use quote::quote;


#[proc_macro_attribute]
pub fn command(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(item as syn::ImplItemMethod);

    impl_command(&ast)
}

fn impl_command(function: &syn::ImplItemMethod) -> TokenStream {
    // get method args
    let inputs = &function.sig.inputs;
            
    // method must have only 2 args
    if inputs.len() < 1 { 
        panic!("A command must have at least one argument of type `&mut CommandContext`"); 
    }
    
    let attrs = &function.attrs;
    if attrs.iter()
        .any(|a| a.path.segments.last().unwrap().ident == "command_args") {
            panic!("this attribute can only be used once.")
    }
    
    let Some(syn::FnArg::Typed(t)) = &inputs.first() else {
        panic!("First argument must be of type `&mut CommandContext`");
    };
    let syn::Type::Reference(r) = &*t.ty else {
        panic!("First argument is not a reference");
    };
    let Some(_) = r.mutability else {
        panic!("Reference must be mutable");
    };
    let syn::Type::Path(p) = &*t.ty else {
        panic!("Failed to parse first argument type path");
    };
    if p.path.segments.last().unwrap().ident != "CommandContext" {
        panic!("First argument must be of type `&mut CommandContext`");
    }

    //check if method returns Result<()>
    let output = &function.sig.output;
    let syn::ReturnType::Type(_, a) = output else { 
        panic!("The return type of a command must be `Result<()>"); 
    };
    let syn::Type::Path(path) = &**a else { 
        panic!("The return type of a command must be `Result<()>"); 
    };
    let Some(seg) = path.path.segments.last() else { 
        panic!("The return type of a command must be `Result<()>"); 
    };
    if seg.ident != "Result" { 
        panic!("The return type of a command must be `Result<()>"); 
    };

    let syn::PathArguments::AngleBracketed(bracketed) =
        &seg.arguments else { 
            panic!("The return type of a command must be `Result<()>"); 
        };
    let Some(syn::GenericArgument::Type(gen_ty)) = 
        bracketed.args.first() else { 
            panic!("The return type of a command must be `Result<()>"); 
        };
    if let syn::Type::Path(_path) = gen_ty {
        panic!("The return type of a command must be `Result<()>");
    } else {
        quote!{
            #function
        }.into()
    }
}


/// 
/// Implements `CommandModule` for a struct, by registering certain methods as commands,
/// must be used on its' `impl` block.
/// 
/// Will only register functions that are public, have a single `&mut CommandContext` argument
/// and a return type of `anyhow::Result<()>`.
/// ```
/// pub fn foo(context: &mut CommandContext) -> Result<()> {
///     /.../
/// }
/// ```
/// 
/// # Example
/// ```
///pub struct TestModule;
///#[command_module]
///impl TestModule {
///    pub fn test(context: &mut CommandContext) -> Result<()> {
///        println!("Working!");
///        Ok(())
///    }
///}
/// ```
/// Macro used in this exaple will emit this code
/// ```
/// impl CommandModule for TestModule {
///     pub fn init() -> Vec<Command> {
///         let commands: Vec<Command> = vec![];
///         commands.push(Command{
///             name: "test".into(),
///             desc: None,
///             args_num: None,
///             function: Box::new(Self::test),
///         });
///         return commands;    
///     }
/// }
/// ```
/// 
/// ⣿⣿⣿⣿⣿⣿⣿⣿⡿⠿⠛⠛⠛⠋⠉⠈⠉⠉⠉⠉⠛⠻⢿⣿⣿⣿⣿⣿⣿⣿
/// ⣿⣿⣿⣿⣿⡿⠋⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠛⢿⣿⣿⣿⣿
/// ⣿⣿⣿⣿⡏⣀⠀⠀⠀⠀⠀⠀⠀⣀⣤⣤⣤⣄⡀⠀⠀⠀⠀⠀⠀⠀⠙⢿⣿⣿
/// ⣿⣿⣿⢏⣴⣿⣷⠀⠀⠀⠀⠀⢾⣿⣿⣿⣿⣿⣿⡆⠀⠀⠀⠀⠀⠀⠀⠈⣿⣿
/// ⣿⣿⣟⣾⣿⡟⠁⠀⠀⠀⠀⠀⢀⣾⣿⣿⣿⣿⣿⣷⢢⠀⠀⠀⠀⠀⠀⠀⢸⣿
/// ⣿⣿⣿⣿⣟⠀⡴⠄⠀⠀⠀⠀⠀⠀⠙⠻⣿⣿⣿⣿⣷⣄⠀⠀⠀⠀⠀⠀⠀⣿
/// ⣿⣿⣿⠟⠻⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠶⢴⣿⣿⣿⣿⣿⣧⠀⠀⠀⠀⠀⠀⣿
/// ⣿⣁⡀⠀⠀⢰⢠⣦⠀⠀⠀⠀⠀⠀⠀⠀⢀⣼⣿⣿⣿⣿⣿⡄⠀⣴⣶⣿⡄⣿
/// ⣿⡋⠀⠀⠀⠎⢸⣿⡆⠀⠀⠀⠀⠀⠀⣴⣿⣿⣿⣿⣿⣿⣿⠗⢘⣿⣟⠛⠿⣼
/// ⣿⣿⠋⢀⡌⢰⣿⡿⢿⡀⠀⠀⠀⠀⠀⠙⠿⣿⣿⣿⣿⣿⡇⠀⢸⣿⣿⣧⢀⣼
/// ⣿⣿⣷⢻⠄⠘⠛⠋⠛⠃⠀⠀⠀⠀⠀⢿⣧⠈⠉⠙⠛⠋⠀⠀⠀⣿⣿⣿⣿⣿
/// ⣿⣿⣧⠀⠈⢸⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠟⠀⠀⠀⠀⢀⢃⠀⠀⢸⣿⣿⣿⣿
/// ⣿⣿⡿⠀⠴⢗⣠⣤⣴⡶⠶⠖⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⡸⠀⣿⣿⣿⣿
/// ⣿⣿⣿⡀⢠⣾⣿⠏⠀⠠⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠛⠉⠀⣿⣿⣿⣿
/// ⣿⣿⣿⣧⠈⢹⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⣿⣿⣿⣿
/// ⣿⣿⣿⣿⡄⠈⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣠⣴⣾⣿⣿⣿⣿⣿
/// ⣿⣿⣿⣿⣧⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣠⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿
/// ⣿⣿⣿⣿⣷⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
/// ⣿⣿⣿⣿⣿⣦⣄⣀⣀⣀⣀⠀⠀⠀⠀⠘⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
/// ⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⡄⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
/// ⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧⠀⠀⠀⠙⣿⣿⡟⢻⣿⣿⣿⣿⣿⣿⣿⣿⣿
/// ⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠇⠀⠁⠀⠀⠹⣿⠃⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿
/// ⣿⣿⣿⣿⣿⣿⣿⣿⡿⠛⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⢐⣿⣿⣿⣿⣿⣿⣿⣿⣿
/// ⣿⣿⣿⣿⠿⠛⠉⠉⠁⠀⢻⣿⡇⠀⠀⠀⠀⠀⠀⢀⠈⣿⣿⡿⠉⠛⠛⠛⠉⠉
/// ⣿⡿⠋⠁⠀⠀⢀⣀⣠⡴⣸⣿⣇⡄⠀⠀⠀⠀⢀⡿⠄⠙⠛⠀⣀⣠⣤⣤⠄⠀
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
            if let syn::Type::Path(_path) = gen_ty {
                continue;
            } 
            let args_count = get_args_count(&i.attrs);
            let desc = get_description(&i.attrs);
                    

            methods.push((i, args_count, desc));
        }
        
        
        let mut init_method: syn::ImplItemMethod = syn::parse_quote!(
            fn init() -> Vec<Command> {
                let mut commands: Vec<Command> = vec![];
            }
        );
        //check if there is a group defined for these commands
        let impl_group = get_group(&ast.attrs);
        
        let mut stmts = vec![];
        for (m, a, d) in methods {
            let path = &m.sig.ident;

            let args_num;
            if let Some(args) = a {
                let args = args as usize;
                args_num = quote!{Some(#args)};
            } else {
                args_num = quote!{None};
            }
            let description;
            if let Some(desc) = d {
                description = quote!{Some(#desc.to_owned())};
            } else {
                description = quote!{None};
            }
            let group;
            if let Some(g) = &impl_group {
                group = quote!{Some(#g.to_owned())};
            } else {
                group = quote!{None};
            }

            let stmt: syn::Stmt = syn::parse_quote!{
                commands.push( Command {
                    name: stringify!(#path).into(),
                    group: #group,
                    desc: #description,
                    args_num: #args_num,
                    function: Box::new(Self::#path),
                });
            };
            stmts.push(stmt);
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

fn get_group(attrs: &Vec<Attribute>) -> Option<String> {
    let valid = attrs.iter()
        .filter(|a| a.path.segments.last()
            .unwrap().ident == "command_group")
        .collect::<Vec<&Attribute>>();
    if let Some(first) = valid.first() {
        let Ok(syn::Lit::Str(group)) 
            = first.parse_args() else { panic!("failed parsing command group")};
        return Some(group.value());
    }
    None
}

fn get_args_count(attrs: &Vec<Attribute>) -> Option<i32> {
    let valid = attrs.iter()
        .filter(|a| a.path.segments.last()
            .unwrap().ident == "command_args")
        .collect::<Vec<&Attribute>>();
    if let Some(first) = valid.first() {
        let Ok(syn::Lit::Int(count)) 
            = first.parse_args() else { panic!("failed parsing arg count")};
        return Some(count.base10_parse::<i32>().unwrap());
    }
    None
}

fn get_description(attrs: &Vec<Attribute>) -> Option<String> {
    let valid = attrs.iter()
        .filter(|a| a.path.segments.last()
            .unwrap().ident == "command_description")
        .collect::<Vec<&Attribute>>();

    if let Some(first) = valid.first() {
        let Ok(syn::Lit::Str(d)) 
            = first.parse_args() else { panic!("failed parsing description")};
        return Some(d.value());
    }
    None
}

/// Tells the `CommandHandler` how many arguments this command will require.
///⣀⣠⣤⣤⣤⣤⢤⣤⣄⣀⣀⣀⣀⡀⡀⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄
///⠄⠉⠹⣾⣿⣛⣿⣿⣞⣿⣛⣺⣻⢾⣾⣿⣿⣿⣶⣶⣶⣄⡀⠄⠄⠄
///⠄⠄⠠⣿⣷⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣯⣿⣿⣿⣿⣿⣿⣆⠄⠄
///⠄⠄⠘⠛⠛⠛⠛⠋⠿⣷⣿⣿⡿⣿⢿⠟⠟⠟⠻⠻⣿⣿⣿⣿⡀⠄
///⠄⢀⠄⠄⠄⠄⠄⠄⠄⠄⢛⣿⣁⠄⠄⠒⠂⠄⠄⣀⣰⣿⣿⣿⣿⡀
///⠄⠉⠛⠺⢶⣷⡶⠃⠄⠄⠨⣿⣿⡇⠄⡺⣾⣾⣾⣿⣿⣿⣿⣽⣿⣿
///⠄⠄⠄⠄⠄⠛⠁⠄⠄⠄⢀⣿⣿⣧⡀⠄⠹⣿⣿⣿⣿⣿⡿⣿⣻⣿
///⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠉⠛⠟⠇⢀⢰⣿⣿⣿⣏⠉⢿⣽⢿⡏
///⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠠⠤⣤⣴⣾⣿⣿⣾⣿⣿⣦⠄⢹⡿⠄
///⠄⠄⠄⠄⠄⠄⠄⠄⠒⣳⣶⣤⣤⣄⣀⣀⡈⣀⢁⢁⢁⣈⣄⢐⠃⠄
///⠄⠄⠄⠄⠄⠄⠄⠄⠄⣰⣿⣛⣻⡿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡯⠄⠄
///⠄⠄⠄⠄⠄⠄⠄⠄⠄⣬⣽⣿⣻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠁⠄⠄
///⠄⠄⠄⠄⠄⠄⠄⠄⠄⢘⣿⣿⣻⣛⣿⡿⣟⣻⣿⣿⣿⣿⡟⠄⠄⠄
///⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠛⢛⢿⣿⣿⣿⣿⣿⣿⣷⡿⠁⠄⠄⠄
///⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠄⠉⠉⠉⠉⠈⠄⠄⠄⠄⠄⠄
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

#[proc_macro_attribute]
pub fn command_description(args: TokenStream, item: TokenStream) -> TokenStream {
    let method_ast = syn::parse_macro_input!(item as syn::ItemFn);
    let args_ast = syn::parse_macro_input!(args as syn::AttributeArgs);

    impl_command_description(&method_ast, &args_ast)
}

fn impl_command_description(function: &syn::ItemFn, args: &Vec<syn::NestedMeta>) 
    -> TokenStream {
    if args.len() != 1 {
        panic!("the `command_description` macro must contain only one argument of type String");
    }
    let Some(syn::NestedMeta::Lit(nested)) = args.first() else 
        { panic!("failed parsing attribute argument 1") };
    let syn::Lit::Str(_) = nested else 
        { panic!("failed parsing attribute argument 2") };

    let attrs = &function.attrs;
    if attrs.iter()
        .any(|a| a.path.segments.last().unwrap().ident == "command_description") {
            panic!("this attribute can only be used once.")
    }
    quote!{
        #function
    }.into()
    
}

#[proc_macro_attribute]
pub fn command_group(args: TokenStream, item: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(args as syn::AttributeArgs);
    let item = syn::parse_macro_input!(item as syn::ItemImpl);

    impl_command_group(&item, &args)
}

fn impl_command_group(function: &syn::ItemImpl, args: &Vec<syn::NestedMeta>) -> TokenStream {
    if args.len() != 1 {
        panic!("the `command_group` macro must contain only one argument of type String");
    }
    let Some(syn::NestedMeta::Lit(nested)) = args.first() else 
        { panic!("failed parsing attribute argument 1") };
    let syn::Lit::Str(lit) = nested else 
        { panic!("failed parsing attribute argument 2") };
    let group_name = lit.value();
    if group_name.contains(' ') || !group_name.is_ascii() {
        panic!("group name may only contain ascii characters and no spaces!")
    }


    let attrs = &function.attrs;
    if attrs.iter()
        .any(|a| a.path.segments.last().unwrap().ident == "command_group") {
            panic!("this attribute can only be used once.")
    }
    quote!{
        #function
    }.into()
}