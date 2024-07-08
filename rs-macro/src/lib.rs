use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemFn};

#[proc_macro_derive(HelloDerive)]
pub fn hello_derive(input: TokenStream) -> TokenStream {
    // 解析输入的 TokenStream
    let input = parse_macro_input!(input as DeriveInput);

    // 获取结构体的名称
    let name = &input.ident;

    // println 无法输出日志

    // cargo check 后，编译器会输出如下信息
    // eprintln!("Deriving HelloWorld for {}", name);
    // dbg!(&name);

    // 生成特征实现代码
    let expanded = quote! {
        impl HelloWorld for #name {
            fn hello_world() {
                println!("Hello, World! My name is {}.", stringify!(#name));
            }
        }
    };

    // 将生成的代码转换为 TokenStream 并返回
    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn hello_attr(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // 解析输入的 TokenStream
    let input = parse_macro_input!(item as ItemFn);

    // 获取函数名
    let fn_name = &input.sig.ident;

    // 生成一个新的函数名
    let new_fn_name = syn::Ident::new(&format!("_{}", fn_name), fn_name.span());

    // 生成新的函数体
    let expanded = quote! {
        #input

        fn #new_fn_name() {
            println!("Function {} is called with my_attribute.", stringify!(#fn_name));
            #fn_name();
        }

        // fn myfn() {
        //     println!("Function {} is called with my_attribute.", stringify!(#fn_name));
        //     #fn_name();
        // }
    };

    // 将生成的代码转换为 TokenStream 并返回
    TokenStream::from(expanded)
}

#[proc_macro]
pub fn add(input: TokenStream) -> TokenStream {
    // 解析输入的 TokenStream
    let input = parse_macro_input!(input as syn::ExprBinary);

    // 获取两个操作数
    let left = match &*input.left {
        syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(lit_int), .. }) => lit_int.base10_parse::<i32>().unwrap(),
        _ => panic!("Left operand must be an integer literal"),
    };

    let right = match &*input.right {
        syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(lit_int), .. }) => lit_int.base10_parse::<i32>().unwrap(),
        _ => panic!("Right operand must be an integer literal"),
    };

    // 计算结果
    let result = left + right;

    // 生成代码
    let expanded = quote! {
        #result
    };

    // 将生成的代码转换为 TokenStream 并返回
    TokenStream::from(expanded)
}
