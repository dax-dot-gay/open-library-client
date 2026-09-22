use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    Ident, Lit, LitInt, LitStr, Token, bracketed, parenthesized, parse::Parse,
    punctuated::Punctuated,
};

//open_library_client::wrapper::SOLRTerm::

#[derive(Clone, Debug)]
struct SOLRInput {
    pub cpath: syn::Path,
    pub solr: SOLRRoot,
}

impl Parse for SOLRInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(Token![crate]) && input.peek2(Token![=]) {
            let _ = input.parse::<Token![crate]>();
            let _ = input.parse::<Token![=]>();
            let path = input.parse::<syn::Path>()?;
            let _ = input.parse::<Token![,]>();
            Ok(Self {
                cpath: path,
                solr: SOLRRoot::parse(input)?,
            })
        } else {
            Ok(Self {
                cpath: syn::parse_quote!(open_library_client::wrapper),
                solr: SOLRRoot::parse(input)?,
            })
        }
    }
}

pub fn solr(input: TokenStream) -> syn::Result<TokenStream> {
    let SOLRInput { cpath, solr } = syn::parse2(input)?;
    let mut trees: Vec<TokenStream> = vec![];
    for tree in solr.0 {
        trees.push(parse_solr_tree(tree, false, cpath.clone())?);
    }
    Ok(quote! {
        #cpath::wrapper::SOLR(vec![#(#trees),*])
    })
}

