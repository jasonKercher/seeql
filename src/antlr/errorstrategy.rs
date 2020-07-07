use std::borrow::Borrow;

use antlr_rust::error_strategy::{DefaultErrorStrategy, ErrorStrategy};
use antlr_rust::errors::*;
use antlr_rust::parser::Parser;
use antlr_rust::token::{OwningToken, Token};

/* Copied from antlr_rust::util */
fn escape_whitespaces(data: impl Borrow<str>, escape_spaces: bool) -> String {
    let data = data.borrow();
    let mut res = String::with_capacity(data.len());
    data.chars().for_each(|ch| match ch {
        ' ' if escape_spaces => res.extend("\u{00B7}".chars()),
        '\t' => res.extend("\\t".chars()),
        '\n' => res.extend("\\n".chars()),
        '\r' => res.extend("\\r".chars()),
        _ => res.push(ch),
    });
    res
}

fn escape_wsand_quote(&self, s: &str) -> String {
    format!("'{}'", escape_whitespaces(s, false))
}

pub struct SeeqlErrorStrategy(DefaultErrorStrategy);

impl SeeqlErrorStrategy {
    pub fn new() -> SeeqlErrorStrategy {
        Self(DefaultErrorStrategy::new())
    }

    //fn report_input_mismatch(&self, recognizer: &dyn Parser, token: &InputMisMatchError) -> String {
    //    format!(
    //        "Syntax error near token '{}'",
    //        self.get_token_error_display(e.get_offending_token()),
    //    )
    //}

    fn get_token_error_display(&self, t: &dyn Token) -> String {
        let text = t.get_text();
        escape_wsand_quote(text)
    }
}

impl ErrorStrategy for SeeqlErrorStrategy {
    fn reset(&mut self, recognizer: &mut dyn Parser) {
        self.0.reset(recognizer)
    }

    fn recover_inline(&mut self, recognizer: &mut dyn Parser) -> Result<OwningToken, ANTLRError> {
        self.0.recover_inline(recognizer)
    }

    fn recover(&mut self, recognizer: &mut dyn Parser, e: &ANTLRError) -> Result<(), ANTLRError> {
        self.0.recover(recognizer, e)
    }

    fn sync(&mut self, _recognizer: &mut dyn Parser) -> Result<(), ANTLRError> {
        self.0.sync(_recognizer)
    }

    fn in_error_recovery_mode(&mut self, recognizer: &mut dyn Parser) -> bool {
        self.0.in_error_recovery_mode(recognizer)
    }

    fn report_error(&mut self, recognizer: &mut dyn Parser, e: &ANTLRError) {
        if self.in_error_recovery_mode(recognizer) {
            return;
        }

        let offending_token = e.get_offending_token();

        //self.begin_error_condition(recognizer);
        let msg = match e {
            ANTLRError::InputMismatchError(_e) => format!(
                "Syntax error near token {}",
                self.get_token_error_display(offending_token.unwrap())
            ),
            _ => e.to_string(),
        };
        let offending_token_index = e.get_offending_token().map(|it| it.get_token_index());
        recognizer.notify_error_listeners(msg, offending_token_index, Some(&e))
    }

    fn report_match(&mut self, recognizer: &mut dyn Parser) {
        self.0.report_match(recognizer)
    }
}

