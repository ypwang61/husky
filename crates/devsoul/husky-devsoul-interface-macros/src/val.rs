// todo: move to ml-task-macros
use super::*;
use quote::quote;

pub(crate) fn val(args: TokenStream, input: TokenStream) -> TokenStream {
    let ValArgs {
        ingredient_index,
        lazy,
        return_ref,
    } = syn::parse_macro_input!(args as ValArgs);
    let ItemFn {
        attrs: _,
        vis,
        sig:
            Signature {
                constness: _,
                asyncness: _,
                unsafety: _,
                abi: _,
                fn_token: _,
                ident,
                generics: _,
                paren_token: _,
                inputs: _,
                variadic: _,
                output,
            },
        block,
    } = syn::parse_macro_input!(input as syn::ItemFn);
    let ReturnType::Type(_, ref return_ty) = output else {
        unreachable!()
    };
    let aux_ident = Ident::new(&format!("__{}", ident), ident.span());
    if lazy {
        if return_ref {
            quote! {
                #vis fn #ident() -> Leash<#return_ty> {
                    todo!("return leash for lazy val, change the return type")
                    // __eval_lazy_val(#ingredient_index)
                }
            }
            .into()
        } else {
            quote! {
                #vis fn #ident() -> #return_ty {
                    todo!("return copied for lazy val")
                    // __eval_lazy_val(#ingredient_index)
                }
            }
            .into()
        }
    } else {
        if return_ref {
            quote! {
                #vis fn #ident() -> Leash<#return_ty> {
                    todo!("return leash for eager val, change the return type")
                    // __eval_eager_val_with(
                    //     #ingredient_index,
                    //     || __KiControlFlow::Continue(__ValueLeashTest(#aux_ident()).into_value())
                    // )
                }

                #vis fn #aux_ident() -> #return_ty #block
            }
            .into()
        } else {
            quote! {
                #vis fn #ident() -> #return_ty {
                    todo!("return copied for eager val")
                    // __eval_eager_val_with(
                    //     #ingredient_index,
                    //     || __KiControlFlow::Continue(__ValueLeashTest(#aux_ident()).into_value())
                    // )
                }

                #vis fn #aux_ident() -> #return_ty #block
            }
            .into()
        }
    }
}

struct ValArgs {
    ingredient_index: usize,
    // default false
    lazy: bool,
    // default false
    return_ref: bool,
}

impl syn::parse::Parse for ValArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: syn::Ident = syn::Ident::parse_any(input)?;
        assert!(ident == "ingredient_index");
        let _eq = Equals::parse(input)?;
        let lit = syn::LitInt::parse(input)?;
        let ingredient_index: usize = lit.base10_parse()?;
        let mut slf = Self {
            ingredient_index,
            lazy: false,
            return_ref: false,
        };
        loop {
            if input.is_empty() {
                return Ok(slf);
            }
            let _comma = Comma::parse(input)?;
            let ident: syn::Ident = syn::Ident::parse_any(input)?;
            if ident == "lazy" {
                assert!(!slf.lazy);
                slf.lazy = true
            } else if ident == "return_ref" {
                assert!(!slf.return_ref);
                slf.return_ref = true
            }
        }
    }
}
