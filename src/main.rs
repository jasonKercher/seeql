#![allow(non_camel_case_types)]
#![feature(inner_deref)]
#![feature(try_blocks)]

#[macro_use]
extern crate lazy_static;

use std::io::{self, Read};

//use antlr_rust::common_token_factory::CommonTokenFactory;
use antlr_rust::common_token_stream::CommonTokenStream;
//use antlr_rust::int_stream::IntStream;
//use antlr_rust::lexer::Lexer;
//use antlr_rust::token::Token;
//use antlr_rust::token_stream::{TokenStream, UnbufferedTokenStream};

mod antlr {
    pub mod upperstream;
}

use antlr::upperstream::UpperStream;

mod gen {
    pub mod tsqllexer;
    pub mod tsqlparser;
    pub mod tsqlparserlistener;
}

use gen::tsqllexer::TSqlLexer;
use gen::tsqlparser::TSqlParser;

mod listeners {
    pub mod analyzer;
}

mod sql {
    pub mod join;
    pub mod table;
    pub mod update;
}

use listeners::analyzer::Analyzer;

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle
        .read_to_string(&mut buffer)
        .expect("Failed to read stdin");

    let mut _lexer = TSqlLexer::new(Box::new(UpperStream::new(buffer.into())));
    let token_source = CommonTokenStream::new(_lexer);
    let mut parser = TSqlParser::new(Box::new(token_source));

    let analyzer = Box::new(Analyzer::new(true));

    //let analyzer = Box::new(Analyzer {
    //    verbose: true,
    //    output: String::new(),
    //    updates: Vec::new(),
    //});
    parser.add_parse_listener(analyzer);

    let _result = parser.tsql_file().expect("parser failed");
}