fn parse_solr_tree(tree: SOLRTree, in_field: bool, cpath: syn::Path) -> syn::Result<TokenStream> {
    match tree {
        SOLRTree::Field {
            span,
            key,
            _colon,
            clause,
        } => {
            if in_field {
                Err(syn::Error::new(
                    span,
                    "Cannot query fields inside field clauses",
                ))
            } else {
                let key_name = key.to_string();
                let child = parse_solr_tree(*clause, true, cpath.clone())?;
                Ok(quote! {
                    #cpath::wrapper::SOLRTerm::Field {
                        key: #key_name.to_string(),
                        clause: Box::new(#child)
                    }
                })
            }
        }
        SOLRTree::Wildcard { span, _wildcard } => {
            if !in_field {
                Err(syn::Error::new(span, "Wildcards cannot be top-level"))
            } else {
                Ok(quote! {#cpath::wrapper::SOLRTerm::Wildcard {}})
            }
        }
        SOLRTree::Literal { value, .. } => {
            Ok(quote! {#cpath::wrapper::SOLRTerm::Literal {value: #value.to_string()}})
        }
        SOLRTree::LiteralPhrase { value, .. } => {
            let trimmed = value.value().trim_matches('\'').to_string();
            Ok(quote! {#cpath::wrapper::SOLRTerm::LiteralPhrase {value: #trimmed.to_string()}})
        }
        SOLRTree::LiteralProximity {
            value,
            _prox,
            proximity,
            ..
        } => {
            let trimmed = value.value().trim_matches('\'').to_string();
            Ok(
                quote! {#cpath::wrapper::SOLRTerm::LiteralProximity {value: #trimmed.to_string(), proximity: #proximity}},
            )
        }
        SOLRTree::Require { _plus, clause, .. } => {
            let child = parse_solr_tree(*clause, in_field, cpath.clone())?;
            Ok(quote! {#cpath::wrapper::SOLRTerm::Require {clause: Box::new(#child)}})
        }
        SOLRTree::Exclude { _minus, clause, .. } => {
            let child = parse_solr_tree(*clause, in_field, cpath.clone())?;
            Ok(quote! {#cpath::wrapper::SOLRTerm::Exclude {clause: Box::new(#child)}})
        }
        SOLRTree::CommaSequence {
            _paren, clauses, ..
        } => {
            let mut children: Vec<TokenStream> = vec![];
            for child in clauses {
                children.push(parse_solr_tree(child, in_field, cpath.clone())?);
            }

            Ok(quote! {#cpath::wrapper::SOLRTerm::Group {
                clauses: vec![#(#children),*],
                delimiter: #cpath::wrapper::SOLRGroupDelimiter::Space
            }})
        }
        SOLRTree::AndSequence {
            _paren, clauses, ..
        } => {
            let mut children: Vec<TokenStream> = vec![];
            for child in clauses {
                children.push(parse_solr_tree(child, in_field, cpath.clone())?);
            }

            Ok(quote! {#cpath::wrapper::SOLRTerm::Group {
                clauses: vec![#(#children),*],
                delimiter: #cpath::wrapper::SOLRGroupDelimiter::And
            }})
        }
        SOLRTree::OrSequence {
            _paren, clauses, ..
        } => {
            let mut children: Vec<TokenStream> = vec![];
            for child in clauses {
                children.push(parse_solr_tree(child, in_field, cpath.clone())?);
            }

            Ok(quote! {#cpath::wrapper::SOLRTerm::Group {
                clauses: vec![#(#children),*],
                delimiter: #cpath::wrapper::SOLRGroupDelimiter::Or
            }})
        }
        SOLRTree::Range {
            span,
            _bracket,
            from,
            _range,
            to,
        } => {
            if !in_field {
                Err(syn::Error::new(span, "Ranges cannot be top-level"))
            } else {
                let low = from
                    .map(|v| quote! {Some(#v.to_string())})
                    .unwrap_or(quote! {None});
                let high = to
                    .map(|v| quote! {Some(#v.to_string())})
                    .unwrap_or(quote! {None});
                Ok(quote! {#cpath::wrapper::SOLRTerm::Range {
                    low: #low,
                    high: #high
                }})
            }
        }
    }
}

#[derive(Clone, Debug)]
struct SOLRRoot(Punctuated<SOLRTree, Token![,]>);

impl Parse for SOLRRoot {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self(Punctuated::parse_terminated(&input)?))
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
enum SOLRTree {
    Field {
        span: Span,
        key: Ident,
        _colon: Token![:],
        clause: Box<SOLRTree>,
    },
    Wildcard {
        span: Span,
        _wildcard: Token![*],
    },
    Literal {
        span: Span,
        value: Lit,
    },
    LiteralPhrase {
        span: Span,
        value: LitStr,
    },
    LiteralProximity {
        span: Span,
        value: LitStr,
        _prox: Token![~],
        proximity: LitInt,
    },
    Require {
        span: Span,
        _plus: Token![+],
        clause: Box<SOLRTree>,
    },
    Exclude {
        span: Span,
        _minus: Token![-],
        clause: Box<SOLRTree>,
    },
    CommaSequence {
        span: Span,
        _paren: syn::token::Paren,
        clauses: Punctuated<SOLRTree, Token![,]>,
    },
    AndSequence {
        span: Span,
        _paren: syn::token::Paren,
        clauses: Punctuated<SOLRTree, Token![&]>,
    },
    OrSequence {
        span: Span,
        _paren: syn::token::Paren,
        clauses: Punctuated<SOLRTree, Token![|]>,
    },
    Range {
        span: Span,
        _bracket: syn::token::Bracket,
        from: Option<Lit>,
        _range: Token![..],
        to: Option<Lit>,
    },
}

impl Parse for SOLRTree {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let span = input.span();
        if input.peek(Ident) && input.peek2(Token![:]) {
            return Ok(SOLRTree::Field {
                span,
                key: input.parse()?,
                _colon: input.parse()?,
                clause: Box::new(input.parse()?),
            });
        }

        if input.peek(Token![*]) {
            return Ok(SOLRTree::Wildcard {
                span,
                _wildcard: input.parse()?,
            });
        }

        if input.peek(Lit) {
            let val: Lit = input.parse()?;
            if let Lit::Str(lstr) = val {
                if lstr.value().starts_with("'") && lstr.value().ends_with("'") {
                    if input.peek(Token![~]) && input.peek2(LitInt) {
                        return Ok(SOLRTree::LiteralProximity {
                            span,
                            value: lstr,
                            _prox: input.parse()?,
                            proximity: input.parse()?,
                        });
                    } else {
                        return Ok(SOLRTree::LiteralPhrase { span, value: lstr });
                    }
                } else {
                    if input.peek(Token![~]) && input.peek2(LitInt) {
                        return Ok(SOLRTree::LiteralProximity {
                            span,
                            value: lstr,
                            _prox: input.parse()?,
                            proximity: input.parse()?,
                        });
                    } else {
                        return Ok(SOLRTree::Literal {
                            span,
                            value: Lit::Str(lstr),
                        });
                    }
                }
            } else {
                return Ok(SOLRTree::Literal { span, value: val });
            }
        }

        if input.peek(syn::token::Bracket) {
            let content;
            let bracket = bracketed!(content in input);
            let from = if content.peek(Lit) {
                Some(content.parse::<Lit>()?)
            } else {
                None
            };
            let range = content.parse::<Token![..]>()?;
            let to = if content.peek(Lit) {
                Some(content.parse::<Lit>()?)
            } else {
                None
            };
            return Ok(SOLRTree::Range {
                span,
                _bracket: bracket,
                from,
                _range: range,
                to,
            });
        }

        if input.peek(Token![+]) {
            return Ok(SOLRTree::Require {
                span,
                _plus: input.parse()?,
                clause: Box::new(input.parse()?),
            });
        }

        if input.peek(Token![-]) {
            return Ok(SOLRTree::Exclude {
                span,
                _minus: input.parse()?,
                clause: Box::new(input.parse()?),
            });
        }

        if input.peek(syn::token::Paren) {
            let content;
            let parens = parenthesized!(content in input);
            let ahead = content.fork();
            if let Ok(_) = ahead.parse::<SOLRTree>() {
                if ahead.peek(Token![&]) {
                    return Ok(SOLRTree::AndSequence {
                        span,
                        _paren: parens,
                        clauses: Punctuated::parse_terminated(&content)?,
                    });
                } else if ahead.peek(Token![|]) {
                    return Ok(SOLRTree::OrSequence {
                        span,
                        _paren: parens,
                        clauses: Punctuated::parse_terminated(&content)?,
                    });
                } else {
                    return Ok(SOLRTree::CommaSequence {
                        span,
                        _paren: parens,
                        clauses: Punctuated::parse_terminated(&content)?,
                    });
                }
            } else {
                return Ok(SOLRTree::CommaSequence {
                    span,
                    _paren: parens,
                    clauses: Punctuated::new(),
                });
            }
        }

        Err(syn::Error::new(
            input.span(),
            "Unable to find a valid path from this input",
        ))
    }
}
