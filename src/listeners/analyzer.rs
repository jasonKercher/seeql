use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::token::Token;
use antlr_rust::tree::{ParseTreeListener, TerminalNode};

use crypto::digest::Digest;
use crypto::md5::Md5;

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
    update: Option<UpdateStatement>,
    original_text: String,
    id_type: IDType,
    state: State,
    update_count: u32,
    subquery_depth: i32,
    expression_depth: i32,
    verbose: bool,
}

fn gen_md5(hashme: &str) -> String {
    let mut sh = Md5::new();
    sh.input_str(hashme);
    sh.result_str()
}

impl Analyzer {
    pub fn new(verbose: bool, original_text: String) -> Analyzer {
        Analyzer {
            update: None,
            original_text,
            id_type: IDType::NONE,
            state: State::NONE,
            update_count: 0,
            subquery_depth: 0,
            expression_depth: 0,
            verbose,
        }
    }

    fn set_state(&mut self, state: State) {
        if self.subquery_depth == 0 {
            self.state = state;
        }
    }

    fn set_id_type(&mut self, id_type: IDType) {
        if self.subquery_depth == 0 {
            self.id_type = id_type;
        }
    }

    fn get_text(&self, a: isize, b: isize) -> String {
        let c = b + 1;
        self.original_text
            .chars()
            .skip(a as usize)
            .take(c as usize)
            .collect()
    }

    pub fn print_output(&self) {
        for (i, col) in self.update.as_ref().unwrap().set_column.iter().enumerate() {
            let update_table_alias = if self.update.as_ref().unwrap().update_table.alias.is_empty()
            {
                &self.update.as_ref().unwrap().update_table.name
            } else {
                &self.update.as_ref().unwrap().update_table.alias
            };

            let check_name = &format!("check_{}_{}", self.update_count, i);
            let table_name = &self.update.as_ref().unwrap().update_table.name;

            /*
             * Build Check table
             */
            println!(
                "\nSELECT {}.{} val, {} new_val",
                update_table_alias,
                col,
                self.update.as_ref().unwrap().set_value[i]
            );

            println!("INTO #{}", check_name);

            if self.update.as_ref().unwrap().from_clause.is_empty() {
                println!("FROM {}", table_name);
            } else {
                println!("FROM {}", self.update.as_ref().unwrap().from_clause);
            }

            if !self.update.as_ref().unwrap().where_clause.is_empty() {
                println!("WHERE {}", self.update.as_ref().unwrap().where_clause);
            }

            /*
             * Calculate statistics
             */
            let query = str::replace(&self.update.as_ref().unwrap().query, "'", "''");
            let clean_query = query.split_whitespace().collect::<String>();
            let hash = gen_md5(&clean_query);
            let file_name = "aFile";

            println!("\ninsert into check_tracking (query, hash, file_name, table_name, field_name, table_count, affected, changed, to_null, to_blank)\n\
                select '{}', '{}', '{}', '{}', '{}'\n\
                    \t,count(*) affected\n\
                    \t,(select count(*) from [{}]) table_count\n\
                    \t,sum(case when val != new_val then 1 else 0 end) changed\n\
                    \t,sum(case when val is not null and new_val is null then 1 else 0 end) to_null\n\
                    \t,sum(case when val != '' and new_val = '' then 1 else 0 end) to_blank\n\
                from #{}\n", query, hash, file_name, table_name, col, table_name, check_name);

            println!("update check_tracking\n\
                set  percent_affected = case when table_count > 0 then 100 * (cast(affected as float) / cast(table_count as float)) end\n\
                    \t,percent_redundance = case when affected > 0 then 100 * ((cast(affected as float) - cast(changed as float)) / cast(affected as float)) end\n\
                where hash = '{}'", hash);

            println!(
                "declare @{}_start datetime = GETDATE()\n\n\
                /**** Original Code ****/\n\
                {}\n\
                /**** Original Code ****/\n\n\
                declare @{}_rowcount int = @@rowcount\n\
                declare @{}_end datetime = GETDATE()\n\
                declare @{}_duration int = DATEDIFF(s,@{}_start,@{}_end)\n",
                check_name,
                self.update.as_ref().unwrap().query,
                check_name,
                check_name,
                check_name,
                check_name,
                check_name,
            );

            println!(
                "update check_tracking\n\
                set actual_affected = @{}_rowcount\n\
                    \t,duration = @{}_duration\n\
                where hash = '{}'\n",
                check_name, check_name, hash
            );
        }
    }
}

