extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, ExprArray, LitInt, Result, Token};

mod kw {
    syn::custom_keyword!(opcode);
    syn::custom_keyword!(regs);
    syn::custom_keyword!(lenght);
    syn::custom_keyword!(cycles);
    syn::custom_keyword!(flags);
}

enum Argument {
    Opcode {
        opcode_token: kw::opcode,
        eq_token: Token![=],
        value: LitInt,
    },
    Regs {
        regs_token: kw::regs,
        eq_token: Token![=],
        value: ExprArray,
    },
    Lenght {
        lenght_token: kw::lenght,
        eq_token: Token![=],
        value: LitInt,
    },
    Cycles {
        cycles_token: kw::cycles,
        eq_token: Token![=],
        value: LitInt,
    },
    Flags {
        flags_token: kw::flags,
        eq_token: Token![=],
        value: ExprArray,
    },
}
impl Parse for Argument {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::opcode) {
            Ok(Argument::Opcode {
                opcode_token: input.parse::<kw::opcode>()?,
                eq_token: input.parse()?,
                value: input.parse()?,
            })
        } else if lookahead.peek(kw::regs) {
            Ok(Argument::Regs {
                regs_token: input.parse::<kw::regs>()?,
                eq_token: input.parse()?,
                value: input.parse()?,
            })
        } else if lookahead.peek(kw::lenght) {
            Ok(Argument::Lenght {
                lenght_token: input.parse::<kw::lenght>()?,
                eq_token: input.parse()?,
                value: input.parse()?,
            })
        } else if lookahead.peek(kw::cycles) {
            Ok(Argument::Cycles {
                cycles_token: input.parse::<kw::cycles>()?,
                eq_token: input.parse()?,
                value: input.parse()?,
            })
        } else if lookahead.peek(kw::flags) {
            Ok(Argument::Flags {
                flags_token: input.parse::<kw::flags>()?,
                eq_token: input.parse()?,
                value: input.parse()?,
            })
        } else {
            Err(lookahead.error())
        }
    }
}
/// #[declare_instruction(opcode=0xff, regs=[], lenght=2, cycles=8, flags=[]])
#[proc_macro_attribute]
pub fn declare_instruction(args: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as Argument);
    impl_declare_instruction(args, item)
}

fn impl_declare_instruction(arguments: Argument, item: TokenStream) -> TokenStream {
    let typename = quote! {item.to_string()};
    let args = quote! {arguments};
    let generated = quote! {
        pub struct #typename {
            opcode: #args.opcode.value,
            regs: #args.regs.value,
            lenght: #args.lenght.value,
            cycles: #args.cycles.value,
            flags:  #args.flags.value,
        }
    };

    TokenStream::from(generated)
}
