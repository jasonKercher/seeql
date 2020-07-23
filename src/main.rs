#![allow(non_camel_case_types)]
#![feature(inner_deref)]
#![feature(try_blocks)]

#[macro_use]
extern crate lazy_static;
extern crate crypto;

use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::Path;

use antlr_rust::common_token_stream::CommonTokenStream;

mod antlr {
    pub mod errorlistener;
    pub mod errorstrategy;
    pub mod upperstream;
}

use antlr::errorlistener::SeeqlErrorListener;
use antlr::errorstrategy::SeeqlErrorStrategy;
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

pub struct Props {
    pub verbose: bool,
    pub analyze: bool,
}

fn main() {
    let mut buffer = String::new();

    let args: Vec<String> = env::args().collect();

    let mut props = Props {
        verbose: false,
        analyze: false,
    };
    let mut files = Vec::new();

    for (i, arg) in args.iter().enumerate() {
        if i == 0 {
            continue;
        }

        if arg == "-v" || arg == "--verbose" {
            props.verbose = true;
        } else if arg == "-a" || arg == "--analyze" {
            props.analyze = true;
        } else if arg == "--create" {
            println!(
                "if object_id('_check_', 'U') is NULL\n\
                    \tcreate table _check_ (\n\
                        \t\t query              varchar(max)\n\
                        \t\t,hash               varchar(50)\n\
                        \t\t,file_name          varchar(100)\n\
                        \t\t,line               int\n\
                        \t\t,table_name         varchar(50)\n\
                        \t\t,field_name         varchar(50)\n\
                        \t\t,table_count        int\n\
                        \t\t,affected           int\n\
                        \t\t,actual_affected    int\n\
                        \t\t,percent_affected   float\n\
                        \t\t,changed            int\n\
                        \t\t,percent_redundance float\n\
                        \t\t,to_null            int\n\
                        \t\t,to_blank           int\n\
                        \t\t,duration           int\n\
                        \t\t,exec_datetime      datetime\n\
                        \t\t,query_id           int not null identity(1, 1)\n\
                    \t);"
            );
            std::process::exit(0);
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

    let basename = match Path::new(&file_name).file_name() {
        Some(x) => String::from(x.to_str().unwrap()),
        None => file_name,
    };

    /* This print is actually a hack to avoid leading comments */
    let mut query = String::from("_no_op_label_:\n");
    let hack_length = query.len();

    query += &buffer;
    let query2 = query.clone();

    let mut lexer = TSqlLexer::new(Box::new(UpperStream::new(query.into())));
    lexer.remove_error_listeners();

    //let error_listener = Box::new(SeeqlErrorListener {});
    lexer.add_error_listener(Box::new(SeeqlErrorListener {}));
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = TSqlParser::new(Box::new(token_source));
    parser.set_error_strategy(Box::new(SeeqlErrorStrategy::new()));
    let analyzer = Box::new(Analyzer::new(props, query2, basename, hack_length as isize));
    parser.add_parse_listener(analyzer);

    let _result = parser.tsql_file().expect("parser failed");
}
