#![allow(unused_imports)]
#![allow(unused_variables)]
use type_name::TypeName;
use field_name::FieldName;
use schema::*;
use parse::{Document, Definition, OperationType};
use parse::tok::{self, Tok};
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__Document {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use type_name::TypeName;
    use field_name::FieldName;
    use schema::*;
    use parse::{Document, Definition, OperationType};
    use parse::tok::{self, Tok};
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    use super::__ToTriple;
    pub fn parse_Document<
        'input,
        __TOKEN: __ToTriple<'input, Error=tok::Error>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        text: &'input str,
        __tokens: __TOKENS,
    ) -> Result<Document, __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let __tokens = __tokens.into_iter();
        let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match try!(__state0(text, &mut __tokens, __lookahead)) {
            (Some(__lookahead), _) => {
                Err(__ParseError::ExtraToken { token: __lookahead })
            }
            (None, __Nonterminal::____Document((_, __nt, _))) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<'input> {
        _28Definition_29((usize, Definition, usize)),
        _28Definition_29_2b((usize, ::std::vec::Vec<Definition>, usize)),
        _28FieldDefinition_29((usize, Field, usize)),
        _28FieldDefinition_29_2b((usize, ::std::vec::Vec<Field>, usize)),
        _28ImplementsInterfaces_29((usize, Vec<TypeName>, usize)),
        _28ImplementsInterfaces_29_3f((usize, ::std::option::Option<Vec<TypeName>>, usize)),
        _28OperationTypeDefinition_29((usize, OperationType, usize)),
        _28OperationTypeDefinition_29_2b((usize, ::std::vec::Vec<OperationType>, usize)),
        _28TypeName_29((usize, TypeName, usize)),
        _28TypeName_29_2b((usize, ::std::vec::Vec<TypeName>, usize)),
        Definition((usize, Definition, usize)),
        Document((usize, Document, usize)),
        FieldDefinition((usize, Field, usize)),
        FieldName((usize, FieldName, usize)),
        Identifier((usize, &'input str, usize)),
        ImplementsInterfaces((usize, Vec<TypeName>, usize)),
        ObjectTypeDefinition((usize, Definition, usize)),
        OperationTypeDefinition((usize, OperationType, usize)),
        SchemaDefinition((usize, Definition, usize)),
        TypeDefinition((usize, Definition, usize)),
        TypeName((usize, TypeName, usize)),
        TypeSystemDefinition((usize, Definition, usize)),
        ____Document((usize, Document, usize)),
    }

    pub fn __state0<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok<'input>, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::Schema, __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state8(text, __tokens, __sym0));
            }
            Some((__loc1, __tok @ Tok::Type, __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state9(text, __tokens, __sym0));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28Definition_29_2b(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(text, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Definition(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state2(text, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::Document(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state3(text, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::ObjectTypeDefinition(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state4(text, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::SchemaDefinition(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state5(text, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::TypeDefinition(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state6(text, __tokens, __lookahead, __sym0));
                }
                __Nonterminal::TypeSystemDefinition(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state7(text, __tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    pub fn __state1<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok<'input>, usize)>,
        __sym0: &mut Option<(usize, ::std::vec::Vec<Definition>, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::Schema, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state8(text, __tokens, __sym1));
            }
            Some((__loc1, __tok @ Tok::Type, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state9(text, __tokens, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1(text, __sym0);
                let __nt = __Nonterminal::Document((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Definition(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state10(text, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::ObjectTypeDefinition(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state4(text, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::SchemaDefinition(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state5(text, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::TypeDefinition(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state6(text, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::TypeSystemDefinition(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state7(text, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state2<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok<'input>, usize)>,
        __sym0: &mut Option<(usize, Definition, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, Tok::Schema, _)) |
            Some((_, Tok::Type, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action36(text, __sym0);
                let __nt = __Nonterminal::_28Definition_29_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state3<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok<'input>, usize)>,
        __sym0: &mut Option<(usize, Document, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(text, __sym0);
                let __nt = __Nonterminal::____Document((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state4<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok<'input>, usize)>,
        __sym0: &mut Option<(usize, Definition, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, Tok::Schema, _)) |
            Some((_, Tok::Type, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17(text, __sym0);
                let __nt = __Nonterminal::TypeDefinition((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state5<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok<'input>, usize)>,
        __sym0: &mut Option<(usize, Definition, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, Tok::Schema, _)) |
            Some((_, Tok::Type, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3(text, __sym0);
                let __nt = __Nonterminal::TypeSystemDefinition((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state6<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok<'input>, usize)>,
        __sym0: &mut Option<(usize, Definition, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, Tok::Schema, _)) |
            Some((_, Tok::Type, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4(text, __sym0);
                let __nt = __Nonterminal::TypeSystemDefinition((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state7<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok<'input>, usize)>,
        __sym0: &mut Option<(usize, Definition, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, Tok::Schema, _)) |
            Some((_, Tok::Type, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2(text, __sym0);
                let __nt = __Nonterminal::Definition((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state8<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, Tok<'input>, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((__loc1, __tok @ Tok::LBrace, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state11(text, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state9<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, Tok<'input>, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((__loc1, Tok::Identifier(__tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state14(text, __tokens, __sym1));
            }
            Some((__loc1, __tok @ Tok::Implements, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state15(text, __tokens, __sym1));
            }
            Some((__loc1, __tok @ Tok::Mutation, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state16(text, __tokens, __sym1));
            }
            Some((__loc1, __tok @ Tok::On, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state17(text, __tokens, __sym1));
            }
            Some((__loc1, __tok @ Tok::Query, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state18(text, __tokens, __sym1));
            }
            Some((__loc1, __tok @ Tok::Schema, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state19(text, __tokens, __sym1));
            }
            Some((__loc1, __tok @ Tok::Type, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state20(text, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Identifier(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state12(text, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::TypeName(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state13(text, __tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state10<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok<'input>, usize)>,
        __sym0: &mut Option<(usize, ::std::vec::Vec<Definition>, usize)>,
        __sym1: &mut Option<(usize, Definition, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        match __lookahead {
            None |
            Some((_, Tok::Schema, _)) |
            Some((_, Tok::Type, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action37(text, __sym0, __sym1);
                let __nt = __Nonterminal::_28Definition_29_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state11<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, Tok<'input>, usize)>,
        __sym1: &mut Option<(usize, Tok<'input>, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((__loc1, __tok @ Tok::Mutation, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state23(text, __tokens, __sym2));
            }
            Some((__loc1, __tok @ Tok::Query, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state24(text, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28OperationTypeDefinition_29_2b(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state21(text, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::OperationTypeDefinition(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state22(text, __tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state12<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok<'input>, usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, Tok::Identifier(_), _)) |
            Some((_, Tok::Implements, _)) |
            Some((_, Tok::Mutation, _)) |
            Some((_, Tok::On, _)) |
            Some((_, Tok::Query, _)) |
            Some((_, Tok::Schema, _)) |
            Some((_, Tok::Type, _)) |
            Some((_, Tok::LBrace, _)) |
            Some((_, Tok::RBrace, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8(text, __sym0);
                let __nt = __Nonterminal::TypeName((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state13<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok<'input>, usize)>,
        __sym0: &mut Option<(usize, Tok<'input>, usize)>,
        __sym1: &mut Option<(usize, TypeName, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::Implements, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state26(text, __tokens, __sym2));
            }
            Some((__loc1, __tok @ Tok::LBrace, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state27(text, __tokens, __sym0, __sym1, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::ImplementsInterfaces(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state25(text, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state14<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, Tok::Colon, _)) |
            Some((_, Tok::Identifier(_), _)) |
            Some((_, Tok::Implements, _)) |
            Some((_, Tok::Mutation, _)) |
            Some((_, Tok::On, _)) |
            Some((_, Tok::Query, _)) |
            Some((_, Tok::Schema, _)) |
            Some((_, Tok::Type, _)) |
            Some((_, Tok::LBrace, _)) |
            Some((_, Tok::RBrace, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16(text, __sym0);
                let __nt = __Nonterminal::Identifier((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state15<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, Tok<'input>, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, Tok::Colon, _)) |
            Some((_, Tok::Identifier(_), _)) |
            Some((_, Tok::Implements, _)) |
            Some((_, Tok::Mutation, _)) |
            Some((_, Tok::On, _)) |
            Some((_, Tok::Query, _)) |
            Some((_, Tok::Schema, _)) |
            Some((_, Tok::Type, _)) |
            Some((_, Tok::LBrace, _)) |
            Some((_, Tok::RBrace, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14(text, __sym0);
                let __nt = __Nonterminal::Identifier((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state16<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, Tok<'input>, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, Tok::Colon, _)) |
            Some((_, Tok::Identifier(_), _)) |
            Some((_, Tok::Implements, _)) |
            Some((_, Tok::Mutation, _)) |
            Some((_, Tok::On, _)) |
            Some((_, Tok::Query, _)) |
            Some((_, Tok::Schema, _)) |
            Some((_, Tok::Type, _)) |
            Some((_, Tok::LBrace, _)) |
            Some((_, Tok::RBrace, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12(text, __sym0);
                let __nt = __Nonterminal::Identifier((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state17<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, Tok<'input>, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, Tok::Colon, _)) |
            Some((_, Tok::Identifier(_), _)) |
            Some((_, Tok::Implements, _)) |
            Some((_, Tok::Mutation, _)) |
            Some((_, Tok::On, _)) |
            Some((_, Tok::Query, _)) |
            Some((_, Tok::Schema, _)) |
            Some((_, Tok::Type, _)) |
            Some((_, Tok::LBrace, _)) |
            Some((_, Tok::RBrace, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15(text, __sym0);
                let __nt = __Nonterminal::Identifier((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state18<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, Tok<'input>, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, Tok::Colon, _)) |
            Some((_, Tok::Identifier(_), _)) |
            Some((_, Tok::Implements, _)) |
            Some((_, Tok::Mutation, _)) |
            Some((_, Tok::On, _)) |
            Some((_, Tok::Query, _)) |
            Some((_, Tok::Schema, _)) |
            Some((_, Tok::Type, _)) |
            Some((_, Tok::LBrace, _)) |
            Some((_, Tok::RBrace, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11(text, __sym0);
                let __nt = __Nonterminal::Identifier((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state19<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, Tok<'input>, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, Tok::Colon, _)) |
            Some((_, Tok::Identifier(_), _)) |
            Some((_, Tok::Implements, _)) |
            Some((_, Tok::Mutation, _)) |
            Some((_, Tok::On, _)) |
            Some((_, Tok::Query, _)) |
            Some((_, Tok::Schema, _)) |
            Some((_, Tok::Type, _)) |
            Some((_, Tok::LBrace, _)) |
            Some((_, Tok::RBrace, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10(text, __sym0);
                let __nt = __Nonterminal::Identifier((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state20<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, Tok<'input>, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, Tok::Colon, _)) |
            Some((_, Tok::Identifier(_), _)) |
            Some((_, Tok::Implements, _)) |
            Some((_, Tok::Mutation, _)) |
            Some((_, Tok::On, _)) |
            Some((_, Tok::Query, _)) |
            Some((_, Tok::Schema, _)) |
            Some((_, Tok::Type, _)) |
            Some((_, Tok::LBrace, _)) |
            Some((_, Tok::RBrace, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13(text, __sym0);
                let __nt = __Nonterminal::Identifier((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state21<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok<'input>, usize)>,
        __sym0: &mut Option<(usize, Tok<'input>, usize)>,
        __sym1: &mut Option<(usize, Tok<'input>, usize)>,
        __sym2: &mut Option<(usize, ::std::vec::Vec<OperationType>, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::Mutation, __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state23(text, __tokens, __sym3));
            }
            Some((__loc1, __tok @ Tok::Query, __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state24(text, __tokens, __sym3));
            }
            Some((__loc1, __tok @ Tok::RBrace, __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state29(text, __tokens, __sym0, __sym1, __sym2, __sym3));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::OperationTypeDefinition(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state28(text, __tokens, __lookahead, __sym2, __sym3));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state22<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok<'input>, usize)>,
        __sym0: &mut Option<(usize, OperationType, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, Tok::Mutation, _)) |
            Some((_, Tok::Query, _)) |
            Some((_, Tok::RBrace, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action43(text, __sym0);
                let __nt = __Nonterminal::_28OperationTypeDefinition_29_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state23<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, Tok<'input>, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((__loc1, __tok @ Tok::Colon, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state30(text, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state24<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, Tok<'input>, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((__loc1, __tok @ Tok::Colon, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state31(text, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state25<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok<'input>, usize)>,
        __sym0: &mut Option<(usize, Tok<'input>, usize)>,
        __sym1: &mut Option<(usize, TypeName, usize)>,
        __sym2: &mut Option<(usize, Vec<TypeName>, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::LBrace, __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state32(text, __tokens, __sym0, __sym1, __sym2, __sym3));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state26<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, Tok<'input>, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((__loc1, Tok::Identifier(__tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state14(text, __tokens, __sym1));
            }
            Some((__loc1, __tok @ Tok::Implements, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state15(text, __tokens, __sym1));
            }
            Some((__loc1, __tok @ Tok::Mutation, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state16(text, __tokens, __sym1));
            }
            Some((__loc1, __tok @ Tok::On, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state17(text, __tokens, __sym1));
            }
            Some((__loc1, __tok @ Tok::Query, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state18(text, __tokens, __sym1));
            }
            Some((__loc1, __tok @ Tok::Schema, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state19(text, __tokens, __sym1));
            }
            Some((__loc1, __tok @ Tok::Type, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state20(text, __tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28TypeName_29_2b(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state33(text, __tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Identifier(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state12(text, __tokens, __lookahead, __sym1));
                }
                __Nonterminal::TypeName(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state34(text, __tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state27<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, Tok<'input>, usize)>,
        __sym1: &mut Option<(usize, TypeName, usize)>,
        __sym2: &mut Option<(usize, Tok<'input>, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((__loc1, Tok::Identifier(__tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state14(text, __tokens, __sym3));
            }
            Some((__loc1, __tok @ Tok::Implements, __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state15(text, __tokens, __sym3));
            }
            Some((__loc1, __tok @ Tok::Mutation, __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state16(text, __tokens, __sym3));
            }
            Some((__loc1, __tok @ Tok::On, __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state17(text, __tokens, __sym3));
            }
            Some((__loc1, __tok @ Tok::Query, __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state18(text, __tokens, __sym3));
            }
            Some((__loc1, __tok @ Tok::Schema, __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state19(text, __tokens, __sym3));
            }
            Some((__loc1, __tok @ Tok::Type, __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state20(text, __tokens, __sym3));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28FieldDefinition_29_2b(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state35(text, __tokens, __lookahead, __sym0, __sym1, __sym2, __sym3));
                }
                __Nonterminal::FieldDefinition(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state36(text, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::FieldName(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state37(text, __tokens, __lookahead, __sym3));
                }
                __Nonterminal::Identifier(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state38(text, __tokens, __lookahead, __sym3));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state28<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok<'input>, usize)>,
        __sym0: &mut Option<(usize, ::std::vec::Vec<OperationType>, usize)>,
        __sym1: &mut Option<(usize, OperationType, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, Tok::Mutation, _)) |
            Some((_, Tok::Query, _)) |
            Some((_, Tok::RBrace, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action44(text, __sym0, __sym1);
                let __nt = __Nonterminal::_28OperationTypeDefinition_29_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state29<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, Tok<'input>, usize)>,
        __sym1: &mut Option<(usize, Tok<'input>, usize)>,
        __sym2: &mut Option<(usize, ::std::vec::Vec<OperationType>, usize)>,
        __sym3: &mut Option<(usize, Tok<'input>, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            None |
            Some((_, Tok::Schema, _)) |
            Some((_, Tok::Type, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action5(text, __sym0, __sym1, __sym2, __sym3);
                let __nt = __Nonterminal::SchemaDefinition((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state30<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, Tok<'input>, usize)>,
        __sym1: &mut Option<(usize, Tok<'input>, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((__loc1, Tok::Identifier(__tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state14(text, __tokens, __sym2));
            }
            Some((__loc1, __tok @ Tok::Implements, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state15(text, __tokens, __sym2));
            }
            Some((__loc1, __tok @ Tok::Mutation, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state16(text, __tokens, __sym2));
            }
            Some((__loc1, __tok @ Tok::On, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state17(text, __tokens, __sym2));
            }
            Some((__loc1, __tok @ Tok::Query, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state18(text, __tokens, __sym2));
            }
            Some((__loc1, __tok @ Tok::Schema, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state19(text, __tokens, __sym2));
            }
            Some((__loc1, __tok @ Tok::Type, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state20(text, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Identifier(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state12(text, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::TypeName(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state39(text, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state31<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, Tok<'input>, usize)>,
        __sym1: &mut Option<(usize, Tok<'input>, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((__loc1, Tok::Identifier(__tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state14(text, __tokens, __sym2));
            }
            Some((__loc1, __tok @ Tok::Implements, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state15(text, __tokens, __sym2));
            }
            Some((__loc1, __tok @ Tok::Mutation, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state16(text, __tokens, __sym2));
            }
            Some((__loc1, __tok @ Tok::On, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state17(text, __tokens, __sym2));
            }
            Some((__loc1, __tok @ Tok::Query, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state18(text, __tokens, __sym2));
            }
            Some((__loc1, __tok @ Tok::Schema, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state19(text, __tokens, __sym2));
            }
            Some((__loc1, __tok @ Tok::Type, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state20(text, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Identifier(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state12(text, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::TypeName(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state40(text, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state32<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, Tok<'input>, usize)>,
        __sym1: &mut Option<(usize, TypeName, usize)>,
        __sym2: &mut Option<(usize, Vec<TypeName>, usize)>,
        __sym3: &mut Option<(usize, Tok<'input>, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((__loc1, Tok::Identifier(__tok0), __loc2)) => {
                let mut __sym4 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state14(text, __tokens, __sym4));
            }
            Some((__loc1, __tok @ Tok::Implements, __loc2)) => {
                let mut __sym4 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state15(text, __tokens, __sym4));
            }
            Some((__loc1, __tok @ Tok::Mutation, __loc2)) => {
                let mut __sym4 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state16(text, __tokens, __sym4));
            }
            Some((__loc1, __tok @ Tok::On, __loc2)) => {
                let mut __sym4 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state17(text, __tokens, __sym4));
            }
            Some((__loc1, __tok @ Tok::Query, __loc2)) => {
                let mut __sym4 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state18(text, __tokens, __sym4));
            }
            Some((__loc1, __tok @ Tok::Schema, __loc2)) => {
                let mut __sym4 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state19(text, __tokens, __sym4));
            }
            Some((__loc1, __tok @ Tok::Type, __loc2)) => {
                let mut __sym4 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state20(text, __tokens, __sym4));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym3.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::_28FieldDefinition_29_2b(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state41(text, __tokens, __lookahead, __sym0, __sym1, __sym2, __sym3, __sym4));
                }
                __Nonterminal::FieldDefinition(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state36(text, __tokens, __lookahead, __sym4));
                }
                __Nonterminal::FieldName(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state37(text, __tokens, __lookahead, __sym4));
                }
                __Nonterminal::Identifier(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state38(text, __tokens, __lookahead, __sym4));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state33<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok<'input>, usize)>,
        __sym0: &mut Option<(usize, Tok<'input>, usize)>,
        __sym1: &mut Option<(usize, ::std::vec::Vec<TypeName>, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, Tok::Identifier(__tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state14(text, __tokens, __sym2));
            }
            Some((__loc1, __tok @ Tok::Implements, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state15(text, __tokens, __sym2));
            }
            Some((__loc1, __tok @ Tok::Mutation, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state16(text, __tokens, __sym2));
            }
            Some((__loc1, __tok @ Tok::On, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state17(text, __tokens, __sym2));
            }
            Some((__loc1, __tok @ Tok::Query, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state18(text, __tokens, __sym2));
            }
            Some((__loc1, __tok @ Tok::Schema, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state19(text, __tokens, __sym2));
            }
            Some((__loc1, __tok @ Tok::Type, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state20(text, __tokens, __sym2));
            }
            Some((_, Tok::LBrace, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action19(text, __sym0, __sym1);
                let __nt = __Nonterminal::ImplementsInterfaces((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Identifier(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state12(text, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::TypeName(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state42(text, __tokens, __lookahead, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state34<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok<'input>, usize)>,
        __sym0: &mut Option<(usize, TypeName, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, Tok::Identifier(_), _)) |
            Some((_, Tok::Implements, _)) |
            Some((_, Tok::Mutation, _)) |
            Some((_, Tok::On, _)) |
            Some((_, Tok::Query, _)) |
            Some((_, Tok::Schema, _)) |
            Some((_, Tok::Type, _)) |
            Some((_, Tok::LBrace, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action45(text, __sym0);
                let __nt = __Nonterminal::_28TypeName_29_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state35<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok<'input>, usize)>,
        __sym0: &mut Option<(usize, Tok<'input>, usize)>,
        __sym1: &mut Option<(usize, TypeName, usize)>,
        __sym2: &mut Option<(usize, Tok<'input>, usize)>,
        __sym3: &mut Option<(usize, ::std::vec::Vec<Field>, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, Tok::Identifier(__tok0), __loc2)) => {
                let mut __sym4 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state14(text, __tokens, __sym4));
            }
            Some((__loc1, __tok @ Tok::Implements, __loc2)) => {
                let mut __sym4 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state15(text, __tokens, __sym4));
            }
            Some((__loc1, __tok @ Tok::Mutation, __loc2)) => {
                let mut __sym4 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state16(text, __tokens, __sym4));
            }
            Some((__loc1, __tok @ Tok::On, __loc2)) => {
                let mut __sym4 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state17(text, __tokens, __sym4));
            }
            Some((__loc1, __tok @ Tok::Query, __loc2)) => {
                let mut __sym4 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state18(text, __tokens, __sym4));
            }
            Some((__loc1, __tok @ Tok::Schema, __loc2)) => {
                let mut __sym4 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state19(text, __tokens, __sym4));
            }
            Some((__loc1, __tok @ Tok::Type, __loc2)) => {
                let mut __sym4 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state20(text, __tokens, __sym4));
            }
            Some((__loc1, __tok @ Tok::RBrace, __loc2)) => {
                let mut __sym4 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state44(text, __tokens, __sym0, __sym1, __sym2, __sym3, __sym4));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym3.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::FieldDefinition(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state43(text, __tokens, __lookahead, __sym3, __sym4));
                }
                __Nonterminal::FieldName(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state37(text, __tokens, __lookahead, __sym4));
                }
                __Nonterminal::Identifier(__nt) => {
                    let __sym4 = &mut Some(__nt);
                    __result = try!(__state38(text, __tokens, __lookahead, __sym4));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state36<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok<'input>, usize)>,
        __sym0: &mut Option<(usize, Field, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, Tok::Identifier(_), _)) |
            Some((_, Tok::Implements, _)) |
            Some((_, Tok::Mutation, _)) |
            Some((_, Tok::On, _)) |
            Some((_, Tok::Query, _)) |
            Some((_, Tok::Schema, _)) |
            Some((_, Tok::Type, _)) |
            Some((_, Tok::RBrace, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38(text, __sym0);
                let __nt = __Nonterminal::_28FieldDefinition_29_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state37<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok<'input>, usize)>,
        __sym0: &mut Option<(usize, FieldName, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::Colon, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state45(text, __tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state38<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok<'input>, usize)>,
        __sym0: &mut Option<(usize, &'input str, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, Tok::Colon, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9(text, __sym0);
                let __nt = __Nonterminal::FieldName((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state39<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok<'input>, usize)>,
        __sym0: &mut Option<(usize, Tok<'input>, usize)>,
        __sym1: &mut Option<(usize, Tok<'input>, usize)>,
        __sym2: &mut Option<(usize, TypeName, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, Tok::Mutation, _)) |
            Some((_, Tok::Query, _)) |
            Some((_, Tok::RBrace, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action7(text, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::OperationTypeDefinition((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state40<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok<'input>, usize)>,
        __sym0: &mut Option<(usize, Tok<'input>, usize)>,
        __sym1: &mut Option<(usize, Tok<'input>, usize)>,
        __sym2: &mut Option<(usize, TypeName, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, Tok::Mutation, _)) |
            Some((_, Tok::Query, _)) |
            Some((_, Tok::RBrace, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action6(text, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::OperationTypeDefinition((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state41<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok<'input>, usize)>,
        __sym0: &mut Option<(usize, Tok<'input>, usize)>,
        __sym1: &mut Option<(usize, TypeName, usize)>,
        __sym2: &mut Option<(usize, Vec<TypeName>, usize)>,
        __sym3: &mut Option<(usize, Tok<'input>, usize)>,
        __sym4: &mut Option<(usize, ::std::vec::Vec<Field>, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((__loc1, Tok::Identifier(__tok0), __loc2)) => {
                let mut __sym5 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state14(text, __tokens, __sym5));
            }
            Some((__loc1, __tok @ Tok::Implements, __loc2)) => {
                let mut __sym5 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state15(text, __tokens, __sym5));
            }
            Some((__loc1, __tok @ Tok::Mutation, __loc2)) => {
                let mut __sym5 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state16(text, __tokens, __sym5));
            }
            Some((__loc1, __tok @ Tok::On, __loc2)) => {
                let mut __sym5 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state17(text, __tokens, __sym5));
            }
            Some((__loc1, __tok @ Tok::Query, __loc2)) => {
                let mut __sym5 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state18(text, __tokens, __sym5));
            }
            Some((__loc1, __tok @ Tok::Schema, __loc2)) => {
                let mut __sym5 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state19(text, __tokens, __sym5));
            }
            Some((__loc1, __tok @ Tok::Type, __loc2)) => {
                let mut __sym5 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state20(text, __tokens, __sym5));
            }
            Some((__loc1, __tok @ Tok::RBrace, __loc2)) => {
                let mut __sym5 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state46(text, __tokens, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym4.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::FieldDefinition(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state43(text, __tokens, __lookahead, __sym4, __sym5));
                }
                __Nonterminal::FieldName(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state37(text, __tokens, __lookahead, __sym5));
                }
                __Nonterminal::Identifier(__nt) => {
                    let __sym5 = &mut Some(__nt);
                    __result = try!(__state38(text, __tokens, __lookahead, __sym5));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state42<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok<'input>, usize)>,
        __sym0: &mut Option<(usize, ::std::vec::Vec<TypeName>, usize)>,
        __sym1: &mut Option<(usize, TypeName, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, Tok::Identifier(_), _)) |
            Some((_, Tok::Implements, _)) |
            Some((_, Tok::Mutation, _)) |
            Some((_, Tok::On, _)) |
            Some((_, Tok::Query, _)) |
            Some((_, Tok::Schema, _)) |
            Some((_, Tok::Type, _)) |
            Some((_, Tok::LBrace, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action46(text, __sym0, __sym1);
                let __nt = __Nonterminal::_28TypeName_29_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state43<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok<'input>, usize)>,
        __sym0: &mut Option<(usize, ::std::vec::Vec<Field>, usize)>,
        __sym1: &mut Option<(usize, Field, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, Tok::Identifier(_), _)) |
            Some((_, Tok::Implements, _)) |
            Some((_, Tok::Mutation, _)) |
            Some((_, Tok::On, _)) |
            Some((_, Tok::Query, _)) |
            Some((_, Tok::Schema, _)) |
            Some((_, Tok::Type, _)) |
            Some((_, Tok::RBrace, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action39(text, __sym0, __sym1);
                let __nt = __Nonterminal::_28FieldDefinition_29_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state44<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, Tok<'input>, usize)>,
        __sym1: &mut Option<(usize, TypeName, usize)>,
        __sym2: &mut Option<(usize, Tok<'input>, usize)>,
        __sym3: &mut Option<(usize, ::std::vec::Vec<Field>, usize)>,
        __sym4: &mut Option<(usize, Tok<'input>, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            None |
            Some((_, Tok::Schema, _)) |
            Some((_, Tok::Type, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __sym4 = __sym4.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action42(text, __sym0, __sym1, __sym2, __sym3, __sym4);
                let __nt = __Nonterminal::ObjectTypeDefinition((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state45<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, FieldName, usize)>,
        __sym1: &mut Option<(usize, Tok<'input>, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((__loc1, Tok::Identifier(__tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state14(text, __tokens, __sym2));
            }
            Some((__loc1, __tok @ Tok::Implements, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state15(text, __tokens, __sym2));
            }
            Some((__loc1, __tok @ Tok::Mutation, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state16(text, __tokens, __sym2));
            }
            Some((__loc1, __tok @ Tok::On, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state17(text, __tokens, __sym2));
            }
            Some((__loc1, __tok @ Tok::Query, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state18(text, __tokens, __sym2));
            }
            Some((__loc1, __tok @ Tok::Schema, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state19(text, __tokens, __sym2));
            }
            Some((__loc1, __tok @ Tok::Type, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state20(text, __tokens, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Identifier(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state12(text, __tokens, __lookahead, __sym2));
                }
                __Nonterminal::TypeName(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state47(text, __tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state46<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(usize, Tok<'input>, usize)>,
        __sym1: &mut Option<(usize, TypeName, usize)>,
        __sym2: &mut Option<(usize, Vec<TypeName>, usize)>,
        __sym3: &mut Option<(usize, Tok<'input>, usize)>,
        __sym4: &mut Option<(usize, ::std::vec::Vec<Field>, usize)>,
        __sym5: &mut Option<(usize, Tok<'input>, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            None |
            Some((_, Tok::Schema, _)) |
            Some((_, Tok::Type, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __sym4 = __sym4.take().unwrap();
                let __sym5 = __sym5.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action41(text, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __nt = __Nonterminal::ObjectTypeDefinition((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state47<
        'input,
        __TOKENS: Iterator<Item=Result<(usize, Tok<'input>, usize),tok::Error>>,
    >(
        text: &'input str,
        __tokens: &mut __TOKENS,
        __lookahead: Option<(usize, Tok<'input>, usize)>,
        __sym0: &mut Option<(usize, FieldName, usize)>,
        __sym1: &mut Option<(usize, Tok<'input>, usize)>,
        __sym2: &mut Option<(usize, TypeName, usize)>,
    ) -> Result<(Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>), __ParseError<usize,Tok<'input>,tok::Error>>
    {
        let mut __result: (Option<(usize, Tok<'input>, usize)>, __Nonterminal<'input>);
        match __lookahead {
            Some((_, Tok::Identifier(_), _)) |
            Some((_, Tok::Implements, _)) |
            Some((_, Tok::Mutation, _)) |
            Some((_, Tok::On, _)) |
            Some((_, Tok::Query, _)) |
            Some((_, Tok::Schema, _)) |
            Some((_, Tok::Type, _)) |
            Some((_, Tok::RBrace, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action20(text, __sym0, __sym1, __sym2);
                let __nt = __Nonterminal::FieldDefinition((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }
}
pub use self::__parse__Document::parse_Document;

pub fn __action0<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Document, usize),
) -> Document
{
    (__0)
}

pub fn __action1<
    'input,
>(
    text: &'input str,
    (_, definitions, _): (usize, ::std::vec::Vec<Definition>, usize),
) -> Document
{
    Document { definitions: definitions }
}

pub fn __action2<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Definition, usize),
) -> Definition
{
    (__0)
}

pub fn __action3<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Definition, usize),
) -> Definition
{
    (__0)
}

pub fn __action4<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Definition, usize),
) -> Definition
{
    (__0)
}

pub fn __action5<
    'input,
>(
    text: &'input str,
    (_, _, _): (usize, Tok<'input>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, operations, _): (usize, ::std::vec::Vec<OperationType>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
) -> Definition
{
    {
      let query = operations.iter().filter_map(|q| {
        match q {
          &OperationType::Query(ref type_name) => Some(type_name),
          _ => None
        }
      }).collect::<Vec<_>>();

      if query.len() > 1 {
        panic!("schema must not contain more than one query definition");
      }

      Definition::Schema(Schema::new(query.get(0).map(|&v| v.clone()), None))
    }
}

pub fn __action6<
    'input,
>(
    text: &'input str,
    (_, _, _): (usize, Tok<'input>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, named_type, _): (usize, TypeName, usize),
) -> OperationType
{
    {
        OperationType::Query(named_type)
    }
}

pub fn __action7<
    'input,
>(
    text: &'input str,
    (_, _, _): (usize, Tok<'input>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, named_type, _): (usize, TypeName, usize),
) -> OperationType
{
    {
        OperationType::Mutation(named_type)
    }
}

pub fn __action8<
    'input,
>(
    text: &'input str,
    (_, name, _): (usize, &'input str, usize),
) -> TypeName
{
    { TypeName(name.to_string()) }
}

pub fn __action9<
    'input,
>(
    text: &'input str,
    (_, name, _): (usize, &'input str, usize),
) -> FieldName
{
    { FieldName(name.to_string()) }
}

pub fn __action10<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Tok<'input>, usize),
) -> &'input str
{
    { "schema" }
}

pub fn __action11<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Tok<'input>, usize),
) -> &'input str
{
    { "query" }
}

pub fn __action12<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Tok<'input>, usize),
) -> &'input str
{
    { "mutation" }
}

pub fn __action13<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Tok<'input>, usize),
) -> &'input str
{
    { "type" }
}

pub fn __action14<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Tok<'input>, usize),
) -> &'input str
{
    { "Implements" }
}

pub fn __action15<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Tok<'input>, usize),
) -> &'input str
{
    { "on" }
}

pub fn __action16<
    'input,
>(
    text: &'input str,
    (_, value, _): (usize, &'input str, usize),
) -> &'input str
{
    { value }
}

pub fn __action17<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Definition, usize),
) -> Definition
{
    (__0)
}

pub fn __action18<
    'input,
>(
    text: &'input str,
    (_, _, _): (usize, Tok<'input>, usize),
    (_, name, _): (usize, TypeName, usize),
    (_, interfaces, _): (usize, ::std::option::Option<Vec<TypeName>>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, fields, _): (usize, ::std::vec::Vec<Field>, usize),
    (_, _, _): (usize, Tok<'input>, usize),
) -> Definition
{
    {
        Definition::Object(Object::new(name, fields, interfaces.unwrap_or_else(|| Vec::new())))
    }
}

pub fn __action19<
    'input,
>(
    text: &'input str,
    (_, _, _): (usize, Tok<'input>, usize),
    (_, types, _): (usize, ::std::vec::Vec<TypeName>, usize),
) -> Vec<TypeName>
{
    { types }
}

pub fn __action20<
    'input,
>(
    text: &'input str,
    (_, name, _): (usize, FieldName, usize),
    (_, _, _): (usize, Tok<'input>, usize),
    (_, type_name, _): (usize, TypeName, usize),
) -> Field
{
    {
        Field::new(name, type_name)
    }
}

pub fn __action21<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, TypeName, usize),
) -> ::std::vec::Vec<TypeName>
{
    vec![__0]
}

pub fn __action22<
    'input,
>(
    text: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<TypeName>, usize),
    (_, e, _): (usize, TypeName, usize),
) -> ::std::vec::Vec<TypeName>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action23<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, TypeName, usize),
) -> TypeName
{
    (__0)
}

pub fn __action24<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Field, usize),
) -> ::std::vec::Vec<Field>
{
    vec![__0]
}

pub fn __action25<
    'input,
>(
    text: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Field>, usize),
    (_, e, _): (usize, Field, usize),
) -> ::std::vec::Vec<Field>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action26<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Field, usize),
) -> Field
{
    (__0)
}

pub fn __action27<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Vec<TypeName>, usize),
) -> ::std::option::Option<Vec<TypeName>>
{
    Some(__0)
}

pub fn __action28<
    'input,
>(
    text: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<Vec<TypeName>>
{
    None
}

pub fn __action29<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Vec<TypeName>, usize),
) -> Vec<TypeName>
{
    (__0)
}

pub fn __action30<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, OperationType, usize),
) -> ::std::vec::Vec<OperationType>
{
    vec![__0]
}

pub fn __action31<
    'input,
>(
    text: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<OperationType>, usize),
    (_, e, _): (usize, OperationType, usize),
) -> ::std::vec::Vec<OperationType>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action32<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, OperationType, usize),
) -> OperationType
{
    (__0)
}

pub fn __action33<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Definition, usize),
) -> ::std::vec::Vec<Definition>
{
    vec![__0]
}

pub fn __action34<
    'input,
>(
    text: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Definition>, usize),
    (_, e, _): (usize, Definition, usize),
) -> ::std::vec::Vec<Definition>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action35<
    'input,
>(
    text: &'input str,
    (_, __0, _): (usize, Definition, usize),
) -> Definition
{
    (__0)
}

pub fn __action36<
    'input,
>(
    text: &'input str,
    __0: (usize, Definition, usize),
) -> ::std::vec::Vec<Definition>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action35(
        text,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action33(
        text,
        __temp0,
    )
}

pub fn __action37<
    'input,
>(
    text: &'input str,
    __0: (usize, ::std::vec::Vec<Definition>, usize),
    __1: (usize, Definition, usize),
) -> ::std::vec::Vec<Definition>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action35(
        text,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action34(
        text,
        __0,
        __temp0,
    )
}

pub fn __action38<
    'input,
>(
    text: &'input str,
    __0: (usize, Field, usize),
) -> ::std::vec::Vec<Field>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action26(
        text,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action24(
        text,
        __temp0,
    )
}

pub fn __action39<
    'input,
>(
    text: &'input str,
    __0: (usize, ::std::vec::Vec<Field>, usize),
    __1: (usize, Field, usize),
) -> ::std::vec::Vec<Field>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action26(
        text,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action25(
        text,
        __0,
        __temp0,
    )
}

pub fn __action40<
    'input,
>(
    text: &'input str,
    __0: (usize, Vec<TypeName>, usize),
) -> ::std::option::Option<Vec<TypeName>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action29(
        text,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action27(
        text,
        __temp0,
    )
}

pub fn __action41<
    'input,
>(
    text: &'input str,
    __0: (usize, Tok<'input>, usize),
    __1: (usize, TypeName, usize),
    __2: (usize, Vec<TypeName>, usize),
    __3: (usize, Tok<'input>, usize),
    __4: (usize, ::std::vec::Vec<Field>, usize),
    __5: (usize, Tok<'input>, usize),
) -> Definition
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action40(
        text,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action18(
        text,
        __0,
        __1,
        __temp0,
        __3,
        __4,
        __5,
    )
}

pub fn __action42<
    'input,
>(
    text: &'input str,
    __0: (usize, Tok<'input>, usize),
    __1: (usize, TypeName, usize),
    __2: (usize, Tok<'input>, usize),
    __3: (usize, ::std::vec::Vec<Field>, usize),
    __4: (usize, Tok<'input>, usize),
) -> Definition
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action28(
        text,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action18(
        text,
        __0,
        __1,
        __temp0,
        __2,
        __3,
        __4,
    )
}

pub fn __action43<
    'input,
>(
    text: &'input str,
    __0: (usize, OperationType, usize),
) -> ::std::vec::Vec<OperationType>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action32(
        text,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action30(
        text,
        __temp0,
    )
}

pub fn __action44<
    'input,
>(
    text: &'input str,
    __0: (usize, ::std::vec::Vec<OperationType>, usize),
    __1: (usize, OperationType, usize),
) -> ::std::vec::Vec<OperationType>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action32(
        text,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action31(
        text,
        __0,
        __temp0,
    )
}

pub fn __action45<
    'input,
>(
    text: &'input str,
    __0: (usize, TypeName, usize),
) -> ::std::vec::Vec<TypeName>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action23(
        text,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action21(
        text,
        __temp0,
    )
}

pub fn __action46<
    'input,
>(
    text: &'input str,
    __0: (usize, ::std::vec::Vec<TypeName>, usize),
    __1: (usize, TypeName, usize),
) -> ::std::vec::Vec<TypeName>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action23(
        text,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action22(
        text,
        __0,
        __temp0,
    )
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,Tok<'input>,usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, Tok<'input>, usize) {
    type Error = tok::Error;
    fn to_triple(value: Self) -> Result<(usize,Tok<'input>,usize),tok::Error> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, Tok<'input>, usize),tok::Error> {
    type Error = tok::Error;
    fn to_triple(value: Self) -> Result<(usize,Tok<'input>,usize),tok::Error> {
        value
    }
}
