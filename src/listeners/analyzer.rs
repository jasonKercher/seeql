use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::token::Token;
use antlr_rust::tree::{ParseTreeListener, TerminalNode};

use crate::gen::tsqlparser;
use crate::gen::tsqlparserlistener::TSqlParserListener;

use crate::query::update::UpdateStatement;

pub struct Analyzer {
    pub verbose: bool,
    output: String,
    updates: Vec<UpdateStatement>,
}

impl Analyzer {
    pub fn new(verbose: bool) -> Analyzer {
        Analyzer {
            verbose,
            output: String::new(),
            updates: Vec::new(),
        }
    }
}

impl<'input> TSqlParserListener for Analyzer {}

impl<'input> ParseTreeListener for Analyzer {
    fn visit_terminal(&mut self, node: &TerminalNode) {
        if !self.verbose {
            return;
        }
        eprintln!("NODE {}", node.symbol.get_text());
    }

    fn enter_every_rule(&mut self, ctx: &dyn ParserRuleContext) {
        if !self.verbose {
            return;
        }
        eprintln!(
            "ENTERED {}: {}",
            tsqlparser::ruleNames
                .get(ctx.get_rule_index())
                .unwrap_or(&"error"),
            ctx.get_text()
        )
    }

    fn exit_every_rule(&mut self, ctx: &dyn ParserRuleContext) {
        if !self.verbose {
            return;
        }
        eprintln!(
            "EXITED {}: {}",
            tsqlparser::ruleNames
                .get(ctx.get_rule_index())
                .unwrap_or(&"error"),
            ctx.get_text()
        )
    }
}
