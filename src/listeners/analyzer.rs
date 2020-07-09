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
    file_name: String,
    id_type: IDType,
    state: State,
    location: isize,
    update_count: u32,
    logic_depth: i32,
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
    pub fn new(
        verbose: bool,
        original_text: String,
        file_name: String,
        location: isize,
    ) -> Analyzer {
        Analyzer {
            update: None,
            original_text,
            file_name,
            id_type: IDType::NONE,
            state: State::NONE,
            location,
            update_count: 0,
            logic_depth: 0,
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

    fn get_text(&self, start: isize, stop: isize) -> String {
        let length = stop - start + 1;

        self.original_text
            .chars()
            .skip(start as usize)
            .take(length as usize)
            .collect()
    }

    pub fn print_output(&self) {
        let query = str::replace(&self.update.as_ref().unwrap().query, "'", "''");
        let lineno = self.update.as_ref().unwrap().lineno - 1;
        let clean_query = query.split_whitespace().collect::<String>();
        let hash = gen_md5(&clean_query);

        let update_table_alias = if self.update.as_ref().unwrap().update_table.alias.is_empty() {
            &self.update.as_ref().unwrap().update_table.name
        } else {
            &self.update.as_ref().unwrap().update_table.alias
        };

        let check_name = &format!("check_{}", self.update_count);
        let table_name = &self.update.as_ref().unwrap().update_table.name;
        let table_name_pure = table_name.replace(&['[', ']', '\''][..], "");

        println!("\n/**** BEGIN GENERATED OUTPUT ****/\n");

        println!(
            "\ndeclare @{}_max_id int = isnull((select max(query_id) from _check_), 0)",
            check_name
        );

        for (i, col) in self.update.as_ref().unwrap().set_column.iter().enumerate() {
            let col_pure = col.replace(&['[', ']', '\''][..], "");

            /*
             * Build Check table
             */
            println!(
                "SELECT {}.{} val, {} new_val",
                update_table_alias,
                col,
                self.update.as_ref().unwrap().set_value[i]
            );

            println!("INTO #{}_{}", check_name, i);

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
            println!("\ninsert into _check_ (query, hash, file_name, line, table_name, field_name, affected, table_count, changed, to_null, to_blank)\n\
                select '{}', '{}', '{}', {}, '{}', '{}'\n\
                    \t,count(*) affected\n\
                    \t,(select count(*) from {}) table_count\n\
                    \t,sum(case when val != new_val then 1 else 0 end) changed\n\
                    \t,sum(case when val is not null and new_val is null then 1 else 0 end) to_null\n\
                    \t,sum(case when val != '' and new_val = '' then 1 else 0 end) to_blank\n\
                from #{}_{}\n", query, hash, self.file_name, lineno, table_name_pure, col_pure, table_name, check_name, i);
        }

        println!("update _check_\n\
            set  percent_affected = case when table_count > 0 then 100 * (cast(affected as float) / cast(table_count as float)) end\n\
                \t,percent_redundance = case when affected > 0 then 100 * ((cast(affected as float) - cast(changed as float)) / cast(affected as float)) end\n\
            where hash = '{}' and query_id > @{}_max_id\n", hash, check_name);

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
            "update _check_\n\
            set actual_affected = @{}_rowcount\n\
                \t,duration = @{}_duration\n\
                \t,exec_datetime = getdate()\n\
            where hash = '{}' and query_id > @{}_max_id\n",
            check_name, check_name, hash, check_name
        );

        println!("/**** END GENERATED OUTPUT ****/\n");
    }

    fn print_non_update(&self, stop: isize) {
        if self.location < stop {
            print!("{}", self.get_text(self.location, stop));
        }
    }
}

impl<'input> TSqlParserListener for Analyzer {
    fn exit_tsql_file(&mut self, _ctx: &Tsql_fileContext) {
        /* Add new line to the end of the output */
        println!();
    }

    fn exit_sql_clause(&mut self, _ctx: &Sql_clauseContext) {
        let stop = _ctx.get_stop().get_stop();

        if self.update.is_some() {
            let start = _ctx.get_start().get_start();
            self.update.as_mut().unwrap().query = self.get_text(start, stop);
            self.print_output();
            self.update = None;
        } else {
            self.print_non_update(stop);
        }

        self.location = stop + 1;
    }

    fn enter_update_statement(&mut self, _ctx: &Update_statementContext) {
        if self.logic_depth != 0 {
            return;
        }
        let lineno = _ctx.get_start().get_line();
        self.update = Some(UpdateStatement::new(lineno));
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

    fn exit_full_table_name(&mut self, _ctx: &Full_table_nameContext) {
        if self.update.is_none() || self.subquery_depth != 0 {
            return;
        }
        let stop = _ctx.get_stop().get_stop();
        let start = _ctx.get_start().get_start();
        self.update.as_mut().unwrap().update_table.name = self.get_text(start, stop);
    }

    fn exit_table_name_with_hint(&mut self, _ctx: &Table_name_with_hintContext) {
        if self.update.is_none() || self.subquery_depth != 0 {
            return;
        }
        let stop = _ctx.get_stop().get_stop();
        let start = _ctx.get_start().get_start();
        let new_table = Table::new(self.get_text(start, stop));
        self.update.as_mut().unwrap().tables.push(new_table);
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
        if self.update.is_none() || self.subquery_depth != 0 {
            return;
        }

        let value = String::from(_ctx.get_start().get_text());

        //if self.id_type == IDType::UPDATE_TABLE {
        //    self.update.as_mut().unwrap().update_table.name = value;
        if self.id_type == IDType::SET_COLUMN {
            self.update.as_mut().unwrap().set_column.push(value);
        //} else if self.id_type == IDType::TABLE_NAME {
        //    let new_table = Table::new(value);
        //    self.update.as_mut().unwrap().tables.push(new_table);
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

    fn enter_if_statement(&mut self, _ctx: &If_statementContext) {
        self.logic_depth += 1;
    }

    fn exit_if_statement(&mut self, _ctx: &If_statementContext) {
        self.logic_depth -= 1;
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
