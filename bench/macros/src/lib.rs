use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{LitStr, Token};

/// Parses the inner part of main!("name", fn1, fn2) or main!(fn1, fn2)
struct MainArgs {
    name: Option<String>,
    benchmarks: Vec<syn::Path>,
}

impl syn::parse::Parse for MainArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = if input.peek(LitStr) {
            Some(input.parse::<LitStr>()?.value())
        } else {
            None
        };

        if name.is_some() && !input.is_empty() {
            input.parse::<Token![,]>()?;
        }

        let mut benchmarks = Vec::new();
        while !input.is_empty() {
            let benchmark = input.parse::<syn::Path>()?;
            benchmarks.push(benchmark);
            if input.is_empty() {
                break;
            }
            input.parse::<syn::Token![,]>()?;
        }

        Ok(MainArgs { name, benchmarks })
    }
}

#[proc_macro]
pub fn main(item: TokenStream) -> TokenStream {
    let main_args = match syn::parse::<MainArgs>(item) {
        Ok(args) => args,
        Err(err) => return err.to_compile_error().into(),
    };

    expand_main(main_args)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

fn expand_main(MainArgs { name, benchmarks }: MainArgs) -> syn::Result<proc_macro2::TokenStream> {
    let mut bench_runs = Vec::new();
    for mut bench in benchmarks {
        let bench_struct_path = {
            // Turn `add` to `Benchmark_add` and `adder::add` to `adder::Benchmark_add`
            let last_segment = bench.segments.last().unwrap();
            bench.segments.last_mut().unwrap().ident = syn::Ident::new(
                &format!("Benchmark_{}", last_segment.ident),
                last_segment.ident.span(),
            );
            bench
        };

        bench_runs.push(quote! {
            {
                let name = #bench_struct_path::name();
                if let Some(params) = #bench_struct_path::params() {
                    let (param_names, params): (Vec<String>, Vec<_>) = params.into_iter().unzip();
                    let params = param_names.iter().map(|n| n.as_str()).zip(params.into_iter()).collect::<Vec<_>>();
                    bench.benchmark_with(&name, params, |b, p| {
                        #bench_struct_path::run(b, p.clone());
                    });
                } else {
                    bench.benchmark(&name, None, |b| {
                        #bench_struct_path::run_without_param(b);
                    });
                }
            }
        });
    }

    let name = match name {
        Some(name) => syn::LitStr::new(&name, proc_macro2::Span::call_site()).into_token_stream(),
        None => quote! {
            core::module_path!()
        },
    };

    Ok(quote! {
        fn main() {
            extern crate benchy as _benchy;
            use _benchy::BenchmarkFn;

            let mut bench = _benchy::Benchmark::from_env(#name);

            #(#bench_runs)*

            bench.output();
        }
    })
}

/// Parses the inner part of:
/// - `#[benchmark!("name", [("param1", param1), ("param2", param2)])`
/// - or `#benchmark([("param1", param1), ("param2", param2)])`
/// - or `#[benchmark("name")]`
/// - or `#[benchmark]`
struct BenchMarkArgs {
    name: Option<String>,
    params: Option<syn::Expr>,
}

impl syn::parse::Parse for BenchMarkArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = match input.parse::<syn::LitStr>() {
            Ok(lit) => Some(lit.value()),
            Err(_) => None,
        };

        let params = if !input.is_empty() {
            if name.is_some() {
                input.parse::<syn::Token![,]>()?;
            }

            Some(input.parse::<syn::Expr>()?)
        } else {
            None
        };

        if !input.is_empty() {
            return Err(input.error("Unexpected input"));
        }

        Ok(BenchMarkArgs { name, params })
    }
}

#[proc_macro_attribute]
pub fn benchmark(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_benchmark(attr, item)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

fn expand_benchmark(attr: TokenStream, item: TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let attr = proc_macro2::TokenStream::from(attr);
    let BenchMarkArgs { name, params } = syn::parse::<BenchMarkArgs>(attr.into())?;
    let item_fn = syn::parse::<syn::ItemFn>(item)?;
    let fn_vis = &item_fn.vis;
    let fn_name = &item_fn.sig.ident;

    let (param_type, takes_param) = match item_fn.sig.inputs.iter().nth(1) {
        None => (quote! { () }, false),
        Some(syn::FnArg::Typed(pat_type)) => (pat_type.ty.to_token_stream(), true),
        Some(_) => {
            return Err(syn::Error::new_spanned(
                item_fn.sig.inputs,
                "Expected second parameter to be a typed pattern",
            ));
        }
    };

    let bench_struct_name = syn::Ident::new(&format!("Benchmark_{}", fn_name), fn_name.span());

    let params = match takes_param {
        true => {
            quote! { Some(#params.into_iter().map(|(name, value)| (name.into(), value)).collect()) }
        }
        false => quote! { None },
    };

    let name_impl = match name {
        Some(name) => quote! { #name.to_owned() },
        None => quote! { stringify!(#fn_name).to_owned() },
    };

    let run_impl = match takes_param {
        true => quote! {
            fn run(b: &mut _benchy::BenchmarkRun, p: Self::ParamType) {
                #fn_name(b, p);
            }
        },
        false => quote! {
            fn run_without_param(b: &mut _benchy::BenchmarkRun) {
                #fn_name(b);
            }
        },
    };

    Ok(quote! {
        #[allow(non_camel_case_types)]
        #fn_vis struct #bench_struct_name;

        #[allow(non_upper_case_globals)]
        const _: () = {
            extern crate benchy as _benchy;

            #[automatically_derived]
            impl _benchy::BenchmarkFn for #bench_struct_name {
                type ParamType = #param_type;

                fn name() -> String {
                    #name_impl
                }

                fn params() -> Option<Vec<(String, Self::ParamType)>> {
                    #params
                }

                #run_impl
            }
        };

        #item_fn
    })
}
