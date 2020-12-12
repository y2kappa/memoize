use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{AttributeArgs, ItemFn};

#[proc_macro_attribute]
pub fn memoize(args: TokenStream, input: TokenStream) -> TokenStream {
    let _options = syn::parse_macro_input!(args as AttributeArgs);
    let function = syn::parse_macro_input!(input as ItemFn);

    // Parse function sig
    let ItemFn {
        attrs,
        vis,
        sig,
        block: body,
        ..
    } = &function;

    println!("Sig {:?}", sig.output);

    let mut args = vec![];
    sig.inputs.iter().for_each(|x| match x {
        syn::FnArg::Typed(x) => match x.pat.as_ref() {
            syn::Pat::Ident(x) => args.push(x.ident.clone()),
            _ => {}
        },
        _ => {}
    });

    let return_type = match &sig.output {
        syn::ReturnType::Type(_, x) => Some(x),
        _ => None,
    };

    let format = args
        .iter()
        .map(|_| "{}".to_string())
        .collect::<Vec<String>>()
        .join("-");

    let result = quote! {

        use std::collections::HashMap;
        use std::sync::Mutex;
        lazy_static! {
            static ref CACHE : Mutex<HashMap<String, #return_type >> = Mutex::new(HashMap::new());
        }

        #(#attrs)*
        #vis #sig {

            let key = format!(#format, #(#args),*);
            let mut map = CACHE.lock().unwrap();
            if let Some(output) = map.get(&key) {
                println!("Loaded {:?} from cache with value {:?}", key, output);
                output.clone()
            } else {
                let output = { #body };
                map.insert(key.clone(), output.clone());
                println!("Calculated {:?} with value {:?}", key, output);
                output
            }
        }
    };

    let res: proc_macro::TokenStream = result.into();
    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use std::collections::HashMap;
        let mut m: HashMap<i32, i32> = HashMap::new();
        if let Some(x) = m.get(&2) {
            println!("Found {}", x);
        } else {
            m.insert(1, 2);
        }
    }
}
