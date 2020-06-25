use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::tree::{ParseTreeListener, TerminalNode};
use antlr_rust::token::Token;

use crate::gen::tsqlparser;
use crate::gen::tsqlparserlistener::TSqlParserListener;

pub struct Analyzer;

impl<'input> TSqlParserListener for Analyzer {}

impl<'input> ParseTreeListener for Analyzer {
    fn visit_terminal(&mut self, node: &TerminalNode) {
        println!("terminal node {}", node.symbol.get_text());
    }

    fn enter_every_rule(&mut self, ctx: &dyn ParserRuleContext) {
        println!(
            "rule entered {}",
            tsqlparser::ruleNames
                .get(ctx.get_rule_index())
                .unwrap_or(&"error")
        )
    }

    fn exit_every_rule(&mut self, ctx: &dyn ParserRuleContext) {
        println!(
            "rule exited {}",
            tsqlparser::ruleNames
                .get(ctx.get_rule_index())
                .unwrap_or(&"error")
        )
    }
}
