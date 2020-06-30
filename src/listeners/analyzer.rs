use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::token::Token;
use antlr_rust::tree::{ParseTreeListener, TerminalNode};

//use crate::gen::tsqlparser::*;
use crate::gen::tsqlparser::*;
use crate::gen::tsqlparserlistener::TSqlParserListener;

use crate::sql::table::Table;
use crate::sql::update::UpdateStatement;

#[derive(PartialEq)]
enum IDType {
    NONE,
    UPDATE_TABLE,
    SET_COLUMN,
    SET_VALUE,
    TABLE_NAME,
    TABLE_ALIAS,
}

#[derive(PartialEq)]
enum State {
    NONE,
    FROM,
    CONDITION,
    EXPRESSION,
}

pub struct Analyzer {
    update: UpdateStatement,
    id_type: IDType,
    state: State,
    expression_depth: i32,
    verbose: bool,
}

impl Analyzer {
    pub fn new(verbose: bool) -> Analyzer {
        Analyzer {
            update: UpdateStatement::new(),
            id_type: IDType::NONE,
            state: State::NONE,
            expression_depth: 0,
            verbose,
        }
    }

    pub fn print_output(&self) {
        for (i, col) in self.update.set_column.iter().enumerate() {
            println!("SELECT {}, {}", col, self.update.set_value[i]);
        }
    }
}

impl<'input> TSqlParserListener for Analyzer {
    fn enter_update_statement(&mut self, _ctx: &Update_statementContext) {
        self.id_type = IDType::UPDATE_TABLE;
    }

    fn enter_update_elem(&mut self, _ctx: &Update_elemContext) {
        if self.id_type != IDType::NONE {
            self.id_type = IDType::SET_COLUMN;
        }
    }

    fn exit_full_column_name(&mut self, _ctx: &Full_column_nameContext) {
        if self.id_type == IDType::SET_COLUMN {
            self.id_type = IDType::SET_VALUE;
        }
    }

    //fn exit_assignment_operator(&mut self, _ctx: &Assignment_operatorContext) {
    //    if self.id_type == IDType::NONE {
    //        return;
    //    }
    //    let value = String::from(_ctx.get_start().get_text());
    //    self.id_type = IDType::SET_VALUE;
    //    self.update.set_assignment.push(value);
    //}

    fn enter_expression(&mut self, _ctx: &ExpressionContext) {
        if self.id_type == IDType::SET_VALUE {
            if self.expression_depth == 0 {
                self.state = State::EXPRESSION;
                self.update.set_value.push(String::from(""));
            }
        }
    }

    fn exit_update_elem(&mut self, _ctx: &Update_elemContext) {
        if self.id_type == IDType::SET_VALUE {
            self.expression_depth = 0;
            self.state = State::NONE;
        }
    }

    fn enter_table_sources(&mut self, _ctx: &Table_sourcesContext) {
        if self.id_type != IDType::NONE {
            self.state = State::FROM;
        }
    }

    fn exit_table_sources(&mut self, _ctx: &Table_sourcesContext) {
        self.state = State::NONE;
    }

    fn enter_table_name_with_hint(&mut self, _ctx: &Table_name_with_hintContext) {
        if self.id_type != IDType::NONE {
            self.id_type = IDType::TABLE_NAME;
        }
    }

    fn enter_table_alias(&mut self, _ctx: &Table_aliasContext) {
        if self.id_type != IDType::NONE {
            self.id_type = IDType::TABLE_ALIAS;
        }
    }

    fn enter_search_condition_list(&mut self, _ctx: &Search_condition_listContext) {
        if self.id_type != IDType::NONE {
            self.state = State::CONDITION;
        }
    }

    fn exit_search_condition_list(&mut self, _ctx: &Search_condition_listContext) {
        self.state = State::NONE;
    }

    fn exit_id(&mut self, _ctx: &IdContext) {
        if self.id_type == IDType::NONE {
            return;
        }

        let value = String::from(_ctx.get_start().get_text());

        if self.id_type == IDType::UPDATE_TABLE {
            self.update.update_table = value;
        } else if self.id_type == IDType::SET_COLUMN {
            self.update.set_column.push(value);
        } else if self.id_type == IDType::TABLE_NAME {
            let new_table = Table::new(value);
            self.update.tables.push(new_table);
        } else if self.id_type == IDType::TABLE_ALIAS {
            self.update.tables.last_mut().unwrap().alias = value;
        }
    }

    fn exit_update_statement(&mut self, _ctx: &Update_statementContext) {
        self.id_type = IDType::NONE;
        self.print_output();
    }
}

impl<'input> ParseTreeListener for Analyzer {
    fn visit_terminal(&mut self, node: &TerminalNode) {
        let value = node.symbol.get_text();
        if self.state == State::FROM {
            self.update.from_clause.push_str(&format!(" {}", &value));
        } else if self.state == State::CONDITION {
            self.update.where_clause.push_str(&format!(" {}", &value));
        } else if self.state == State::EXPRESSION {
            self.update
                .set_value
                .last_mut()
                .unwrap()
                .push_str(&format!(" {}", &value));
        }

        if !self.verbose {
            return;
        }
        eprintln!("NODE {}", value);
    }

    fn enter_every_rule(&mut self, ctx: &dyn ParserRuleContext) {
        if !self.verbose {
            return;
        }
        eprintln!(
            "ENTERED {}: {}",
            ruleNames.get(ctx.get_rule_index()).unwrap_or(&"error"),
            ctx.get_text()
        )
    }

    fn exit_every_rule(&mut self, ctx: &dyn ParserRuleContext) {
        if !self.verbose {
            return;
        }
        eprintln!(
            "EXITED {}: {}",
            ruleNames.get(ctx.get_rule_index()).unwrap_or(&"error"),
            ctx.get_text()
        )
    }
}
