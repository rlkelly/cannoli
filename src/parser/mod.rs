pub mod ast;
mod util;

use super::lexer::{Lexer, ResultToken};
use super::lexer::tokens::Token;
use self::ast::*;

fn parse_return_stmt(opt: Option<(usize, ResultToken)>, stream: &mut Lexer)
    -> (Option<(usize, ResultToken)>, Statement) {
    unimplemented!()
}

fn parse_flow_stmt(opt: Option<(usize, ResultToken)>, stream: &mut Lexer)
    -> (Option<(usize, ResultToken)>, Statement) {
    let token = util::get_token(&opt);

    match token {
        Token::Break    => (stream.next(), Statement::Break),
        Token::Continue => (stream.next(), Statement::Continue),
        Token::Return   => parse_return_stmt(stream.next(), stream),
        Token::Raise    => unimplemented!(),
        Token::Yield    => unimplemented!(),
        _ => unimplemented!()
    }
}

fn parse_nonlocal_stmt(opt: Option<(usize, ResultToken)>, stream: &mut Lexer)
    -> (Option<(usize, ResultToken)>, Statement) {
    let token = util::get_token(&opt);

    match token {
        Token::Identifier(name) => {
            let opt = stream.next();
            let token = util::get_token(&opt);

            match token {
                Token::Comma => {
                    let (opt, stmt) =
                        parse_nonlocal_stmt(stream.next(), stream);
                    let mut names = match stmt {
                        Statement::Nonlocal { names } => names,
                        _ => panic!("invalid enum, found {:?}", stmt)
                    };

                    names.insert(0, name);
                    (opt, Statement::Nonlocal { names })
                },
                _ => (opt, Statement::Nonlocal { names: vec![name] })
            }
        }
        _ => panic!("expected 'identifier', found '{:?}'", token)
    }
}

fn parse_global_stmt(opt: Option<(usize, ResultToken)>, stream: &mut Lexer)
    -> (Option<(usize, ResultToken)>, Statement) {
    let token = util::get_token(&opt);

    match token {
        Token::Identifier(name) => {
            let opt = stream.next();
            let token = util::get_token(&opt);

            match token {
                Token::Comma => {
                    let (opt, stmt) = parse_global_stmt(stream.next(), stream);
                    let mut names = match stmt {
                        Statement::Global { names } => names,
                        _ => panic!("invalid enum, found {:?}", stmt)
                    };

                    names.insert(0, name);
                    (opt, Statement::Global { names })
                },
                _ => (opt, Statement::Global { names: vec![name] })
            }
        },
        _ => panic!("expected 'identifier', found '{:?}'", token)
    }
}

fn parse_small_stmt(opt: Option<(usize, ResultToken)>, stream: &mut Lexer)
    -> (Option<(usize, ResultToken)>, Statement) {
    let token = util::get_token(&opt);

    match token {
        Token::Pass => (stream.next(), Statement::Pass),
        Token::Global => parse_global_stmt(stream.next(), stream),
        Token::Nonlocal => parse_nonlocal_stmt(stream.next(), stream),
        ref token if util::valid_flow_stmt(&token) => {
            parse_flow_stmt(opt, stream)
        },
        _ => unimplemented!()
    }
}

fn parse_simple_stmt(opt: Option<(usize, ResultToken)>, mut stream: &mut Lexer)
    -> (Option<(usize, ResultToken)>, Vec<Statement>) {
    let (opt, small_stmt) = parse_small_stmt(opt, &mut stream);
    let token = util::get_token(&opt);

    // TODO maybe use a peek here?
    match token {
        Token::Semi => {
            let opt = stream.next();
            let token = util::get_token(&opt);

            match token {
                Token::Newline => (stream.next(), vec![small_stmt]),
                _ => {
                    let (opt, mut stmts) = parse_simple_stmt(opt, stream);

                    stmts.insert(0, small_stmt);
                    (opt, stmts)
                }
            }
        },
        Token::Newline => {
            (stream.next(), vec![small_stmt])
        },
        bad_token => {
            panic!("expected ';' or '\\n', found '{:?}'", bad_token);
        }
    }
}

fn parse_compound_stmt(opt: Option<(usize, ResultToken)>, stream: &mut Lexer)
    -> (Option<(usize, ResultToken)>, Statement) {
    unimplemented!()
}

fn parse_stmt(opt: Option<(usize, ResultToken)>, mut stream: &mut Lexer)
    -> (Option<(usize, ResultToken)>, Vec<Statement>) {
    let token = util::get_token(&opt);

    if util::valid_simple_stmt(&token) {
        parse_simple_stmt(opt, &mut stream)
    } else {
        let (opt, stmt) = parse_compound_stmt(opt, &mut stream);
        (opt, vec![stmt])
    }
}

fn parse_file_input(opt: Option<(usize, ResultToken)>,
    mut stream: &mut Lexer) -> (Option<(usize, ResultToken)>, Ast) {
    if opt.is_none() {
        return (opt, Ast::Module { body: vec![] });
    }

    let token = util::get_token(&opt);

    match token {
        Token::Newline => parse_file_input(stream.next(), &mut stream),
        _ => {
            let (opt, mut stmt_vec) = parse_stmt(opt, &mut stream);
            let (opt, Ast::Module { body }) =
                parse_file_input(opt, &mut stream);

            stmt_vec.extend(body);
            (opt, Ast::Module { body: stmt_vec })
        }
    }
}

pub fn parse_start_symbol(mut stream: Lexer) -> Ast {
    let (next_token, ast) = parse_file_input(stream.next(), &mut stream);

    match next_token {
        Some(_) => panic!("expected 'EOF' found '{:?}'", next_token.unwrap()),
        None    => ast
    }
}