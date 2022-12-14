use proc_macro::TokenStream;
use syn::{self, Attribute, punctuated::Punctuated};
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
    let syn::Type::Path(p) = &*r.elem else {
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
    } 
    let name = &function.sig.ident;
    let name = format!("{}_parse", name);
    let name: syn::Ident = syn::parse_str(&name).unwrap();
    let arg_count = inputs.len() - 1;
    let mut parse_method: syn::ImplItemMethod = syn::parse_quote!{
        #[doc(hidden)]
        pub fn #name (context: &mut CommandContext, args: Vec<String>) -> Result<()> {
            use anyhow::anyhow;
            use syd::commands::ArgParse;
            if args.len() != #arg_count {
                return Err(anyhow!("invalid argument count!"));
            }  
        }
    };

    let ident = &function.sig.ident;
    let mut caller = syn::ExprCall {
        func: Box::new(syn::parse_quote!{Self::#ident}),
        args: Punctuated::new(),
        attrs: Vec::new(),
        paren_token: syn::token::Paren::default(),
    };
    caller.args.push(syn::parse_quote!{context});

    for (n,i) in inputs.iter().skip(1).enumerate() {
        let syn::FnArg::Typed(pat) = i else {panic!("adadada")};
        let syn::Type::Path(path) = &*pat.ty else {panic!("adad")};
        let arg: syn::Ident = syn::parse_str(&format!("arg{}", n)).unwrap();
        let stmt: syn::Stmt = syn::parse_quote!{
            let #arg = <#path as ArgParse>::arg_parse(&args[#n])?;
        };
        parse_method.block.stmts.push(stmt);
        caller.args.push(syn::parse_quote!{ #arg });
    };
    parse_method.block.stmts.push(syn::parse_quote!{#caller?;});
    parse_method.block.stmts.push(syn::parse_quote!{ return Ok(()); });

    quote!{
        #function
        #parse_method
    }.into()
    
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
        let methods =  ast.items
                .iter()
                .filter_map(|x| match x {
                    syn::ImplItem::Method(m) => Some(m),
                    _ => None,
                })
                .filter(|m| {
                    m.attrs.iter().any(|a| {
                        a.path.segments.last().unwrap().ident == "command"
                    })
                })
                .collect::<Vec<&syn::ImplItemMethod>>();        
        
        let mut init_method: syn::ImplItemMethod = syn::parse_quote!(
            fn init() -> Vec<Command> {
                let mut commands: Vec<Command> = vec![];
            }
        );
        //check if there is a group defined for these commands
        let impl_group = get_group(&ast.attrs);
        
        let mut stmts = vec![];
        for m in methods {
            let path = &m.sig.ident;

            let args = get_args(&m.sig.inputs);
            let description;
            if let Some(desc) = get_description(&m.attrs) {
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
            let ident: syn::Ident = syn::parse_str(&format!("{}_parse", path)).unwrap();
            let stmt: syn::Stmt = syn::parse_quote!{
                commands.push( Command {
                    name: stringify!(#path).into(),
                    group: #group,
                    desc: #description,
                    args: args,
                    function: Box::new(Self::#ident),
                });
            };
            stmts.extend(args);
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
            use syd::commands::*;
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

fn get_args(input: &Punctuated<syn::FnArg, syn::token::Comma>) -> Vec<syn::Stmt> {
    let mut stmts: Vec<syn::Stmt> = vec![];
    stmts.push(syn::parse_quote!{
        let mut args: Vec<CommandArg> = vec![];
    });
    for arg in input.iter().skip(1) {
        let syn::FnArg::Typed(t) = arg else { panic!("todo") };
        let name = &*t.pat;
        let ty = &*t.ty;
        stmts.push(syn::parse_quote!{
            args.push(CommandArg{
                name: stringify!(#name).to_owned(),
                ty: stringify!(#ty).to_owned(),
            });
        });
    }
    stmts
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