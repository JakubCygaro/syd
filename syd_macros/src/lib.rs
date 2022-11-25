use proc_macro::TokenStream;
use syn;
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
            if inputs.len() != 2 { continue; }
            // first arg cannot be self
            if let syn::FnArg::Receiver(_) = inputs.first().unwrap() {
                continue;
            }

            //first arg is a type
            let Some(syn::FnArg::Typed(arg)) = inputs[0] else { continue; };
            //is a reference
            let Some(syn::Type::Reference(a)) = &*arg.ty else { continue; };
            //is a mut reference
            let Some(_) = a.mutability else { continue; };
            //of type CommandContext
            let syn::Type::Path(ty) = &*a.elem else { continue; };
            let Some(last) = ty.path.segments.last() else { continue; };
            if last.ident != "ComandContext" { continue; }

            //second arg is a type
            let Some(syn::FnArg::Typed(arg)) = inputs[1] else { continue; };
            //is Vec
            let Some(syn::Type::Path(a)) = &*arg.ty else { continue; };
            let Some(ty) = a.path.segments.last() else { continue; };
            if ty.ident != "Vec" { continue; }
            // is Vec<String>
            let syn::PathArguments::AngleBracketed(nested) = 
                ty.arguments else { continue; }; 
            let Some(syn::GenericArgument::Type(syn::Type::Path(generic))) =
                nested.args.first() else { continue; };
            let Some(t) = generic.path.segments.last() else { continue; };
            if t.ident != "String" { continue; }

            //check if method returns Result<()>
            let output = &i.sig.output;
            let Some(syn::Type::Path(a)) = output else { continue; };
            let Some(seg) = a.path.segments.last() else {continue;};
            if seg.ident != "Result" {continue;};
            let syn::PathArguments::AngleBracketed(nested) = 
                seg.arguments else {continue;};
            let Some(syn::GenericArgument::Type(::syn::Type::Path(generic))) =
                nested.args.first() else {continue;};
            let Some(t) = generic.path.segments.last() else {continue;};
            if t.ident != "()" {continue;}

            methods.push(i);
        }
        
        //implement CommandModule for this struct
        let struct_name = a.path.segments.last().unwrap().ident;
        let mut expressions = vec![];
        for m in methods {
        }
        let init_method: syn::ImplItemMethod = syn::parse_quote!(
            fn init() -> Vec<Command> {
                let mut commands: Vec<Command> = vec![];
                commands.push(Command{
                    name: 
                })
            }
        );
        let mut trait_impl: syn::ItemImpl = syn::parse_quote!(
            impl CommandModule for #struct_name {

            }
        );
        trait_impl.items.push(init_method);
        quote!{
            #ast
            #trait_impl
        }.into()


    }
    else {
        panic!("Failed to resolve struct name!")
        
    }
}