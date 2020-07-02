#![allow(non_camel_case_types)]
#![feature(inner_deref)]
#![feature(try_blocks)]

#[macro_use]
extern crate lazy_static;
extern crate crypto;

use std::env;
use std::fs;
use std::io::{self, Read};

use antlr_rust::common_token_stream::CommonTokenStream;

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
    pub mod table;
    pub mod update;
}

use listeners::analyzer::Analyzer;

fn main() {
    let mut buffer = String::new();
    let mut verbose = false;

    let args: Vec<String> = env::args().collect();

    let mut files = Vec::new();

    for (i, arg) in args.iter().enumerate() {
        if i == 0 {
            continue;
        }

        if arg == "-v" || arg == "--verbose" {
            verbose = true;
        } else {
            files.push(arg);
        }
    }

    let file_name = String::from(if files.is_empty() {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle
            .read_to_string(&mut buffer)
            .expect("Failed to read stdin");
        ""
    } else {
        buffer = fs::read_to_string(files[0]).expect("Failed to read file");
        &files[0]
    });

    /* This print is actually a hack to avoid leading comments */
    let query = String::from("print 'seeql invoked here\n\n") + &buffer;
    let query2 = query.clone();

    let mut _lexer = TSqlLexer::new(Box::new(UpperStream::new(query.into())));
    let token_source = CommonTokenStream::new(_lexer);
    let mut parser = TSqlParser::new(Box::new(token_source));
    let analyzer = Box::new(Analyzer::new(verbose, query2, file_name));
    parser.add_parse_listener(analyzer);

    let _result = parser.tsql_file().expect("parser failed");
}
