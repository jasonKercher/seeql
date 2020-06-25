/**
 * This is just a wrapper for the standard
 * ANTLR InputStream. This will upper case
 * all tokens.
 */
use std::result;

use antlr_rust::char_stream::CharStream;
use antlr_rust::errors::ANTLRError;
use antlr_rust::input_stream::InputStream;
use antlr_rust::int_stream::IntStream;
use antlr_rust::interval_set::Interval;
use antlr_rust::token::Token;

pub struct UpperStream {
    stream: InputStream,
}

impl UpperStream {
    pub fn new(data: String) -> UpperStream {
        let stream = InputStream::new(data);
        UpperStream { stream }
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.stream.reset();
    }

    #[allow(dead_code)]
    pub fn lt(&mut self, offset: isize) -> isize {
        self.stream.lt(offset)
    }
}

impl CharStream for UpperStream {
    fn get_text(&self, _start: isize, _stop: isize) -> String {
        self.stream.get_text(_start, _stop)
    }

    fn get_text_from_tokens(&self, _start: &dyn Token, _stop: &dyn Token) -> &str {
        self.stream.get_text_from_tokens(_start, _stop)
    }

    fn get_text_from_interval(&self, i: &Interval) -> String {
        self.stream.get_text_from_interval(i)
    }
}

impl IntStream for UpperStream {
    fn consume(&mut self) -> result::Result<(), ANTLRError> {
        self.stream.consume()
    }

    #[allow(unused_mut)]
    fn la(&mut self, mut offset: isize) -> isize {
        let val = self.stream.la(offset);

        /* If val represents a capital ascii
         * character, upper case it
         */
        if val >= 97 && val <= 122 {
            return val - 32;
        }
        val
    }

    fn mark(&mut self) -> isize {
        self.stream.mark()
    }

    fn release(&mut self, _marker: isize) {
        self.stream.release(_marker);
    }

    fn index(&self) -> isize {
        self.stream.index()
    }

    #[allow(unused_mut)]
    fn seek(&mut self, mut index: isize) {
        self.stream.seek(index);
    }

    fn size(&self) -> isize {
        self.stream.size()
    }

    fn get_source_name(&self) -> String {
        self.stream.get_source_name()
    }
}