impl<'input> TSqlParserListener for Analyzer {
    fn enter_update_statement(&mut self, _ctx: &Update_statementContext) {
        self.update = Some(UpdateStatement::new());
        //self.update.as_mut().unwrap().start = _ctx.get_start().get_start();
        self.update_count += 1;
        self.set_id_type(IDType::UPDATE_TABLE);
    }

    fn enter_update_elem(&mut self, _ctx: &Update_elemContext) {
        if self.update.is_some() {
            self.set_id_type(IDType::SET_COLUMN);
        }
    }

    fn exit_full_column_name(&mut self, _ctx: &Full_column_nameContext) {
        if self.id_type == IDType::SET_COLUMN {
            self.set_id_type(IDType::SET_VALUE);
        }
    }

    /** This will need to be implemented for all the weird assignment operators like += **/
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
                self.set_state(State::EXPRESSION);
                self.update
                    .as_mut()
                    .unwrap()
                    .set_value
                    .push(String::from(""));
            }
            self.expression_depth += 1;
        }
    }

    fn exit_update_elem(&mut self, _ctx: &Update_elemContext) {
        if self.update.is_some() {
            self.expression_depth = 0;
            self.set_state(State::NONE);
        }
    }

    fn enter_table_sources(&mut self, _ctx: &Table_sourcesContext) {
        if self.update.is_some() {
            self.set_state(State::FROM);
        }
    }

    fn exit_table_sources(&mut self, _ctx: &Table_sourcesContext) {
        self.set_state(State::NONE);
    }

    fn enter_table_name_with_hint(&mut self, _ctx: &Table_name_with_hintContext) {
        if self.update.is_some() {
            self.set_id_type(IDType::TABLE_NAME);
        }
    }

    fn enter_table_alias(&mut self, _ctx: &Table_aliasContext) {
        if self.update.is_some() {
            self.set_id_type(IDType::TABLE_ALIAS);
        }
    }

    fn enter_search_condition_list(&mut self, _ctx: &Search_condition_listContext) {
        if self.update.is_some() {
            self.set_state(State::CONDITION);
            self.set_id_type(IDType::NONE);
        }
    }

    fn exit_search_condition_list(&mut self, _ctx: &Search_condition_listContext) {
        self.set_state(State::NONE);
    }

    fn exit_id(&mut self, _ctx: &IdContext) {
        if self.update.is_none() {
            return;
        }

        let value = String::from(_ctx.get_start().get_text());

        if self.id_type == IDType::UPDATE_TABLE {
            self.update.as_mut().unwrap().update_table.name = value;
        } else if self.id_type == IDType::SET_COLUMN {
            self.update.as_mut().unwrap().set_column.push(value);
        } else if self.id_type == IDType::TABLE_NAME {
            let new_table = Table::new(value);
            self.update.as_mut().unwrap().tables.push(new_table);
        } else if self.id_type == IDType::TABLE_ALIAS {
            self.update
                .as_mut()
                .unwrap()
                .tables
                .last_mut()
                .unwrap()
                .alias = value.clone();

            if self.update.as_ref().unwrap().update_table.name == value {
                self.update.as_mut().unwrap().update_table.alias = value;
                self.update.as_mut().unwrap().update_table.name = self
                    .update
                    .as_ref()
                    .unwrap()
                    .tables
                    .last()
                    .unwrap()
                    .name
                    .clone();
            }
        }
    }

    fn exit_update_statement(&mut self, _ctx: &Update_statementContext) {
        let start = _ctx.get_start().get_start();
        let stop = _ctx.get_stop().get_stop();
        self.update.as_mut().unwrap().query = self.get_text(start, stop);
        self.print_output();
        self.update = None;
    }

    fn enter_subquery(&mut self, _ctx: &SubqueryContext) {
        self.subquery_depth += 1;
    }

    fn exit_subquery(&mut self, _ctx: &SubqueryContext) {
        self.subquery_depth -= 1;
    }
}

impl<'input> ParseTreeListener for Analyzer {
    fn visit_terminal(&mut self, node: &TerminalNode) {
        let value = node.symbol.get_text();
        if self.state == State::FROM {
            self.update
                .as_mut()
                .unwrap()
                .from_clause
                .push_str(&format!(" {}", &value));
        } else if self.state == State::CONDITION {
            self.update
                .as_mut()
                .unwrap()
                .where_clause
                .push_str(&format!(" {}", &value));
        } else if self.state == State::EXPRESSION {
            self.update
                .as_mut()
                .unwrap()
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
