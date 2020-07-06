#![allow(non_snake_case)]
// Generated from grammar/TSqlParser.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::tsqlparser::*;

//use std::any::Any;

pub trait TSqlParserListener : ParseTreeListener{

/**
 * Enter a parse tree produced by {@link TSqlParser#tsql_file}.
 * @param ctx the parse tree
 */
fn enter_tsql_file(&mut self, _ctx: &Tsql_fileContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#tsql_file}.
 * @param ctx the parse tree
 */
fn exit_tsql_file(&mut self, _ctx: &Tsql_fileContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#batch}.
 * @param ctx the parse tree
 */
fn enter_batch(&mut self, _ctx: &BatchContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#batch}.
 * @param ctx the parse tree
 */
fn exit_batch(&mut self, _ctx: &BatchContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#sql_clauses}.
 * @param ctx the parse tree
 */
fn enter_sql_clauses(&mut self, _ctx: &Sql_clausesContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#sql_clauses}.
 * @param ctx the parse tree
 */
fn exit_sql_clauses(&mut self, _ctx: &Sql_clausesContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#sql_clause}.
 * @param ctx the parse tree
 */
fn enter_sql_clause(&mut self, _ctx: &Sql_clauseContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#sql_clause}.
 * @param ctx the parse tree
 */
fn exit_sql_clause(&mut self, _ctx: &Sql_clauseContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#dml_clause}.
 * @param ctx the parse tree
 */
fn enter_dml_clause(&mut self, _ctx: &Dml_clauseContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#dml_clause}.
 * @param ctx the parse tree
 */
fn exit_dml_clause(&mut self, _ctx: &Dml_clauseContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#ddl_clause}.
 * @param ctx the parse tree
 */
fn enter_ddl_clause(&mut self, _ctx: &Ddl_clauseContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#ddl_clause}.
 * @param ctx the parse tree
 */
fn exit_ddl_clause(&mut self, _ctx: &Ddl_clauseContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#backup_statement}.
 * @param ctx the parse tree
 */
fn enter_backup_statement(&mut self, _ctx: &Backup_statementContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#backup_statement}.
 * @param ctx the parse tree
 */
fn exit_backup_statement(&mut self, _ctx: &Backup_statementContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#cfl_statement}.
 * @param ctx the parse tree
 */
fn enter_cfl_statement(&mut self, _ctx: &Cfl_statementContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#cfl_statement}.
 * @param ctx the parse tree
 */
fn exit_cfl_statement(&mut self, _ctx: &Cfl_statementContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#block_statement}.
 * @param ctx the parse tree
 */
fn enter_block_statement(&mut self, _ctx: &Block_statementContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#block_statement}.
 * @param ctx the parse tree
 */
fn exit_block_statement(&mut self, _ctx: &Block_statementContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#break_statement}.
 * @param ctx the parse tree
 */
fn enter_break_statement(&mut self, _ctx: &Break_statementContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#break_statement}.
 * @param ctx the parse tree
 */
fn exit_break_statement(&mut self, _ctx: &Break_statementContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#continue_statement}.
 * @param ctx the parse tree
 */
fn enter_continue_statement(&mut self, _ctx: &Continue_statementContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#continue_statement}.
 * @param ctx the parse tree
 */
fn exit_continue_statement(&mut self, _ctx: &Continue_statementContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#goto_statement}.
 * @param ctx the parse tree
 */
fn enter_goto_statement(&mut self, _ctx: &Goto_statementContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#goto_statement}.
 * @param ctx the parse tree
 */
fn exit_goto_statement(&mut self, _ctx: &Goto_statementContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#return_statement}.
 * @param ctx the parse tree
 */
fn enter_return_statement(&mut self, _ctx: &Return_statementContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#return_statement}.
 * @param ctx the parse tree
 */
fn exit_return_statement(&mut self, _ctx: &Return_statementContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#if_statement}.
 * @param ctx the parse tree
 */
fn enter_if_statement(&mut self, _ctx: &If_statementContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#if_statement}.
 * @param ctx the parse tree
 */
fn exit_if_statement(&mut self, _ctx: &If_statementContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#throw_statement}.
 * @param ctx the parse tree
 */
fn enter_throw_statement(&mut self, _ctx: &Throw_statementContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#throw_statement}.
 * @param ctx the parse tree
 */
fn exit_throw_statement(&mut self, _ctx: &Throw_statementContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#throw_error_number}.
 * @param ctx the parse tree
 */
fn enter_throw_error_number(&mut self, _ctx: &Throw_error_numberContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#throw_error_number}.
 * @param ctx the parse tree
 */
fn exit_throw_error_number(&mut self, _ctx: &Throw_error_numberContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#throw_message}.
 * @param ctx the parse tree
 */
fn enter_throw_message(&mut self, _ctx: &Throw_messageContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#throw_message}.
 * @param ctx the parse tree
 */
fn exit_throw_message(&mut self, _ctx: &Throw_messageContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#throw_state}.
 * @param ctx the parse tree
 */
fn enter_throw_state(&mut self, _ctx: &Throw_stateContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#throw_state}.
 * @param ctx the parse tree
 */
fn exit_throw_state(&mut self, _ctx: &Throw_stateContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#try_catch_statement}.
 * @param ctx the parse tree
 */
fn enter_try_catch_statement(&mut self, _ctx: &Try_catch_statementContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#try_catch_statement}.
 * @param ctx the parse tree
 */
fn exit_try_catch_statement(&mut self, _ctx: &Try_catch_statementContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#waitfor_statement}.
 * @param ctx the parse tree
 */
fn enter_waitfor_statement(&mut self, _ctx: &Waitfor_statementContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#waitfor_statement}.
 * @param ctx the parse tree
 */
fn exit_waitfor_statement(&mut self, _ctx: &Waitfor_statementContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#while_statement}.
 * @param ctx the parse tree
 */
fn enter_while_statement(&mut self, _ctx: &While_statementContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#while_statement}.
 * @param ctx the parse tree
 */
fn exit_while_statement(&mut self, _ctx: &While_statementContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#print_statement}.
 * @param ctx the parse tree
 */
fn enter_print_statement(&mut self, _ctx: &Print_statementContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#print_statement}.
 * @param ctx the parse tree
 */
fn exit_print_statement(&mut self, _ctx: &Print_statementContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#raiseerror_statement}.
 * @param ctx the parse tree
 */
fn enter_raiseerror_statement(&mut self, _ctx: &Raiseerror_statementContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#raiseerror_statement}.
 * @param ctx the parse tree
 */
fn exit_raiseerror_statement(&mut self, _ctx: &Raiseerror_statementContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#empty_statement}.
 * @param ctx the parse tree
 */
fn enter_empty_statement(&mut self, _ctx: &Empty_statementContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#empty_statement}.
 * @param ctx the parse tree
 */
fn exit_empty_statement(&mut self, _ctx: &Empty_statementContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#another_statement}.
 * @param ctx the parse tree
 */
fn enter_another_statement(&mut self, _ctx: &Another_statementContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#another_statement}.
 * @param ctx the parse tree
 */
fn exit_another_statement(&mut self, _ctx: &Another_statementContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_application_role}.
 * @param ctx the parse tree
 */
fn enter_alter_application_role(&mut self, _ctx: &Alter_application_roleContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_application_role}.
 * @param ctx the parse tree
 */
fn exit_alter_application_role(&mut self, _ctx: &Alter_application_roleContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_application_role}.
 * @param ctx the parse tree
 */
fn enter_create_application_role(&mut self, _ctx: &Create_application_roleContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_application_role}.
 * @param ctx the parse tree
 */
fn exit_create_application_role(&mut self, _ctx: &Create_application_roleContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_aggregate}.
 * @param ctx the parse tree
 */
fn enter_drop_aggregate(&mut self, _ctx: &Drop_aggregateContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_aggregate}.
 * @param ctx the parse tree
 */
fn exit_drop_aggregate(&mut self, _ctx: &Drop_aggregateContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_application_role}.
 * @param ctx the parse tree
 */
fn enter_drop_application_role(&mut self, _ctx: &Drop_application_roleContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_application_role}.
 * @param ctx the parse tree
 */
fn exit_drop_application_role(&mut self, _ctx: &Drop_application_roleContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_assembly}.
 * @param ctx the parse tree
 */
fn enter_alter_assembly(&mut self, _ctx: &Alter_assemblyContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_assembly}.
 * @param ctx the parse tree
 */
fn exit_alter_assembly(&mut self, _ctx: &Alter_assemblyContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_assembly_start}.
 * @param ctx the parse tree
 */
fn enter_alter_assembly_start(&mut self, _ctx: &Alter_assembly_startContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_assembly_start}.
 * @param ctx the parse tree
 */
fn exit_alter_assembly_start(&mut self, _ctx: &Alter_assembly_startContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_assembly_clause}.
 * @param ctx the parse tree
 */
fn enter_alter_assembly_clause(&mut self, _ctx: &Alter_assembly_clauseContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_assembly_clause}.
 * @param ctx the parse tree
 */
fn exit_alter_assembly_clause(&mut self, _ctx: &Alter_assembly_clauseContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_assembly_from_clause}.
 * @param ctx the parse tree
 */
fn enter_alter_assembly_from_clause(&mut self, _ctx: &Alter_assembly_from_clauseContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_assembly_from_clause}.
 * @param ctx the parse tree
 */
fn exit_alter_assembly_from_clause(&mut self, _ctx: &Alter_assembly_from_clauseContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_assembly_from_clause_start}.
 * @param ctx the parse tree
 */
fn enter_alter_assembly_from_clause_start(&mut self, _ctx: &Alter_assembly_from_clause_startContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_assembly_from_clause_start}.
 * @param ctx the parse tree
 */
fn exit_alter_assembly_from_clause_start(&mut self, _ctx: &Alter_assembly_from_clause_startContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_assembly_drop_clause}.
 * @param ctx the parse tree
 */
fn enter_alter_assembly_drop_clause(&mut self, _ctx: &Alter_assembly_drop_clauseContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_assembly_drop_clause}.
 * @param ctx the parse tree
 */
fn exit_alter_assembly_drop_clause(&mut self, _ctx: &Alter_assembly_drop_clauseContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_assembly_drop_multiple_files}.
 * @param ctx the parse tree
 */
fn enter_alter_assembly_drop_multiple_files(&mut self, _ctx: &Alter_assembly_drop_multiple_filesContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_assembly_drop_multiple_files}.
 * @param ctx the parse tree
 */
fn exit_alter_assembly_drop_multiple_files(&mut self, _ctx: &Alter_assembly_drop_multiple_filesContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_assembly_drop}.
 * @param ctx the parse tree
 */
fn enter_alter_assembly_drop(&mut self, _ctx: &Alter_assembly_dropContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_assembly_drop}.
 * @param ctx the parse tree
 */
fn exit_alter_assembly_drop(&mut self, _ctx: &Alter_assembly_dropContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_assembly_add_clause}.
 * @param ctx the parse tree
 */
fn enter_alter_assembly_add_clause(&mut self, _ctx: &Alter_assembly_add_clauseContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_assembly_add_clause}.
 * @param ctx the parse tree
 */
fn exit_alter_assembly_add_clause(&mut self, _ctx: &Alter_assembly_add_clauseContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_asssembly_add_clause_start}.
 * @param ctx the parse tree
 */
fn enter_alter_asssembly_add_clause_start(&mut self, _ctx: &Alter_asssembly_add_clause_startContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_asssembly_add_clause_start}.
 * @param ctx the parse tree
 */
fn exit_alter_asssembly_add_clause_start(&mut self, _ctx: &Alter_asssembly_add_clause_startContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_assembly_client_file_clause}.
 * @param ctx the parse tree
 */
fn enter_alter_assembly_client_file_clause(&mut self, _ctx: &Alter_assembly_client_file_clauseContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_assembly_client_file_clause}.
 * @param ctx the parse tree
 */
fn exit_alter_assembly_client_file_clause(&mut self, _ctx: &Alter_assembly_client_file_clauseContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_assembly_file_name}.
 * @param ctx the parse tree
 */
fn enter_alter_assembly_file_name(&mut self, _ctx: &Alter_assembly_file_nameContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_assembly_file_name}.
 * @param ctx the parse tree
 */
fn exit_alter_assembly_file_name(&mut self, _ctx: &Alter_assembly_file_nameContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_assembly_file_bits}.
 * @param ctx the parse tree
 */
fn enter_alter_assembly_file_bits(&mut self, _ctx: &Alter_assembly_file_bitsContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_assembly_file_bits}.
 * @param ctx the parse tree
 */
fn exit_alter_assembly_file_bits(&mut self, _ctx: &Alter_assembly_file_bitsContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_assembly_as}.
 * @param ctx the parse tree
 */
fn enter_alter_assembly_as(&mut self, _ctx: &Alter_assembly_asContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_assembly_as}.
 * @param ctx the parse tree
 */
fn exit_alter_assembly_as(&mut self, _ctx: &Alter_assembly_asContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_assembly_with_clause}.
 * @param ctx the parse tree
 */
fn enter_alter_assembly_with_clause(&mut self, _ctx: &Alter_assembly_with_clauseContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_assembly_with_clause}.
 * @param ctx the parse tree
 */
fn exit_alter_assembly_with_clause(&mut self, _ctx: &Alter_assembly_with_clauseContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_assembly_with}.
 * @param ctx the parse tree
 */
fn enter_alter_assembly_with(&mut self, _ctx: &Alter_assembly_withContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_assembly_with}.
 * @param ctx the parse tree
 */
fn exit_alter_assembly_with(&mut self, _ctx: &Alter_assembly_withContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#client_assembly_specifier}.
 * @param ctx the parse tree
 */
fn enter_client_assembly_specifier(&mut self, _ctx: &Client_assembly_specifierContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#client_assembly_specifier}.
 * @param ctx the parse tree
 */
fn exit_client_assembly_specifier(&mut self, _ctx: &Client_assembly_specifierContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#assembly_option}.
 * @param ctx the parse tree
 */
fn enter_assembly_option(&mut self, _ctx: &Assembly_optionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#assembly_option}.
 * @param ctx the parse tree
 */
fn exit_assembly_option(&mut self, _ctx: &Assembly_optionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#network_file_share}.
 * @param ctx the parse tree
 */
fn enter_network_file_share(&mut self, _ctx: &Network_file_shareContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#network_file_share}.
 * @param ctx the parse tree
 */
fn exit_network_file_share(&mut self, _ctx: &Network_file_shareContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#network_computer}.
 * @param ctx the parse tree
 */
fn enter_network_computer(&mut self, _ctx: &Network_computerContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#network_computer}.
 * @param ctx the parse tree
 */
fn exit_network_computer(&mut self, _ctx: &Network_computerContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#network_file_start}.
 * @param ctx the parse tree
 */
fn enter_network_file_start(&mut self, _ctx: &Network_file_startContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#network_file_start}.
 * @param ctx the parse tree
 */
fn exit_network_file_start(&mut self, _ctx: &Network_file_startContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#file_path}.
 * @param ctx the parse tree
 */
fn enter_file_path(&mut self, _ctx: &File_pathContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#file_path}.
 * @param ctx the parse tree
 */
fn exit_file_path(&mut self, _ctx: &File_pathContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#file_directory_path_separator}.
 * @param ctx the parse tree
 */
fn enter_file_directory_path_separator(&mut self, _ctx: &File_directory_path_separatorContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#file_directory_path_separator}.
 * @param ctx the parse tree
 */
fn exit_file_directory_path_separator(&mut self, _ctx: &File_directory_path_separatorContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#local_file}.
 * @param ctx the parse tree
 */
fn enter_local_file(&mut self, _ctx: &Local_fileContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#local_file}.
 * @param ctx the parse tree
 */
fn exit_local_file(&mut self, _ctx: &Local_fileContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#local_drive}.
 * @param ctx the parse tree
 */
fn enter_local_drive(&mut self, _ctx: &Local_driveContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#local_drive}.
 * @param ctx the parse tree
 */
fn exit_local_drive(&mut self, _ctx: &Local_driveContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#multiple_local_files}.
 * @param ctx the parse tree
 */
fn enter_multiple_local_files(&mut self, _ctx: &Multiple_local_filesContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#multiple_local_files}.
 * @param ctx the parse tree
 */
fn exit_multiple_local_files(&mut self, _ctx: &Multiple_local_filesContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#multiple_local_file_start}.
 * @param ctx the parse tree
 */
fn enter_multiple_local_file_start(&mut self, _ctx: &Multiple_local_file_startContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#multiple_local_file_start}.
 * @param ctx the parse tree
 */
fn exit_multiple_local_file_start(&mut self, _ctx: &Multiple_local_file_startContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_assembly}.
 * @param ctx the parse tree
 */
fn enter_create_assembly(&mut self, _ctx: &Create_assemblyContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_assembly}.
 * @param ctx the parse tree
 */
fn exit_create_assembly(&mut self, _ctx: &Create_assemblyContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_assembly}.
 * @param ctx the parse tree
 */
fn enter_drop_assembly(&mut self, _ctx: &Drop_assemblyContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_assembly}.
 * @param ctx the parse tree
 */
fn exit_drop_assembly(&mut self, _ctx: &Drop_assemblyContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_asymmetric_key}.
 * @param ctx the parse tree
 */
fn enter_alter_asymmetric_key(&mut self, _ctx: &Alter_asymmetric_keyContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_asymmetric_key}.
 * @param ctx the parse tree
 */
fn exit_alter_asymmetric_key(&mut self, _ctx: &Alter_asymmetric_keyContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_asymmetric_key_start}.
 * @param ctx the parse tree
 */
fn enter_alter_asymmetric_key_start(&mut self, _ctx: &Alter_asymmetric_key_startContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_asymmetric_key_start}.
 * @param ctx the parse tree
 */
fn exit_alter_asymmetric_key_start(&mut self, _ctx: &Alter_asymmetric_key_startContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#asymmetric_key_option}.
 * @param ctx the parse tree
 */
fn enter_asymmetric_key_option(&mut self, _ctx: &Asymmetric_key_optionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#asymmetric_key_option}.
 * @param ctx the parse tree
 */
fn exit_asymmetric_key_option(&mut self, _ctx: &Asymmetric_key_optionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#asymmetric_key_option_start}.
 * @param ctx the parse tree
 */
fn enter_asymmetric_key_option_start(&mut self, _ctx: &Asymmetric_key_option_startContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#asymmetric_key_option_start}.
 * @param ctx the parse tree
 */
fn exit_asymmetric_key_option_start(&mut self, _ctx: &Asymmetric_key_option_startContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#asymmetric_key_password_change_option}.
 * @param ctx the parse tree
 */
fn enter_asymmetric_key_password_change_option(&mut self, _ctx: &Asymmetric_key_password_change_optionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#asymmetric_key_password_change_option}.
 * @param ctx the parse tree
 */
fn exit_asymmetric_key_password_change_option(&mut self, _ctx: &Asymmetric_key_password_change_optionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_asymmetric_key}.
 * @param ctx the parse tree
 */
fn enter_create_asymmetric_key(&mut self, _ctx: &Create_asymmetric_keyContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_asymmetric_key}.
 * @param ctx the parse tree
 */
fn exit_create_asymmetric_key(&mut self, _ctx: &Create_asymmetric_keyContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_asymmetric_key}.
 * @param ctx the parse tree
 */
fn enter_drop_asymmetric_key(&mut self, _ctx: &Drop_asymmetric_keyContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_asymmetric_key}.
 * @param ctx the parse tree
 */
fn exit_drop_asymmetric_key(&mut self, _ctx: &Drop_asymmetric_keyContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_authorization}.
 * @param ctx the parse tree
 */
fn enter_alter_authorization(&mut self, _ctx: &Alter_authorizationContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_authorization}.
 * @param ctx the parse tree
 */
fn exit_alter_authorization(&mut self, _ctx: &Alter_authorizationContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#authorization_grantee}.
 * @param ctx the parse tree
 */
fn enter_authorization_grantee(&mut self, _ctx: &Authorization_granteeContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#authorization_grantee}.
 * @param ctx the parse tree
 */
fn exit_authorization_grantee(&mut self, _ctx: &Authorization_granteeContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#entity_to}.
 * @param ctx the parse tree
 */
fn enter_entity_to(&mut self, _ctx: &Entity_toContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#entity_to}.
 * @param ctx the parse tree
 */
fn exit_entity_to(&mut self, _ctx: &Entity_toContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#colon_colon}.
 * @param ctx the parse tree
 */
fn enter_colon_colon(&mut self, _ctx: &Colon_colonContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#colon_colon}.
 * @param ctx the parse tree
 */
fn exit_colon_colon(&mut self, _ctx: &Colon_colonContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_authorization_start}.
 * @param ctx the parse tree
 */
fn enter_alter_authorization_start(&mut self, _ctx: &Alter_authorization_startContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_authorization_start}.
 * @param ctx the parse tree
 */
fn exit_alter_authorization_start(&mut self, _ctx: &Alter_authorization_startContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_authorization_for_sql_database}.
 * @param ctx the parse tree
 */
fn enter_alter_authorization_for_sql_database(&mut self, _ctx: &Alter_authorization_for_sql_databaseContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_authorization_for_sql_database}.
 * @param ctx the parse tree
 */
fn exit_alter_authorization_for_sql_database(&mut self, _ctx: &Alter_authorization_for_sql_databaseContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_authorization_for_azure_dw}.
 * @param ctx the parse tree
 */
fn enter_alter_authorization_for_azure_dw(&mut self, _ctx: &Alter_authorization_for_azure_dwContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_authorization_for_azure_dw}.
 * @param ctx the parse tree
 */
fn exit_alter_authorization_for_azure_dw(&mut self, _ctx: &Alter_authorization_for_azure_dwContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_authorization_for_parallel_dw}.
 * @param ctx the parse tree
 */
fn enter_alter_authorization_for_parallel_dw(&mut self, _ctx: &Alter_authorization_for_parallel_dwContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_authorization_for_parallel_dw}.
 * @param ctx the parse tree
 */
fn exit_alter_authorization_for_parallel_dw(&mut self, _ctx: &Alter_authorization_for_parallel_dwContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#class_type}.
 * @param ctx the parse tree
 */
fn enter_class_type(&mut self, _ctx: &Class_typeContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#class_type}.
 * @param ctx the parse tree
 */
fn exit_class_type(&mut self, _ctx: &Class_typeContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#class_type_for_sql_database}.
 * @param ctx the parse tree
 */
fn enter_class_type_for_sql_database(&mut self, _ctx: &Class_type_for_sql_databaseContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#class_type_for_sql_database}.
 * @param ctx the parse tree
 */
fn exit_class_type_for_sql_database(&mut self, _ctx: &Class_type_for_sql_databaseContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#class_type_for_azure_dw}.
 * @param ctx the parse tree
 */
fn enter_class_type_for_azure_dw(&mut self, _ctx: &Class_type_for_azure_dwContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#class_type_for_azure_dw}.
 * @param ctx the parse tree
 */
fn exit_class_type_for_azure_dw(&mut self, _ctx: &Class_type_for_azure_dwContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#class_type_for_parallel_dw}.
 * @param ctx the parse tree
 */
fn enter_class_type_for_parallel_dw(&mut self, _ctx: &Class_type_for_parallel_dwContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#class_type_for_parallel_dw}.
 * @param ctx the parse tree
 */
fn exit_class_type_for_parallel_dw(&mut self, _ctx: &Class_type_for_parallel_dwContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_availability_group}.
 * @param ctx the parse tree
 */
fn enter_drop_availability_group(&mut self, _ctx: &Drop_availability_groupContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_availability_group}.
 * @param ctx the parse tree
 */
fn exit_drop_availability_group(&mut self, _ctx: &Drop_availability_groupContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_availability_group}.
 * @param ctx the parse tree
 */
fn enter_alter_availability_group(&mut self, _ctx: &Alter_availability_groupContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_availability_group}.
 * @param ctx the parse tree
 */
fn exit_alter_availability_group(&mut self, _ctx: &Alter_availability_groupContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_availability_group_start}.
 * @param ctx the parse tree
 */
fn enter_alter_availability_group_start(&mut self, _ctx: &Alter_availability_group_startContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_availability_group_start}.
 * @param ctx the parse tree
 */
fn exit_alter_availability_group_start(&mut self, _ctx: &Alter_availability_group_startContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_availability_group_options}.
 * @param ctx the parse tree
 */
fn enter_alter_availability_group_options(&mut self, _ctx: &Alter_availability_group_optionsContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_availability_group_options}.
 * @param ctx the parse tree
 */
fn exit_alter_availability_group_options(&mut self, _ctx: &Alter_availability_group_optionsContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_or_alter_broker_priority}.
 * @param ctx the parse tree
 */
fn enter_create_or_alter_broker_priority(&mut self, _ctx: &Create_or_alter_broker_priorityContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_or_alter_broker_priority}.
 * @param ctx the parse tree
 */
fn exit_create_or_alter_broker_priority(&mut self, _ctx: &Create_or_alter_broker_priorityContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_broker_priority}.
 * @param ctx the parse tree
 */
fn enter_drop_broker_priority(&mut self, _ctx: &Drop_broker_priorityContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_broker_priority}.
 * @param ctx the parse tree
 */
fn exit_drop_broker_priority(&mut self, _ctx: &Drop_broker_priorityContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_certificate}.
 * @param ctx the parse tree
 */
fn enter_alter_certificate(&mut self, _ctx: &Alter_certificateContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_certificate}.
 * @param ctx the parse tree
 */
fn exit_alter_certificate(&mut self, _ctx: &Alter_certificateContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_column_encryption_key}.
 * @param ctx the parse tree
 */
fn enter_alter_column_encryption_key(&mut self, _ctx: &Alter_column_encryption_keyContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_column_encryption_key}.
 * @param ctx the parse tree
 */
fn exit_alter_column_encryption_key(&mut self, _ctx: &Alter_column_encryption_keyContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_column_encryption_key}.
 * @param ctx the parse tree
 */
fn enter_create_column_encryption_key(&mut self, _ctx: &Create_column_encryption_keyContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_column_encryption_key}.
 * @param ctx the parse tree
 */
fn exit_create_column_encryption_key(&mut self, _ctx: &Create_column_encryption_keyContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_certificate}.
 * @param ctx the parse tree
 */
fn enter_drop_certificate(&mut self, _ctx: &Drop_certificateContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_certificate}.
 * @param ctx the parse tree
 */
fn exit_drop_certificate(&mut self, _ctx: &Drop_certificateContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_column_encryption_key}.
 * @param ctx the parse tree
 */
fn enter_drop_column_encryption_key(&mut self, _ctx: &Drop_column_encryption_keyContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_column_encryption_key}.
 * @param ctx the parse tree
 */
fn exit_drop_column_encryption_key(&mut self, _ctx: &Drop_column_encryption_keyContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_column_master_key}.
 * @param ctx the parse tree
 */
fn enter_drop_column_master_key(&mut self, _ctx: &Drop_column_master_keyContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_column_master_key}.
 * @param ctx the parse tree
 */
fn exit_drop_column_master_key(&mut self, _ctx: &Drop_column_master_keyContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_contract}.
 * @param ctx the parse tree
 */
fn enter_drop_contract(&mut self, _ctx: &Drop_contractContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_contract}.
 * @param ctx the parse tree
 */
fn exit_drop_contract(&mut self, _ctx: &Drop_contractContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_credential}.
 * @param ctx the parse tree
 */
fn enter_drop_credential(&mut self, _ctx: &Drop_credentialContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_credential}.
 * @param ctx the parse tree
 */
fn exit_drop_credential(&mut self, _ctx: &Drop_credentialContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_cryptograhic_provider}.
 * @param ctx the parse tree
 */
fn enter_drop_cryptograhic_provider(&mut self, _ctx: &Drop_cryptograhic_providerContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_cryptograhic_provider}.
 * @param ctx the parse tree
 */
fn exit_drop_cryptograhic_provider(&mut self, _ctx: &Drop_cryptograhic_providerContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_database}.
 * @param ctx the parse tree
 */
fn enter_drop_database(&mut self, _ctx: &Drop_databaseContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_database}.
 * @param ctx the parse tree
 */
fn exit_drop_database(&mut self, _ctx: &Drop_databaseContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_database_audit_specification}.
 * @param ctx the parse tree
 */
fn enter_drop_database_audit_specification(&mut self, _ctx: &Drop_database_audit_specificationContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_database_audit_specification}.
 * @param ctx the parse tree
 */
fn exit_drop_database_audit_specification(&mut self, _ctx: &Drop_database_audit_specificationContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_database_scoped_credential}.
 * @param ctx the parse tree
 */
fn enter_drop_database_scoped_credential(&mut self, _ctx: &Drop_database_scoped_credentialContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_database_scoped_credential}.
 * @param ctx the parse tree
 */
fn exit_drop_database_scoped_credential(&mut self, _ctx: &Drop_database_scoped_credentialContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_default}.
 * @param ctx the parse tree
 */
fn enter_drop_default(&mut self, _ctx: &Drop_defaultContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_default}.
 * @param ctx the parse tree
 */
fn exit_drop_default(&mut self, _ctx: &Drop_defaultContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_endpoint}.
 * @param ctx the parse tree
 */
fn enter_drop_endpoint(&mut self, _ctx: &Drop_endpointContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_endpoint}.
 * @param ctx the parse tree
 */
fn exit_drop_endpoint(&mut self, _ctx: &Drop_endpointContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_external_data_source}.
 * @param ctx the parse tree
 */
fn enter_drop_external_data_source(&mut self, _ctx: &Drop_external_data_sourceContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_external_data_source}.
 * @param ctx the parse tree
 */
fn exit_drop_external_data_source(&mut self, _ctx: &Drop_external_data_sourceContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_external_file_format}.
 * @param ctx the parse tree
 */
fn enter_drop_external_file_format(&mut self, _ctx: &Drop_external_file_formatContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_external_file_format}.
 * @param ctx the parse tree
 */
fn exit_drop_external_file_format(&mut self, _ctx: &Drop_external_file_formatContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_external_library}.
 * @param ctx the parse tree
 */
fn enter_drop_external_library(&mut self, _ctx: &Drop_external_libraryContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_external_library}.
 * @param ctx the parse tree
 */
fn exit_drop_external_library(&mut self, _ctx: &Drop_external_libraryContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_external_resource_pool}.
 * @param ctx the parse tree
 */
fn enter_drop_external_resource_pool(&mut self, _ctx: &Drop_external_resource_poolContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_external_resource_pool}.
 * @param ctx the parse tree
 */
fn exit_drop_external_resource_pool(&mut self, _ctx: &Drop_external_resource_poolContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_external_table}.
 * @param ctx the parse tree
 */
fn enter_drop_external_table(&mut self, _ctx: &Drop_external_tableContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_external_table}.
 * @param ctx the parse tree
 */
fn exit_drop_external_table(&mut self, _ctx: &Drop_external_tableContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_event_notifications}.
 * @param ctx the parse tree
 */
fn enter_drop_event_notifications(&mut self, _ctx: &Drop_event_notificationsContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_event_notifications}.
 * @param ctx the parse tree
 */
fn exit_drop_event_notifications(&mut self, _ctx: &Drop_event_notificationsContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_event_session}.
 * @param ctx the parse tree
 */
fn enter_drop_event_session(&mut self, _ctx: &Drop_event_sessionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_event_session}.
 * @param ctx the parse tree
 */
fn exit_drop_event_session(&mut self, _ctx: &Drop_event_sessionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_fulltext_catalog}.
 * @param ctx the parse tree
 */
fn enter_drop_fulltext_catalog(&mut self, _ctx: &Drop_fulltext_catalogContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_fulltext_catalog}.
 * @param ctx the parse tree
 */
fn exit_drop_fulltext_catalog(&mut self, _ctx: &Drop_fulltext_catalogContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_fulltext_index}.
 * @param ctx the parse tree
 */
fn enter_drop_fulltext_index(&mut self, _ctx: &Drop_fulltext_indexContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_fulltext_index}.
 * @param ctx the parse tree
 */
fn exit_drop_fulltext_index(&mut self, _ctx: &Drop_fulltext_indexContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_fulltext_stoplist}.
 * @param ctx the parse tree
 */
fn enter_drop_fulltext_stoplist(&mut self, _ctx: &Drop_fulltext_stoplistContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_fulltext_stoplist}.
 * @param ctx the parse tree
 */
fn exit_drop_fulltext_stoplist(&mut self, _ctx: &Drop_fulltext_stoplistContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_login}.
 * @param ctx the parse tree
 */
fn enter_drop_login(&mut self, _ctx: &Drop_loginContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_login}.
 * @param ctx the parse tree
 */
fn exit_drop_login(&mut self, _ctx: &Drop_loginContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_master_key}.
 * @param ctx the parse tree
 */
fn enter_drop_master_key(&mut self, _ctx: &Drop_master_keyContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_master_key}.
 * @param ctx the parse tree
 */
fn exit_drop_master_key(&mut self, _ctx: &Drop_master_keyContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_message_type}.
 * @param ctx the parse tree
 */
fn enter_drop_message_type(&mut self, _ctx: &Drop_message_typeContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_message_type}.
 * @param ctx the parse tree
 */
fn exit_drop_message_type(&mut self, _ctx: &Drop_message_typeContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_partition_function}.
 * @param ctx the parse tree
 */
fn enter_drop_partition_function(&mut self, _ctx: &Drop_partition_functionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_partition_function}.
 * @param ctx the parse tree
 */
fn exit_drop_partition_function(&mut self, _ctx: &Drop_partition_functionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_partition_scheme}.
 * @param ctx the parse tree
 */
fn enter_drop_partition_scheme(&mut self, _ctx: &Drop_partition_schemeContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_partition_scheme}.
 * @param ctx the parse tree
 */
fn exit_drop_partition_scheme(&mut self, _ctx: &Drop_partition_schemeContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_queue}.
 * @param ctx the parse tree
 */
fn enter_drop_queue(&mut self, _ctx: &Drop_queueContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_queue}.
 * @param ctx the parse tree
 */
fn exit_drop_queue(&mut self, _ctx: &Drop_queueContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_remote_service_binding}.
 * @param ctx the parse tree
 */
fn enter_drop_remote_service_binding(&mut self, _ctx: &Drop_remote_service_bindingContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_remote_service_binding}.
 * @param ctx the parse tree
 */
fn exit_drop_remote_service_binding(&mut self, _ctx: &Drop_remote_service_bindingContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_resource_pool}.
 * @param ctx the parse tree
 */
fn enter_drop_resource_pool(&mut self, _ctx: &Drop_resource_poolContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_resource_pool}.
 * @param ctx the parse tree
 */
fn exit_drop_resource_pool(&mut self, _ctx: &Drop_resource_poolContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_db_role}.
 * @param ctx the parse tree
 */
fn enter_drop_db_role(&mut self, _ctx: &Drop_db_roleContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_db_role}.
 * @param ctx the parse tree
 */
fn exit_drop_db_role(&mut self, _ctx: &Drop_db_roleContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_route}.
 * @param ctx the parse tree
 */
fn enter_drop_route(&mut self, _ctx: &Drop_routeContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_route}.
 * @param ctx the parse tree
 */
fn exit_drop_route(&mut self, _ctx: &Drop_routeContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_rule}.
 * @param ctx the parse tree
 */
fn enter_drop_rule(&mut self, _ctx: &Drop_ruleContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_rule}.
 * @param ctx the parse tree
 */
fn exit_drop_rule(&mut self, _ctx: &Drop_ruleContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_schema}.
 * @param ctx the parse tree
 */
fn enter_drop_schema(&mut self, _ctx: &Drop_schemaContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_schema}.
 * @param ctx the parse tree
 */
fn exit_drop_schema(&mut self, _ctx: &Drop_schemaContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_search_property_list}.
 * @param ctx the parse tree
 */
fn enter_drop_search_property_list(&mut self, _ctx: &Drop_search_property_listContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_search_property_list}.
 * @param ctx the parse tree
 */
fn exit_drop_search_property_list(&mut self, _ctx: &Drop_search_property_listContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_security_policy}.
 * @param ctx the parse tree
 */
fn enter_drop_security_policy(&mut self, _ctx: &Drop_security_policyContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_security_policy}.
 * @param ctx the parse tree
 */
fn exit_drop_security_policy(&mut self, _ctx: &Drop_security_policyContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_sequence}.
 * @param ctx the parse tree
 */
fn enter_drop_sequence(&mut self, _ctx: &Drop_sequenceContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_sequence}.
 * @param ctx the parse tree
 */
fn exit_drop_sequence(&mut self, _ctx: &Drop_sequenceContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_server_audit}.
 * @param ctx the parse tree
 */
fn enter_drop_server_audit(&mut self, _ctx: &Drop_server_auditContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_server_audit}.
 * @param ctx the parse tree
 */
fn exit_drop_server_audit(&mut self, _ctx: &Drop_server_auditContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_server_audit_specification}.
 * @param ctx the parse tree
 */
fn enter_drop_server_audit_specification(&mut self, _ctx: &Drop_server_audit_specificationContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_server_audit_specification}.
 * @param ctx the parse tree
 */
fn exit_drop_server_audit_specification(&mut self, _ctx: &Drop_server_audit_specificationContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_server_role}.
 * @param ctx the parse tree
 */
fn enter_drop_server_role(&mut self, _ctx: &Drop_server_roleContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_server_role}.
 * @param ctx the parse tree
 */
fn exit_drop_server_role(&mut self, _ctx: &Drop_server_roleContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_service}.
 * @param ctx the parse tree
 */
fn enter_drop_service(&mut self, _ctx: &Drop_serviceContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_service}.
 * @param ctx the parse tree
 */
fn exit_drop_service(&mut self, _ctx: &Drop_serviceContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_signature}.
 * @param ctx the parse tree
 */
fn enter_drop_signature(&mut self, _ctx: &Drop_signatureContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_signature}.
 * @param ctx the parse tree
 */
fn exit_drop_signature(&mut self, _ctx: &Drop_signatureContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_statistics_name_azure_dw_and_pdw}.
 * @param ctx the parse tree
 */
fn enter_drop_statistics_name_azure_dw_and_pdw(&mut self, _ctx: &Drop_statistics_name_azure_dw_and_pdwContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_statistics_name_azure_dw_and_pdw}.
 * @param ctx the parse tree
 */
fn exit_drop_statistics_name_azure_dw_and_pdw(&mut self, _ctx: &Drop_statistics_name_azure_dw_and_pdwContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_symmetric_key}.
 * @param ctx the parse tree
 */
fn enter_drop_symmetric_key(&mut self, _ctx: &Drop_symmetric_keyContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_symmetric_key}.
 * @param ctx the parse tree
 */
fn exit_drop_symmetric_key(&mut self, _ctx: &Drop_symmetric_keyContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_synonym}.
 * @param ctx the parse tree
 */
fn enter_drop_synonym(&mut self, _ctx: &Drop_synonymContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_synonym}.
 * @param ctx the parse tree
 */
fn exit_drop_synonym(&mut self, _ctx: &Drop_synonymContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_user}.
 * @param ctx the parse tree
 */
fn enter_drop_user(&mut self, _ctx: &Drop_userContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_user}.
 * @param ctx the parse tree
 */
fn exit_drop_user(&mut self, _ctx: &Drop_userContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_workload_group}.
 * @param ctx the parse tree
 */
fn enter_drop_workload_group(&mut self, _ctx: &Drop_workload_groupContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_workload_group}.
 * @param ctx the parse tree
 */
fn exit_drop_workload_group(&mut self, _ctx: &Drop_workload_groupContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_xml_schema_collection}.
 * @param ctx the parse tree
 */
fn enter_drop_xml_schema_collection(&mut self, _ctx: &Drop_xml_schema_collectionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_xml_schema_collection}.
 * @param ctx the parse tree
 */
fn exit_drop_xml_schema_collection(&mut self, _ctx: &Drop_xml_schema_collectionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#disable_trigger}.
 * @param ctx the parse tree
 */
fn enter_disable_trigger(&mut self, _ctx: &Disable_triggerContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#disable_trigger}.
 * @param ctx the parse tree
 */
fn exit_disable_trigger(&mut self, _ctx: &Disable_triggerContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#enable_trigger}.
 * @param ctx the parse tree
 */
fn enter_enable_trigger(&mut self, _ctx: &Enable_triggerContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#enable_trigger}.
 * @param ctx the parse tree
 */
fn exit_enable_trigger(&mut self, _ctx: &Enable_triggerContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#lock_table}.
 * @param ctx the parse tree
 */
fn enter_lock_table(&mut self, _ctx: &Lock_tableContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#lock_table}.
 * @param ctx the parse tree
 */
fn exit_lock_table(&mut self, _ctx: &Lock_tableContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#truncate_table}.
 * @param ctx the parse tree
 */
fn enter_truncate_table(&mut self, _ctx: &Truncate_tableContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#truncate_table}.
 * @param ctx the parse tree
 */
fn exit_truncate_table(&mut self, _ctx: &Truncate_tableContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_column_master_key}.
 * @param ctx the parse tree
 */
fn enter_create_column_master_key(&mut self, _ctx: &Create_column_master_keyContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_column_master_key}.
 * @param ctx the parse tree
 */
fn exit_create_column_master_key(&mut self, _ctx: &Create_column_master_keyContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_credential}.
 * @param ctx the parse tree
 */
fn enter_alter_credential(&mut self, _ctx: &Alter_credentialContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_credential}.
 * @param ctx the parse tree
 */
fn exit_alter_credential(&mut self, _ctx: &Alter_credentialContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_credential}.
 * @param ctx the parse tree
 */
fn enter_create_credential(&mut self, _ctx: &Create_credentialContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_credential}.
 * @param ctx the parse tree
 */
fn exit_create_credential(&mut self, _ctx: &Create_credentialContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_cryptographic_provider}.
 * @param ctx the parse tree
 */
fn enter_alter_cryptographic_provider(&mut self, _ctx: &Alter_cryptographic_providerContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_cryptographic_provider}.
 * @param ctx the parse tree
 */
fn exit_alter_cryptographic_provider(&mut self, _ctx: &Alter_cryptographic_providerContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_cryptographic_provider}.
 * @param ctx the parse tree
 */
fn enter_create_cryptographic_provider(&mut self, _ctx: &Create_cryptographic_providerContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_cryptographic_provider}.
 * @param ctx the parse tree
 */
fn exit_create_cryptographic_provider(&mut self, _ctx: &Create_cryptographic_providerContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_event_notification}.
 * @param ctx the parse tree
 */
fn enter_create_event_notification(&mut self, _ctx: &Create_event_notificationContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_event_notification}.
 * @param ctx the parse tree
 */
fn exit_create_event_notification(&mut self, _ctx: &Create_event_notificationContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_or_alter_event_session}.
 * @param ctx the parse tree
 */
fn enter_create_or_alter_event_session(&mut self, _ctx: &Create_or_alter_event_sessionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_or_alter_event_session}.
 * @param ctx the parse tree
 */
fn exit_create_or_alter_event_session(&mut self, _ctx: &Create_or_alter_event_sessionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#event_session_predicate_expression}.
 * @param ctx the parse tree
 */
fn enter_event_session_predicate_expression(&mut self, _ctx: &Event_session_predicate_expressionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#event_session_predicate_expression}.
 * @param ctx the parse tree
 */
fn exit_event_session_predicate_expression(&mut self, _ctx: &Event_session_predicate_expressionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#event_session_predicate_factor}.
 * @param ctx the parse tree
 */
fn enter_event_session_predicate_factor(&mut self, _ctx: &Event_session_predicate_factorContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#event_session_predicate_factor}.
 * @param ctx the parse tree
 */
fn exit_event_session_predicate_factor(&mut self, _ctx: &Event_session_predicate_factorContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#event_session_predicate_leaf}.
 * @param ctx the parse tree
 */
fn enter_event_session_predicate_leaf(&mut self, _ctx: &Event_session_predicate_leafContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#event_session_predicate_leaf}.
 * @param ctx the parse tree
 */
fn exit_event_session_predicate_leaf(&mut self, _ctx: &Event_session_predicate_leafContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_external_data_source}.
 * @param ctx the parse tree
 */
fn enter_alter_external_data_source(&mut self, _ctx: &Alter_external_data_sourceContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_external_data_source}.
 * @param ctx the parse tree
 */
fn exit_alter_external_data_source(&mut self, _ctx: &Alter_external_data_sourceContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_external_library}.
 * @param ctx the parse tree
 */
fn enter_alter_external_library(&mut self, _ctx: &Alter_external_libraryContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_external_library}.
 * @param ctx the parse tree
 */
fn exit_alter_external_library(&mut self, _ctx: &Alter_external_libraryContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_external_library}.
 * @param ctx the parse tree
 */
fn enter_create_external_library(&mut self, _ctx: &Create_external_libraryContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_external_library}.
 * @param ctx the parse tree
 */
fn exit_create_external_library(&mut self, _ctx: &Create_external_libraryContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_external_resource_pool}.
 * @param ctx the parse tree
 */
fn enter_alter_external_resource_pool(&mut self, _ctx: &Alter_external_resource_poolContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_external_resource_pool}.
 * @param ctx the parse tree
 */
fn exit_alter_external_resource_pool(&mut self, _ctx: &Alter_external_resource_poolContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_external_resource_pool}.
 * @param ctx the parse tree
 */
fn enter_create_external_resource_pool(&mut self, _ctx: &Create_external_resource_poolContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_external_resource_pool}.
 * @param ctx the parse tree
 */
fn exit_create_external_resource_pool(&mut self, _ctx: &Create_external_resource_poolContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_fulltext_catalog}.
 * @param ctx the parse tree
 */
fn enter_alter_fulltext_catalog(&mut self, _ctx: &Alter_fulltext_catalogContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_fulltext_catalog}.
 * @param ctx the parse tree
 */
fn exit_alter_fulltext_catalog(&mut self, _ctx: &Alter_fulltext_catalogContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_fulltext_catalog}.
 * @param ctx the parse tree
 */
fn enter_create_fulltext_catalog(&mut self, _ctx: &Create_fulltext_catalogContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_fulltext_catalog}.
 * @param ctx the parse tree
 */
fn exit_create_fulltext_catalog(&mut self, _ctx: &Create_fulltext_catalogContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_fulltext_stoplist}.
 * @param ctx the parse tree
 */
fn enter_alter_fulltext_stoplist(&mut self, _ctx: &Alter_fulltext_stoplistContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_fulltext_stoplist}.
 * @param ctx the parse tree
 */
fn exit_alter_fulltext_stoplist(&mut self, _ctx: &Alter_fulltext_stoplistContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_fulltext_stoplist}.
 * @param ctx the parse tree
 */
fn enter_create_fulltext_stoplist(&mut self, _ctx: &Create_fulltext_stoplistContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_fulltext_stoplist}.
 * @param ctx the parse tree
 */
fn exit_create_fulltext_stoplist(&mut self, _ctx: &Create_fulltext_stoplistContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_login_sql_server}.
 * @param ctx the parse tree
 */
fn enter_alter_login_sql_server(&mut self, _ctx: &Alter_login_sql_serverContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_login_sql_server}.
 * @param ctx the parse tree
 */
fn exit_alter_login_sql_server(&mut self, _ctx: &Alter_login_sql_serverContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_login_sql_server}.
 * @param ctx the parse tree
 */
fn enter_create_login_sql_server(&mut self, _ctx: &Create_login_sql_serverContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_login_sql_server}.
 * @param ctx the parse tree
 */
fn exit_create_login_sql_server(&mut self, _ctx: &Create_login_sql_serverContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_login_azure_sql}.
 * @param ctx the parse tree
 */
fn enter_alter_login_azure_sql(&mut self, _ctx: &Alter_login_azure_sqlContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_login_azure_sql}.
 * @param ctx the parse tree
 */
fn exit_alter_login_azure_sql(&mut self, _ctx: &Alter_login_azure_sqlContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_login_azure_sql}.
 * @param ctx the parse tree
 */
fn enter_create_login_azure_sql(&mut self, _ctx: &Create_login_azure_sqlContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_login_azure_sql}.
 * @param ctx the parse tree
 */
fn exit_create_login_azure_sql(&mut self, _ctx: &Create_login_azure_sqlContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_login_azure_sql_dw_and_pdw}.
 * @param ctx the parse tree
 */
fn enter_alter_login_azure_sql_dw_and_pdw(&mut self, _ctx: &Alter_login_azure_sql_dw_and_pdwContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_login_azure_sql_dw_and_pdw}.
 * @param ctx the parse tree
 */
fn exit_alter_login_azure_sql_dw_and_pdw(&mut self, _ctx: &Alter_login_azure_sql_dw_and_pdwContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_login_pdw}.
 * @param ctx the parse tree
 */
fn enter_create_login_pdw(&mut self, _ctx: &Create_login_pdwContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_login_pdw}.
 * @param ctx the parse tree
 */
fn exit_create_login_pdw(&mut self, _ctx: &Create_login_pdwContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_master_key_sql_server}.
 * @param ctx the parse tree
 */
fn enter_alter_master_key_sql_server(&mut self, _ctx: &Alter_master_key_sql_serverContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_master_key_sql_server}.
 * @param ctx the parse tree
 */
fn exit_alter_master_key_sql_server(&mut self, _ctx: &Alter_master_key_sql_serverContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_master_key_sql_server}.
 * @param ctx the parse tree
 */
fn enter_create_master_key_sql_server(&mut self, _ctx: &Create_master_key_sql_serverContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_master_key_sql_server}.
 * @param ctx the parse tree
 */
fn exit_create_master_key_sql_server(&mut self, _ctx: &Create_master_key_sql_serverContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_master_key_azure_sql}.
 * @param ctx the parse tree
 */
fn enter_alter_master_key_azure_sql(&mut self, _ctx: &Alter_master_key_azure_sqlContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_master_key_azure_sql}.
 * @param ctx the parse tree
 */
fn exit_alter_master_key_azure_sql(&mut self, _ctx: &Alter_master_key_azure_sqlContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_master_key_azure_sql}.
 * @param ctx the parse tree
 */
fn enter_create_master_key_azure_sql(&mut self, _ctx: &Create_master_key_azure_sqlContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_master_key_azure_sql}.
 * @param ctx the parse tree
 */
fn exit_create_master_key_azure_sql(&mut self, _ctx: &Create_master_key_azure_sqlContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_message_type}.
 * @param ctx the parse tree
 */
fn enter_alter_message_type(&mut self, _ctx: &Alter_message_typeContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_message_type}.
 * @param ctx the parse tree
 */
fn exit_alter_message_type(&mut self, _ctx: &Alter_message_typeContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_partition_function}.
 * @param ctx the parse tree
 */
fn enter_alter_partition_function(&mut self, _ctx: &Alter_partition_functionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_partition_function}.
 * @param ctx the parse tree
 */
fn exit_alter_partition_function(&mut self, _ctx: &Alter_partition_functionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_partition_scheme}.
 * @param ctx the parse tree
 */
fn enter_alter_partition_scheme(&mut self, _ctx: &Alter_partition_schemeContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_partition_scheme}.
 * @param ctx the parse tree
 */
fn exit_alter_partition_scheme(&mut self, _ctx: &Alter_partition_schemeContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_remote_service_binding}.
 * @param ctx the parse tree
 */
fn enter_alter_remote_service_binding(&mut self, _ctx: &Alter_remote_service_bindingContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_remote_service_binding}.
 * @param ctx the parse tree
 */
fn exit_alter_remote_service_binding(&mut self, _ctx: &Alter_remote_service_bindingContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_remote_service_binding}.
 * @param ctx the parse tree
 */
fn enter_create_remote_service_binding(&mut self, _ctx: &Create_remote_service_bindingContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_remote_service_binding}.
 * @param ctx the parse tree
 */
fn exit_create_remote_service_binding(&mut self, _ctx: &Create_remote_service_bindingContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_resource_pool}.
 * @param ctx the parse tree
 */
fn enter_create_resource_pool(&mut self, _ctx: &Create_resource_poolContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_resource_pool}.
 * @param ctx the parse tree
 */
fn exit_create_resource_pool(&mut self, _ctx: &Create_resource_poolContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_resource_governor}.
 * @param ctx the parse tree
 */
fn enter_alter_resource_governor(&mut self, _ctx: &Alter_resource_governorContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_resource_governor}.
 * @param ctx the parse tree
 */
fn exit_alter_resource_governor(&mut self, _ctx: &Alter_resource_governorContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_db_role}.
 * @param ctx the parse tree
 */
fn enter_alter_db_role(&mut self, _ctx: &Alter_db_roleContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_db_role}.
 * @param ctx the parse tree
 */
fn exit_alter_db_role(&mut self, _ctx: &Alter_db_roleContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_db_role}.
 * @param ctx the parse tree
 */
fn enter_create_db_role(&mut self, _ctx: &Create_db_roleContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_db_role}.
 * @param ctx the parse tree
 */
fn exit_create_db_role(&mut self, _ctx: &Create_db_roleContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_route}.
 * @param ctx the parse tree
 */
fn enter_create_route(&mut self, _ctx: &Create_routeContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_route}.
 * @param ctx the parse tree
 */
fn exit_create_route(&mut self, _ctx: &Create_routeContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_rule}.
 * @param ctx the parse tree
 */
fn enter_create_rule(&mut self, _ctx: &Create_ruleContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_rule}.
 * @param ctx the parse tree
 */
fn exit_create_rule(&mut self, _ctx: &Create_ruleContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_schema_sql}.
 * @param ctx the parse tree
 */
fn enter_alter_schema_sql(&mut self, _ctx: &Alter_schema_sqlContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_schema_sql}.
 * @param ctx the parse tree
 */
fn exit_alter_schema_sql(&mut self, _ctx: &Alter_schema_sqlContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_schema}.
 * @param ctx the parse tree
 */
fn enter_create_schema(&mut self, _ctx: &Create_schemaContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_schema}.
 * @param ctx the parse tree
 */
fn exit_create_schema(&mut self, _ctx: &Create_schemaContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_schema_azure_sql_dw_and_pdw}.
 * @param ctx the parse tree
 */
fn enter_create_schema_azure_sql_dw_and_pdw(&mut self, _ctx: &Create_schema_azure_sql_dw_and_pdwContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_schema_azure_sql_dw_and_pdw}.
 * @param ctx the parse tree
 */
fn exit_create_schema_azure_sql_dw_and_pdw(&mut self, _ctx: &Create_schema_azure_sql_dw_and_pdwContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_schema_azure_sql_dw_and_pdw}.
 * @param ctx the parse tree
 */
fn enter_alter_schema_azure_sql_dw_and_pdw(&mut self, _ctx: &Alter_schema_azure_sql_dw_and_pdwContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_schema_azure_sql_dw_and_pdw}.
 * @param ctx the parse tree
 */
fn exit_alter_schema_azure_sql_dw_and_pdw(&mut self, _ctx: &Alter_schema_azure_sql_dw_and_pdwContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_search_property_list}.
 * @param ctx the parse tree
 */
fn enter_create_search_property_list(&mut self, _ctx: &Create_search_property_listContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_search_property_list}.
 * @param ctx the parse tree
 */
fn exit_create_search_property_list(&mut self, _ctx: &Create_search_property_listContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_security_policy}.
 * @param ctx the parse tree
 */
fn enter_create_security_policy(&mut self, _ctx: &Create_security_policyContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_security_policy}.
 * @param ctx the parse tree
 */
fn exit_create_security_policy(&mut self, _ctx: &Create_security_policyContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_sequence}.
 * @param ctx the parse tree
 */
fn enter_alter_sequence(&mut self, _ctx: &Alter_sequenceContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_sequence}.
 * @param ctx the parse tree
 */
fn exit_alter_sequence(&mut self, _ctx: &Alter_sequenceContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_sequence}.
 * @param ctx the parse tree
 */
fn enter_create_sequence(&mut self, _ctx: &Create_sequenceContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_sequence}.
 * @param ctx the parse tree
 */
fn exit_create_sequence(&mut self, _ctx: &Create_sequenceContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_server_audit}.
 * @param ctx the parse tree
 */
fn enter_alter_server_audit(&mut self, _ctx: &Alter_server_auditContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_server_audit}.
 * @param ctx the parse tree
 */
fn exit_alter_server_audit(&mut self, _ctx: &Alter_server_auditContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_server_audit}.
 * @param ctx the parse tree
 */
fn enter_create_server_audit(&mut self, _ctx: &Create_server_auditContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_server_audit}.
 * @param ctx the parse tree
 */
fn exit_create_server_audit(&mut self, _ctx: &Create_server_auditContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_server_audit_specification}.
 * @param ctx the parse tree
 */
fn enter_alter_server_audit_specification(&mut self, _ctx: &Alter_server_audit_specificationContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_server_audit_specification}.
 * @param ctx the parse tree
 */
fn exit_alter_server_audit_specification(&mut self, _ctx: &Alter_server_audit_specificationContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_server_audit_specification}.
 * @param ctx the parse tree
 */
fn enter_create_server_audit_specification(&mut self, _ctx: &Create_server_audit_specificationContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_server_audit_specification}.
 * @param ctx the parse tree
 */
fn exit_create_server_audit_specification(&mut self, _ctx: &Create_server_audit_specificationContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_server_configuration}.
 * @param ctx the parse tree
 */
fn enter_alter_server_configuration(&mut self, _ctx: &Alter_server_configurationContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_server_configuration}.
 * @param ctx the parse tree
 */
fn exit_alter_server_configuration(&mut self, _ctx: &Alter_server_configurationContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_server_role}.
 * @param ctx the parse tree
 */
fn enter_alter_server_role(&mut self, _ctx: &Alter_server_roleContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_server_role}.
 * @param ctx the parse tree
 */
fn exit_alter_server_role(&mut self, _ctx: &Alter_server_roleContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_server_role}.
 * @param ctx the parse tree
 */
fn enter_create_server_role(&mut self, _ctx: &Create_server_roleContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_server_role}.
 * @param ctx the parse tree
 */
fn exit_create_server_role(&mut self, _ctx: &Create_server_roleContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_server_role_pdw}.
 * @param ctx the parse tree
 */
fn enter_alter_server_role_pdw(&mut self, _ctx: &Alter_server_role_pdwContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_server_role_pdw}.
 * @param ctx the parse tree
 */
fn exit_alter_server_role_pdw(&mut self, _ctx: &Alter_server_role_pdwContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_service}.
 * @param ctx the parse tree
 */
fn enter_alter_service(&mut self, _ctx: &Alter_serviceContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_service}.
 * @param ctx the parse tree
 */
fn exit_alter_service(&mut self, _ctx: &Alter_serviceContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_service}.
 * @param ctx the parse tree
 */
fn enter_create_service(&mut self, _ctx: &Create_serviceContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_service}.
 * @param ctx the parse tree
 */
fn exit_create_service(&mut self, _ctx: &Create_serviceContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_service_master_key}.
 * @param ctx the parse tree
 */
fn enter_alter_service_master_key(&mut self, _ctx: &Alter_service_master_keyContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_service_master_key}.
 * @param ctx the parse tree
 */
fn exit_alter_service_master_key(&mut self, _ctx: &Alter_service_master_keyContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_symmetric_key}.
 * @param ctx the parse tree
 */
fn enter_alter_symmetric_key(&mut self, _ctx: &Alter_symmetric_keyContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_symmetric_key}.
 * @param ctx the parse tree
 */
fn exit_alter_symmetric_key(&mut self, _ctx: &Alter_symmetric_keyContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_symmetric_key}.
 * @param ctx the parse tree
 */
fn enter_create_symmetric_key(&mut self, _ctx: &Create_symmetric_keyContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_symmetric_key}.
 * @param ctx the parse tree
 */
fn exit_create_symmetric_key(&mut self, _ctx: &Create_symmetric_keyContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_synonym}.
 * @param ctx the parse tree
 */
fn enter_create_synonym(&mut self, _ctx: &Create_synonymContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_synonym}.
 * @param ctx the parse tree
 */
fn exit_create_synonym(&mut self, _ctx: &Create_synonymContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_user}.
 * @param ctx the parse tree
 */
fn enter_alter_user(&mut self, _ctx: &Alter_userContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_user}.
 * @param ctx the parse tree
 */
fn exit_alter_user(&mut self, _ctx: &Alter_userContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_user}.
 * @param ctx the parse tree
 */
fn enter_create_user(&mut self, _ctx: &Create_userContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_user}.
 * @param ctx the parse tree
 */
fn exit_create_user(&mut self, _ctx: &Create_userContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_user_azure_sql_dw}.
 * @param ctx the parse tree
 */
fn enter_create_user_azure_sql_dw(&mut self, _ctx: &Create_user_azure_sql_dwContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_user_azure_sql_dw}.
 * @param ctx the parse tree
 */
fn exit_create_user_azure_sql_dw(&mut self, _ctx: &Create_user_azure_sql_dwContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_user_azure_sql}.
 * @param ctx the parse tree
 */
fn enter_alter_user_azure_sql(&mut self, _ctx: &Alter_user_azure_sqlContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_user_azure_sql}.
 * @param ctx the parse tree
 */
fn exit_alter_user_azure_sql(&mut self, _ctx: &Alter_user_azure_sqlContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_workload_group}.
 * @param ctx the parse tree
 */
fn enter_alter_workload_group(&mut self, _ctx: &Alter_workload_groupContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_workload_group}.
 * @param ctx the parse tree
 */
fn exit_alter_workload_group(&mut self, _ctx: &Alter_workload_groupContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_workload_group}.
 * @param ctx the parse tree
 */
fn enter_create_workload_group(&mut self, _ctx: &Create_workload_groupContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_workload_group}.
 * @param ctx the parse tree
 */
fn exit_create_workload_group(&mut self, _ctx: &Create_workload_groupContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_xml_schema_collection}.
 * @param ctx the parse tree
 */
fn enter_create_xml_schema_collection(&mut self, _ctx: &Create_xml_schema_collectionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_xml_schema_collection}.
 * @param ctx the parse tree
 */
fn exit_create_xml_schema_collection(&mut self, _ctx: &Create_xml_schema_collectionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_queue}.
 * @param ctx the parse tree
 */
fn enter_create_queue(&mut self, _ctx: &Create_queueContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_queue}.
 * @param ctx the parse tree
 */
fn exit_create_queue(&mut self, _ctx: &Create_queueContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#queue_settings}.
 * @param ctx the parse tree
 */
fn enter_queue_settings(&mut self, _ctx: &Queue_settingsContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#queue_settings}.
 * @param ctx the parse tree
 */
fn exit_queue_settings(&mut self, _ctx: &Queue_settingsContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_queue}.
 * @param ctx the parse tree
 */
fn enter_alter_queue(&mut self, _ctx: &Alter_queueContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_queue}.
 * @param ctx the parse tree
 */
fn exit_alter_queue(&mut self, _ctx: &Alter_queueContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#queue_action}.
 * @param ctx the parse tree
 */
fn enter_queue_action(&mut self, _ctx: &Queue_actionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#queue_action}.
 * @param ctx the parse tree
 */
fn exit_queue_action(&mut self, _ctx: &Queue_actionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#queue_rebuild_options}.
 * @param ctx the parse tree
 */
fn enter_queue_rebuild_options(&mut self, _ctx: &Queue_rebuild_optionsContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#queue_rebuild_options}.
 * @param ctx the parse tree
 */
fn exit_queue_rebuild_options(&mut self, _ctx: &Queue_rebuild_optionsContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_contract}.
 * @param ctx the parse tree
 */
fn enter_create_contract(&mut self, _ctx: &Create_contractContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_contract}.
 * @param ctx the parse tree
 */
fn exit_create_contract(&mut self, _ctx: &Create_contractContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#conversation_statement}.
 * @param ctx the parse tree
 */
fn enter_conversation_statement(&mut self, _ctx: &Conversation_statementContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#conversation_statement}.
 * @param ctx the parse tree
 */
fn exit_conversation_statement(&mut self, _ctx: &Conversation_statementContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#message_statement}.
 * @param ctx the parse tree
 */
fn enter_message_statement(&mut self, _ctx: &Message_statementContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#message_statement}.
 * @param ctx the parse tree
 */
fn exit_message_statement(&mut self, _ctx: &Message_statementContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#merge_statement}.
 * @param ctx the parse tree
 */
fn enter_merge_statement(&mut self, _ctx: &Merge_statementContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#merge_statement}.
 * @param ctx the parse tree
 */
fn exit_merge_statement(&mut self, _ctx: &Merge_statementContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#merge_matched}.
 * @param ctx the parse tree
 */
fn enter_merge_matched(&mut self, _ctx: &Merge_matchedContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#merge_matched}.
 * @param ctx the parse tree
 */
fn exit_merge_matched(&mut self, _ctx: &Merge_matchedContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#merge_not_matched}.
 * @param ctx the parse tree
 */
fn enter_merge_not_matched(&mut self, _ctx: &Merge_not_matchedContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#merge_not_matched}.
 * @param ctx the parse tree
 */
fn exit_merge_not_matched(&mut self, _ctx: &Merge_not_matchedContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#delete_statement}.
 * @param ctx the parse tree
 */
fn enter_delete_statement(&mut self, _ctx: &Delete_statementContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#delete_statement}.
 * @param ctx the parse tree
 */
fn exit_delete_statement(&mut self, _ctx: &Delete_statementContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#delete_statement_from}.
 * @param ctx the parse tree
 */
fn enter_delete_statement_from(&mut self, _ctx: &Delete_statement_fromContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#delete_statement_from}.
 * @param ctx the parse tree
 */
fn exit_delete_statement_from(&mut self, _ctx: &Delete_statement_fromContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#insert_statement}.
 * @param ctx the parse tree
 */
fn enter_insert_statement(&mut self, _ctx: &Insert_statementContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#insert_statement}.
 * @param ctx the parse tree
 */
fn exit_insert_statement(&mut self, _ctx: &Insert_statementContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#insert_statement_value}.
 * @param ctx the parse tree
 */
fn enter_insert_statement_value(&mut self, _ctx: &Insert_statement_valueContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#insert_statement_value}.
 * @param ctx the parse tree
 */
fn exit_insert_statement_value(&mut self, _ctx: &Insert_statement_valueContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#receive_statement}.
 * @param ctx the parse tree
 */
fn enter_receive_statement(&mut self, _ctx: &Receive_statementContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#receive_statement}.
 * @param ctx the parse tree
 */
fn exit_receive_statement(&mut self, _ctx: &Receive_statementContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#select_statement}.
 * @param ctx the parse tree
 */
fn enter_select_statement(&mut self, _ctx: &Select_statementContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#select_statement}.
 * @param ctx the parse tree
 */
fn exit_select_statement(&mut self, _ctx: &Select_statementContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#time}.
 * @param ctx the parse tree
 */
fn enter_time(&mut self, _ctx: &TimeContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#time}.
 * @param ctx the parse tree
 */
fn exit_time(&mut self, _ctx: &TimeContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#update_statement}.
 * @param ctx the parse tree
 */
fn enter_update_statement(&mut self, _ctx: &Update_statementContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#update_statement}.
 * @param ctx the parse tree
 */
fn exit_update_statement(&mut self, _ctx: &Update_statementContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#output_clause}.
 * @param ctx the parse tree
 */
fn enter_output_clause(&mut self, _ctx: &Output_clauseContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#output_clause}.
 * @param ctx the parse tree
 */
fn exit_output_clause(&mut self, _ctx: &Output_clauseContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#output_dml_list_elem}.
 * @param ctx the parse tree
 */
fn enter_output_dml_list_elem(&mut self, _ctx: &Output_dml_list_elemContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#output_dml_list_elem}.
 * @param ctx the parse tree
 */
fn exit_output_dml_list_elem(&mut self, _ctx: &Output_dml_list_elemContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#output_column_name}.
 * @param ctx the parse tree
 */
fn enter_output_column_name(&mut self, _ctx: &Output_column_nameContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#output_column_name}.
 * @param ctx the parse tree
 */
fn exit_output_column_name(&mut self, _ctx: &Output_column_nameContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_database}.
 * @param ctx the parse tree
 */
fn enter_create_database(&mut self, _ctx: &Create_databaseContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_database}.
 * @param ctx the parse tree
 */
fn exit_create_database(&mut self, _ctx: &Create_databaseContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_index}.
 * @param ctx the parse tree
 */
fn enter_create_index(&mut self, _ctx: &Create_indexContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_index}.
 * @param ctx the parse tree
 */
fn exit_create_index(&mut self, _ctx: &Create_indexContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_or_alter_procedure}.
 * @param ctx the parse tree
 */
fn enter_create_or_alter_procedure(&mut self, _ctx: &Create_or_alter_procedureContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_or_alter_procedure}.
 * @param ctx the parse tree
 */
fn exit_create_or_alter_procedure(&mut self, _ctx: &Create_or_alter_procedureContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_or_alter_trigger}.
 * @param ctx the parse tree
 */
fn enter_create_or_alter_trigger(&mut self, _ctx: &Create_or_alter_triggerContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_or_alter_trigger}.
 * @param ctx the parse tree
 */
fn exit_create_or_alter_trigger(&mut self, _ctx: &Create_or_alter_triggerContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_or_alter_dml_trigger}.
 * @param ctx the parse tree
 */
fn enter_create_or_alter_dml_trigger(&mut self, _ctx: &Create_or_alter_dml_triggerContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_or_alter_dml_trigger}.
 * @param ctx the parse tree
 */
fn exit_create_or_alter_dml_trigger(&mut self, _ctx: &Create_or_alter_dml_triggerContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#dml_trigger_option}.
 * @param ctx the parse tree
 */
fn enter_dml_trigger_option(&mut self, _ctx: &Dml_trigger_optionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#dml_trigger_option}.
 * @param ctx the parse tree
 */
fn exit_dml_trigger_option(&mut self, _ctx: &Dml_trigger_optionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#dml_trigger_operation}.
 * @param ctx the parse tree
 */
fn enter_dml_trigger_operation(&mut self, _ctx: &Dml_trigger_operationContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#dml_trigger_operation}.
 * @param ctx the parse tree
 */
fn exit_dml_trigger_operation(&mut self, _ctx: &Dml_trigger_operationContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_or_alter_ddl_trigger}.
 * @param ctx the parse tree
 */
fn enter_create_or_alter_ddl_trigger(&mut self, _ctx: &Create_or_alter_ddl_triggerContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_or_alter_ddl_trigger}.
 * @param ctx the parse tree
 */
fn exit_create_or_alter_ddl_trigger(&mut self, _ctx: &Create_or_alter_ddl_triggerContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#ddl_trigger_operation}.
 * @param ctx the parse tree
 */
fn enter_ddl_trigger_operation(&mut self, _ctx: &Ddl_trigger_operationContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#ddl_trigger_operation}.
 * @param ctx the parse tree
 */
fn exit_ddl_trigger_operation(&mut self, _ctx: &Ddl_trigger_operationContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_or_alter_function}.
 * @param ctx the parse tree
 */
fn enter_create_or_alter_function(&mut self, _ctx: &Create_or_alter_functionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_or_alter_function}.
 * @param ctx the parse tree
 */
fn exit_create_or_alter_function(&mut self, _ctx: &Create_or_alter_functionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#func_body_returns_select}.
 * @param ctx the parse tree
 */
fn enter_func_body_returns_select(&mut self, _ctx: &Func_body_returns_selectContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#func_body_returns_select}.
 * @param ctx the parse tree
 */
fn exit_func_body_returns_select(&mut self, _ctx: &Func_body_returns_selectContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#func_body_returns_table}.
 * @param ctx the parse tree
 */
fn enter_func_body_returns_table(&mut self, _ctx: &Func_body_returns_tableContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#func_body_returns_table}.
 * @param ctx the parse tree
 */
fn exit_func_body_returns_table(&mut self, _ctx: &Func_body_returns_tableContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#func_body_returns_scalar}.
 * @param ctx the parse tree
 */
fn enter_func_body_returns_scalar(&mut self, _ctx: &Func_body_returns_scalarContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#func_body_returns_scalar}.
 * @param ctx the parse tree
 */
fn exit_func_body_returns_scalar(&mut self, _ctx: &Func_body_returns_scalarContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#procedure_param}.
 * @param ctx the parse tree
 */
fn enter_procedure_param(&mut self, _ctx: &Procedure_paramContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#procedure_param}.
 * @param ctx the parse tree
 */
fn exit_procedure_param(&mut self, _ctx: &Procedure_paramContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#procedure_option}.
 * @param ctx the parse tree
 */
fn enter_procedure_option(&mut self, _ctx: &Procedure_optionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#procedure_option}.
 * @param ctx the parse tree
 */
fn exit_procedure_option(&mut self, _ctx: &Procedure_optionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#function_option}.
 * @param ctx the parse tree
 */
fn enter_function_option(&mut self, _ctx: &Function_optionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#function_option}.
 * @param ctx the parse tree
 */
fn exit_function_option(&mut self, _ctx: &Function_optionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_statistics}.
 * @param ctx the parse tree
 */
fn enter_create_statistics(&mut self, _ctx: &Create_statisticsContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_statistics}.
 * @param ctx the parse tree
 */
fn exit_create_statistics(&mut self, _ctx: &Create_statisticsContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#update_statistics}.
 * @param ctx the parse tree
 */
fn enter_update_statistics(&mut self, _ctx: &Update_statisticsContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#update_statistics}.
 * @param ctx the parse tree
 */
fn exit_update_statistics(&mut self, _ctx: &Update_statisticsContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_table}.
 * @param ctx the parse tree
 */
fn enter_create_table(&mut self, _ctx: &Create_tableContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_table}.
 * @param ctx the parse tree
 */
fn exit_create_table(&mut self, _ctx: &Create_tableContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#table_options}.
 * @param ctx the parse tree
 */
fn enter_table_options(&mut self, _ctx: &Table_optionsContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#table_options}.
 * @param ctx the parse tree
 */
fn exit_table_options(&mut self, _ctx: &Table_optionsContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_view}.
 * @param ctx the parse tree
 */
fn enter_create_view(&mut self, _ctx: &Create_viewContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_view}.
 * @param ctx the parse tree
 */
fn exit_create_view(&mut self, _ctx: &Create_viewContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#view_attribute}.
 * @param ctx the parse tree
 */
fn enter_view_attribute(&mut self, _ctx: &View_attributeContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#view_attribute}.
 * @param ctx the parse tree
 */
fn exit_view_attribute(&mut self, _ctx: &View_attributeContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_table}.
 * @param ctx the parse tree
 */
fn enter_alter_table(&mut self, _ctx: &Alter_tableContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_table}.
 * @param ctx the parse tree
 */
fn exit_alter_table(&mut self, _ctx: &Alter_tableContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_database}.
 * @param ctx the parse tree
 */
fn enter_alter_database(&mut self, _ctx: &Alter_databaseContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_database}.
 * @param ctx the parse tree
 */
fn exit_alter_database(&mut self, _ctx: &Alter_databaseContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#database_optionspec}.
 * @param ctx the parse tree
 */
fn enter_database_optionspec(&mut self, _ctx: &Database_optionspecContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#database_optionspec}.
 * @param ctx the parse tree
 */
fn exit_database_optionspec(&mut self, _ctx: &Database_optionspecContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#auto_option}.
 * @param ctx the parse tree
 */
fn enter_auto_option(&mut self, _ctx: &Auto_optionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#auto_option}.
 * @param ctx the parse tree
 */
fn exit_auto_option(&mut self, _ctx: &Auto_optionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#change_tracking_option}.
 * @param ctx the parse tree
 */
fn enter_change_tracking_option(&mut self, _ctx: &Change_tracking_optionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#change_tracking_option}.
 * @param ctx the parse tree
 */
fn exit_change_tracking_option(&mut self, _ctx: &Change_tracking_optionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#change_tracking_option_list}.
 * @param ctx the parse tree
 */
fn enter_change_tracking_option_list(&mut self, _ctx: &Change_tracking_option_listContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#change_tracking_option_list}.
 * @param ctx the parse tree
 */
fn exit_change_tracking_option_list(&mut self, _ctx: &Change_tracking_option_listContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#containment_option}.
 * @param ctx the parse tree
 */
fn enter_containment_option(&mut self, _ctx: &Containment_optionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#containment_option}.
 * @param ctx the parse tree
 */
fn exit_containment_option(&mut self, _ctx: &Containment_optionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#cursor_option}.
 * @param ctx the parse tree
 */
fn enter_cursor_option(&mut self, _ctx: &Cursor_optionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#cursor_option}.
 * @param ctx the parse tree
 */
fn exit_cursor_option(&mut self, _ctx: &Cursor_optionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#alter_endpoint}.
 * @param ctx the parse tree
 */
fn enter_alter_endpoint(&mut self, _ctx: &Alter_endpointContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#alter_endpoint}.
 * @param ctx the parse tree
 */
fn exit_alter_endpoint(&mut self, _ctx: &Alter_endpointContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#database_mirroring_option}.
 * @param ctx the parse tree
 */
fn enter_database_mirroring_option(&mut self, _ctx: &Database_mirroring_optionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#database_mirroring_option}.
 * @param ctx the parse tree
 */
fn exit_database_mirroring_option(&mut self, _ctx: &Database_mirroring_optionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#mirroring_set_option}.
 * @param ctx the parse tree
 */
fn enter_mirroring_set_option(&mut self, _ctx: &Mirroring_set_optionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#mirroring_set_option}.
 * @param ctx the parse tree
 */
fn exit_mirroring_set_option(&mut self, _ctx: &Mirroring_set_optionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#mirroring_partner}.
 * @param ctx the parse tree
 */
fn enter_mirroring_partner(&mut self, _ctx: &Mirroring_partnerContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#mirroring_partner}.
 * @param ctx the parse tree
 */
fn exit_mirroring_partner(&mut self, _ctx: &Mirroring_partnerContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#mirroring_witness}.
 * @param ctx the parse tree
 */
fn enter_mirroring_witness(&mut self, _ctx: &Mirroring_witnessContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#mirroring_witness}.
 * @param ctx the parse tree
 */
fn exit_mirroring_witness(&mut self, _ctx: &Mirroring_witnessContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#witness_partner_equal}.
 * @param ctx the parse tree
 */
fn enter_witness_partner_equal(&mut self, _ctx: &Witness_partner_equalContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#witness_partner_equal}.
 * @param ctx the parse tree
 */
fn exit_witness_partner_equal(&mut self, _ctx: &Witness_partner_equalContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#partner_option}.
 * @param ctx the parse tree
 */
fn enter_partner_option(&mut self, _ctx: &Partner_optionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#partner_option}.
 * @param ctx the parse tree
 */
fn exit_partner_option(&mut self, _ctx: &Partner_optionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#witness_option}.
 * @param ctx the parse tree
 */
fn enter_witness_option(&mut self, _ctx: &Witness_optionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#witness_option}.
 * @param ctx the parse tree
 */
fn exit_witness_option(&mut self, _ctx: &Witness_optionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#witness_server}.
 * @param ctx the parse tree
 */
fn enter_witness_server(&mut self, _ctx: &Witness_serverContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#witness_server}.
 * @param ctx the parse tree
 */
fn exit_witness_server(&mut self, _ctx: &Witness_serverContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#partner_server}.
 * @param ctx the parse tree
 */
fn enter_partner_server(&mut self, _ctx: &Partner_serverContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#partner_server}.
 * @param ctx the parse tree
 */
fn exit_partner_server(&mut self, _ctx: &Partner_serverContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#mirroring_host_port_seperator}.
 * @param ctx the parse tree
 */
fn enter_mirroring_host_port_seperator(&mut self, _ctx: &Mirroring_host_port_seperatorContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#mirroring_host_port_seperator}.
 * @param ctx the parse tree
 */
fn exit_mirroring_host_port_seperator(&mut self, _ctx: &Mirroring_host_port_seperatorContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#partner_server_tcp_prefix}.
 * @param ctx the parse tree
 */
fn enter_partner_server_tcp_prefix(&mut self, _ctx: &Partner_server_tcp_prefixContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#partner_server_tcp_prefix}.
 * @param ctx the parse tree
 */
fn exit_partner_server_tcp_prefix(&mut self, _ctx: &Partner_server_tcp_prefixContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#port_number}.
 * @param ctx the parse tree
 */
fn enter_port_number(&mut self, _ctx: &Port_numberContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#port_number}.
 * @param ctx the parse tree
 */
fn exit_port_number(&mut self, _ctx: &Port_numberContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#host}.
 * @param ctx the parse tree
 */
fn enter_host(&mut self, _ctx: &HostContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#host}.
 * @param ctx the parse tree
 */
fn exit_host(&mut self, _ctx: &HostContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#date_correlation_optimization_option}.
 * @param ctx the parse tree
 */
fn enter_date_correlation_optimization_option(&mut self, _ctx: &Date_correlation_optimization_optionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#date_correlation_optimization_option}.
 * @param ctx the parse tree
 */
fn exit_date_correlation_optimization_option(&mut self, _ctx: &Date_correlation_optimization_optionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#db_encryption_option}.
 * @param ctx the parse tree
 */
fn enter_db_encryption_option(&mut self, _ctx: &Db_encryption_optionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#db_encryption_option}.
 * @param ctx the parse tree
 */
fn exit_db_encryption_option(&mut self, _ctx: &Db_encryption_optionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#db_state_option}.
 * @param ctx the parse tree
 */
fn enter_db_state_option(&mut self, _ctx: &Db_state_optionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#db_state_option}.
 * @param ctx the parse tree
 */
fn exit_db_state_option(&mut self, _ctx: &Db_state_optionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#db_update_option}.
 * @param ctx the parse tree
 */
fn enter_db_update_option(&mut self, _ctx: &Db_update_optionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#db_update_option}.
 * @param ctx the parse tree
 */
fn exit_db_update_option(&mut self, _ctx: &Db_update_optionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#db_user_access_option}.
 * @param ctx the parse tree
 */
fn enter_db_user_access_option(&mut self, _ctx: &Db_user_access_optionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#db_user_access_option}.
 * @param ctx the parse tree
 */
fn exit_db_user_access_option(&mut self, _ctx: &Db_user_access_optionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#delayed_durability_option}.
 * @param ctx the parse tree
 */
fn enter_delayed_durability_option(&mut self, _ctx: &Delayed_durability_optionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#delayed_durability_option}.
 * @param ctx the parse tree
 */
fn exit_delayed_durability_option(&mut self, _ctx: &Delayed_durability_optionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#external_access_option}.
 * @param ctx the parse tree
 */
fn enter_external_access_option(&mut self, _ctx: &External_access_optionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#external_access_option}.
 * @param ctx the parse tree
 */
fn exit_external_access_option(&mut self, _ctx: &External_access_optionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#hadr_options}.
 * @param ctx the parse tree
 */
fn enter_hadr_options(&mut self, _ctx: &Hadr_optionsContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#hadr_options}.
 * @param ctx the parse tree
 */
fn exit_hadr_options(&mut self, _ctx: &Hadr_optionsContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#mixed_page_allocation_option}.
 * @param ctx the parse tree
 */
fn enter_mixed_page_allocation_option(&mut self, _ctx: &Mixed_page_allocation_optionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#mixed_page_allocation_option}.
 * @param ctx the parse tree
 */
fn exit_mixed_page_allocation_option(&mut self, _ctx: &Mixed_page_allocation_optionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#parameterization_option}.
 * @param ctx the parse tree
 */
fn enter_parameterization_option(&mut self, _ctx: &Parameterization_optionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#parameterization_option}.
 * @param ctx the parse tree
 */
fn exit_parameterization_option(&mut self, _ctx: &Parameterization_optionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#recovery_option}.
 * @param ctx the parse tree
 */
fn enter_recovery_option(&mut self, _ctx: &Recovery_optionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#recovery_option}.
 * @param ctx the parse tree
 */
fn exit_recovery_option(&mut self, _ctx: &Recovery_optionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#service_broker_option}.
 * @param ctx the parse tree
 */
fn enter_service_broker_option(&mut self, _ctx: &Service_broker_optionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#service_broker_option}.
 * @param ctx the parse tree
 */
fn exit_service_broker_option(&mut self, _ctx: &Service_broker_optionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#snapshot_option}.
 * @param ctx the parse tree
 */
fn enter_snapshot_option(&mut self, _ctx: &Snapshot_optionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#snapshot_option}.
 * @param ctx the parse tree
 */
fn exit_snapshot_option(&mut self, _ctx: &Snapshot_optionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#sql_option}.
 * @param ctx the parse tree
 */
fn enter_sql_option(&mut self, _ctx: &Sql_optionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#sql_option}.
 * @param ctx the parse tree
 */
fn exit_sql_option(&mut self, _ctx: &Sql_optionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#target_recovery_time_option}.
 * @param ctx the parse tree
 */
fn enter_target_recovery_time_option(&mut self, _ctx: &Target_recovery_time_optionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#target_recovery_time_option}.
 * @param ctx the parse tree
 */
fn exit_target_recovery_time_option(&mut self, _ctx: &Target_recovery_time_optionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#termination}.
 * @param ctx the parse tree
 */
fn enter_termination(&mut self, _ctx: &TerminationContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#termination}.
 * @param ctx the parse tree
 */
fn exit_termination(&mut self, _ctx: &TerminationContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_index}.
 * @param ctx the parse tree
 */
fn enter_drop_index(&mut self, _ctx: &Drop_indexContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_index}.
 * @param ctx the parse tree
 */
fn exit_drop_index(&mut self, _ctx: &Drop_indexContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_relational_or_xml_or_spatial_index}.
 * @param ctx the parse tree
 */
fn enter_drop_relational_or_xml_or_spatial_index(&mut self, _ctx: &Drop_relational_or_xml_or_spatial_indexContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_relational_or_xml_or_spatial_index}.
 * @param ctx the parse tree
 */
fn exit_drop_relational_or_xml_or_spatial_index(&mut self, _ctx: &Drop_relational_or_xml_or_spatial_indexContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_backward_compatible_index}.
 * @param ctx the parse tree
 */
fn enter_drop_backward_compatible_index(&mut self, _ctx: &Drop_backward_compatible_indexContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_backward_compatible_index}.
 * @param ctx the parse tree
 */
fn exit_drop_backward_compatible_index(&mut self, _ctx: &Drop_backward_compatible_indexContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_procedure}.
 * @param ctx the parse tree
 */
fn enter_drop_procedure(&mut self, _ctx: &Drop_procedureContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_procedure}.
 * @param ctx the parse tree
 */
fn exit_drop_procedure(&mut self, _ctx: &Drop_procedureContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_trigger}.
 * @param ctx the parse tree
 */
fn enter_drop_trigger(&mut self, _ctx: &Drop_triggerContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_trigger}.
 * @param ctx the parse tree
 */
fn exit_drop_trigger(&mut self, _ctx: &Drop_triggerContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_dml_trigger}.
 * @param ctx the parse tree
 */
fn enter_drop_dml_trigger(&mut self, _ctx: &Drop_dml_triggerContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_dml_trigger}.
 * @param ctx the parse tree
 */
fn exit_drop_dml_trigger(&mut self, _ctx: &Drop_dml_triggerContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_ddl_trigger}.
 * @param ctx the parse tree
 */
fn enter_drop_ddl_trigger(&mut self, _ctx: &Drop_ddl_triggerContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_ddl_trigger}.
 * @param ctx the parse tree
 */
fn exit_drop_ddl_trigger(&mut self, _ctx: &Drop_ddl_triggerContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_function}.
 * @param ctx the parse tree
 */
fn enter_drop_function(&mut self, _ctx: &Drop_functionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_function}.
 * @param ctx the parse tree
 */
fn exit_drop_function(&mut self, _ctx: &Drop_functionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_statistics}.
 * @param ctx the parse tree
 */
fn enter_drop_statistics(&mut self, _ctx: &Drop_statisticsContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_statistics}.
 * @param ctx the parse tree
 */
fn exit_drop_statistics(&mut self, _ctx: &Drop_statisticsContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_table}.
 * @param ctx the parse tree
 */
fn enter_drop_table(&mut self, _ctx: &Drop_tableContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_table}.
 * @param ctx the parse tree
 */
fn exit_drop_table(&mut self, _ctx: &Drop_tableContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_view}.
 * @param ctx the parse tree
 */
fn enter_drop_view(&mut self, _ctx: &Drop_viewContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_view}.
 * @param ctx the parse tree
 */
fn exit_drop_view(&mut self, _ctx: &Drop_viewContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_type}.
 * @param ctx the parse tree
 */
fn enter_create_type(&mut self, _ctx: &Create_typeContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_type}.
 * @param ctx the parse tree
 */
fn exit_create_type(&mut self, _ctx: &Create_typeContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#drop_type}.
 * @param ctx the parse tree
 */
fn enter_drop_type(&mut self, _ctx: &Drop_typeContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#drop_type}.
 * @param ctx the parse tree
 */
fn exit_drop_type(&mut self, _ctx: &Drop_typeContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#rowset_function_limited}.
 * @param ctx the parse tree
 */
fn enter_rowset_function_limited(&mut self, _ctx: &Rowset_function_limitedContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#rowset_function_limited}.
 * @param ctx the parse tree
 */
fn exit_rowset_function_limited(&mut self, _ctx: &Rowset_function_limitedContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#openquery}.
 * @param ctx the parse tree
 */
fn enter_openquery(&mut self, _ctx: &OpenqueryContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#openquery}.
 * @param ctx the parse tree
 */
fn exit_openquery(&mut self, _ctx: &OpenqueryContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#opendatasource}.
 * @param ctx the parse tree
 */
fn enter_opendatasource(&mut self, _ctx: &OpendatasourceContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#opendatasource}.
 * @param ctx the parse tree
 */
fn exit_opendatasource(&mut self, _ctx: &OpendatasourceContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#declare_statement}.
 * @param ctx the parse tree
 */
fn enter_declare_statement(&mut self, _ctx: &Declare_statementContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#declare_statement}.
 * @param ctx the parse tree
 */
fn exit_declare_statement(&mut self, _ctx: &Declare_statementContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#cursor_statement}.
 * @param ctx the parse tree
 */
fn enter_cursor_statement(&mut self, _ctx: &Cursor_statementContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#cursor_statement}.
 * @param ctx the parse tree
 */
fn exit_cursor_statement(&mut self, _ctx: &Cursor_statementContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#backup_database}.
 * @param ctx the parse tree
 */
fn enter_backup_database(&mut self, _ctx: &Backup_databaseContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#backup_database}.
 * @param ctx the parse tree
 */
fn exit_backup_database(&mut self, _ctx: &Backup_databaseContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#backup_log}.
 * @param ctx the parse tree
 */
fn enter_backup_log(&mut self, _ctx: &Backup_logContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#backup_log}.
 * @param ctx the parse tree
 */
fn exit_backup_log(&mut self, _ctx: &Backup_logContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#backup_certificate}.
 * @param ctx the parse tree
 */
fn enter_backup_certificate(&mut self, _ctx: &Backup_certificateContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#backup_certificate}.
 * @param ctx the parse tree
 */
fn exit_backup_certificate(&mut self, _ctx: &Backup_certificateContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#backup_master_key}.
 * @param ctx the parse tree
 */
fn enter_backup_master_key(&mut self, _ctx: &Backup_master_keyContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#backup_master_key}.
 * @param ctx the parse tree
 */
fn exit_backup_master_key(&mut self, _ctx: &Backup_master_keyContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#backup_service_master_key}.
 * @param ctx the parse tree
 */
fn enter_backup_service_master_key(&mut self, _ctx: &Backup_service_master_keyContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#backup_service_master_key}.
 * @param ctx the parse tree
 */
fn exit_backup_service_master_key(&mut self, _ctx: &Backup_service_master_keyContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#kill_statement}.
 * @param ctx the parse tree
 */
fn enter_kill_statement(&mut self, _ctx: &Kill_statementContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#kill_statement}.
 * @param ctx the parse tree
 */
fn exit_kill_statement(&mut self, _ctx: &Kill_statementContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#kill_process}.
 * @param ctx the parse tree
 */
fn enter_kill_process(&mut self, _ctx: &Kill_processContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#kill_process}.
 * @param ctx the parse tree
 */
fn exit_kill_process(&mut self, _ctx: &Kill_processContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#kill_query_notification}.
 * @param ctx the parse tree
 */
fn enter_kill_query_notification(&mut self, _ctx: &Kill_query_notificationContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#kill_query_notification}.
 * @param ctx the parse tree
 */
fn exit_kill_query_notification(&mut self, _ctx: &Kill_query_notificationContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#kill_stats_job}.
 * @param ctx the parse tree
 */
fn enter_kill_stats_job(&mut self, _ctx: &Kill_stats_jobContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#kill_stats_job}.
 * @param ctx the parse tree
 */
fn exit_kill_stats_job(&mut self, _ctx: &Kill_stats_jobContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#execute_statement}.
 * @param ctx the parse tree
 */
fn enter_execute_statement(&mut self, _ctx: &Execute_statementContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#execute_statement}.
 * @param ctx the parse tree
 */
fn exit_execute_statement(&mut self, _ctx: &Execute_statementContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#execute_body}.
 * @param ctx the parse tree
 */
fn enter_execute_body(&mut self, _ctx: &Execute_bodyContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#execute_body}.
 * @param ctx the parse tree
 */
fn exit_execute_body(&mut self, _ctx: &Execute_bodyContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#execute_statement_arg}.
 * @param ctx the parse tree
 */
fn enter_execute_statement_arg(&mut self, _ctx: &Execute_statement_argContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#execute_statement_arg}.
 * @param ctx the parse tree
 */
fn exit_execute_statement_arg(&mut self, _ctx: &Execute_statement_argContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#execute_var_string}.
 * @param ctx the parse tree
 */
fn enter_execute_var_string(&mut self, _ctx: &Execute_var_stringContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#execute_var_string}.
 * @param ctx the parse tree
 */
fn exit_execute_var_string(&mut self, _ctx: &Execute_var_stringContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#security_statement}.
 * @param ctx the parse tree
 */
fn enter_security_statement(&mut self, _ctx: &Security_statementContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#security_statement}.
 * @param ctx the parse tree
 */
fn exit_security_statement(&mut self, _ctx: &Security_statementContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_certificate}.
 * @param ctx the parse tree
 */
fn enter_create_certificate(&mut self, _ctx: &Create_certificateContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_certificate}.
 * @param ctx the parse tree
 */
fn exit_create_certificate(&mut self, _ctx: &Create_certificateContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#existing_keys}.
 * @param ctx the parse tree
 */
fn enter_existing_keys(&mut self, _ctx: &Existing_keysContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#existing_keys}.
 * @param ctx the parse tree
 */
fn exit_existing_keys(&mut self, _ctx: &Existing_keysContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#private_key_options}.
 * @param ctx the parse tree
 */
fn enter_private_key_options(&mut self, _ctx: &Private_key_optionsContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#private_key_options}.
 * @param ctx the parse tree
 */
fn exit_private_key_options(&mut self, _ctx: &Private_key_optionsContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#generate_new_keys}.
 * @param ctx the parse tree
 */
fn enter_generate_new_keys(&mut self, _ctx: &Generate_new_keysContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#generate_new_keys}.
 * @param ctx the parse tree
 */
fn exit_generate_new_keys(&mut self, _ctx: &Generate_new_keysContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#date_options}.
 * @param ctx the parse tree
 */
fn enter_date_options(&mut self, _ctx: &Date_optionsContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#date_options}.
 * @param ctx the parse tree
 */
fn exit_date_options(&mut self, _ctx: &Date_optionsContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#open_key}.
 * @param ctx the parse tree
 */
fn enter_open_key(&mut self, _ctx: &Open_keyContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#open_key}.
 * @param ctx the parse tree
 */
fn exit_open_key(&mut self, _ctx: &Open_keyContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#close_key}.
 * @param ctx the parse tree
 */
fn enter_close_key(&mut self, _ctx: &Close_keyContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#close_key}.
 * @param ctx the parse tree
 */
fn exit_close_key(&mut self, _ctx: &Close_keyContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_key}.
 * @param ctx the parse tree
 */
fn enter_create_key(&mut self, _ctx: &Create_keyContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_key}.
 * @param ctx the parse tree
 */
fn exit_create_key(&mut self, _ctx: &Create_keyContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#key_options}.
 * @param ctx the parse tree
 */
fn enter_key_options(&mut self, _ctx: &Key_optionsContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#key_options}.
 * @param ctx the parse tree
 */
fn exit_key_options(&mut self, _ctx: &Key_optionsContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#algorithm}.
 * @param ctx the parse tree
 */
fn enter_algorithm(&mut self, _ctx: &AlgorithmContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#algorithm}.
 * @param ctx the parse tree
 */
fn exit_algorithm(&mut self, _ctx: &AlgorithmContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#encryption_mechanism}.
 * @param ctx the parse tree
 */
fn enter_encryption_mechanism(&mut self, _ctx: &Encryption_mechanismContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#encryption_mechanism}.
 * @param ctx the parse tree
 */
fn exit_encryption_mechanism(&mut self, _ctx: &Encryption_mechanismContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#decryption_mechanism}.
 * @param ctx the parse tree
 */
fn enter_decryption_mechanism(&mut self, _ctx: &Decryption_mechanismContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#decryption_mechanism}.
 * @param ctx the parse tree
 */
fn exit_decryption_mechanism(&mut self, _ctx: &Decryption_mechanismContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#grant_permission}.
 * @param ctx the parse tree
 */
fn enter_grant_permission(&mut self, _ctx: &Grant_permissionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#grant_permission}.
 * @param ctx the parse tree
 */
fn exit_grant_permission(&mut self, _ctx: &Grant_permissionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#set_statement}.
 * @param ctx the parse tree
 */
fn enter_set_statement(&mut self, _ctx: &Set_statementContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#set_statement}.
 * @param ctx the parse tree
 */
fn exit_set_statement(&mut self, _ctx: &Set_statementContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#transaction_statement}.
 * @param ctx the parse tree
 */
fn enter_transaction_statement(&mut self, _ctx: &Transaction_statementContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#transaction_statement}.
 * @param ctx the parse tree
 */
fn exit_transaction_statement(&mut self, _ctx: &Transaction_statementContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#go_statement}.
 * @param ctx the parse tree
 */
fn enter_go_statement(&mut self, _ctx: &Go_statementContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#go_statement}.
 * @param ctx the parse tree
 */
fn exit_go_statement(&mut self, _ctx: &Go_statementContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#use_statement}.
 * @param ctx the parse tree
 */
fn enter_use_statement(&mut self, _ctx: &Use_statementContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#use_statement}.
 * @param ctx the parse tree
 */
fn exit_use_statement(&mut self, _ctx: &Use_statementContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#setuser_statement}.
 * @param ctx the parse tree
 */
fn enter_setuser_statement(&mut self, _ctx: &Setuser_statementContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#setuser_statement}.
 * @param ctx the parse tree
 */
fn exit_setuser_statement(&mut self, _ctx: &Setuser_statementContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#reconfigure_statement}.
 * @param ctx the parse tree
 */
fn enter_reconfigure_statement(&mut self, _ctx: &Reconfigure_statementContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#reconfigure_statement}.
 * @param ctx the parse tree
 */
fn exit_reconfigure_statement(&mut self, _ctx: &Reconfigure_statementContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#shutdown_statement}.
 * @param ctx the parse tree
 */
fn enter_shutdown_statement(&mut self, _ctx: &Shutdown_statementContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#shutdown_statement}.
 * @param ctx the parse tree
 */
fn exit_shutdown_statement(&mut self, _ctx: &Shutdown_statementContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#dbcc_clause}.
 * @param ctx the parse tree
 */
fn enter_dbcc_clause(&mut self, _ctx: &Dbcc_clauseContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#dbcc_clause}.
 * @param ctx the parse tree
 */
fn exit_dbcc_clause(&mut self, _ctx: &Dbcc_clauseContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#dbcc_options}.
 * @param ctx the parse tree
 */
fn enter_dbcc_options(&mut self, _ctx: &Dbcc_optionsContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#dbcc_options}.
 * @param ctx the parse tree
 */
fn exit_dbcc_options(&mut self, _ctx: &Dbcc_optionsContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#execute_clause}.
 * @param ctx the parse tree
 */
fn enter_execute_clause(&mut self, _ctx: &Execute_clauseContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#execute_clause}.
 * @param ctx the parse tree
 */
fn exit_execute_clause(&mut self, _ctx: &Execute_clauseContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#declare_local}.
 * @param ctx the parse tree
 */
fn enter_declare_local(&mut self, _ctx: &Declare_localContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#declare_local}.
 * @param ctx the parse tree
 */
fn exit_declare_local(&mut self, _ctx: &Declare_localContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#table_type_definition}.
 * @param ctx the parse tree
 */
fn enter_table_type_definition(&mut self, _ctx: &Table_type_definitionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#table_type_definition}.
 * @param ctx the parse tree
 */
fn exit_table_type_definition(&mut self, _ctx: &Table_type_definitionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#xml_type_definition}.
 * @param ctx the parse tree
 */
fn enter_xml_type_definition(&mut self, _ctx: &Xml_type_definitionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#xml_type_definition}.
 * @param ctx the parse tree
 */
fn exit_xml_type_definition(&mut self, _ctx: &Xml_type_definitionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#xml_schema_collection}.
 * @param ctx the parse tree
 */
fn enter_xml_schema_collection(&mut self, _ctx: &Xml_schema_collectionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#xml_schema_collection}.
 * @param ctx the parse tree
 */
fn exit_xml_schema_collection(&mut self, _ctx: &Xml_schema_collectionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#column_def_table_constraints}.
 * @param ctx the parse tree
 */
fn enter_column_def_table_constraints(&mut self, _ctx: &Column_def_table_constraintsContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#column_def_table_constraints}.
 * @param ctx the parse tree
 */
fn exit_column_def_table_constraints(&mut self, _ctx: &Column_def_table_constraintsContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#column_def_table_constraint}.
 * @param ctx the parse tree
 */
fn enter_column_def_table_constraint(&mut self, _ctx: &Column_def_table_constraintContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#column_def_table_constraint}.
 * @param ctx the parse tree
 */
fn exit_column_def_table_constraint(&mut self, _ctx: &Column_def_table_constraintContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#column_definition}.
 * @param ctx the parse tree
 */
fn enter_column_definition(&mut self, _ctx: &Column_definitionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#column_definition}.
 * @param ctx the parse tree
 */
fn exit_column_definition(&mut self, _ctx: &Column_definitionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#materialized_column_definition}.
 * @param ctx the parse tree
 */
fn enter_materialized_column_definition(&mut self, _ctx: &Materialized_column_definitionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#materialized_column_definition}.
 * @param ctx the parse tree
 */
fn exit_materialized_column_definition(&mut self, _ctx: &Materialized_column_definitionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#column_constraint}.
 * @param ctx the parse tree
 */
fn enter_column_constraint(&mut self, _ctx: &Column_constraintContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#column_constraint}.
 * @param ctx the parse tree
 */
fn exit_column_constraint(&mut self, _ctx: &Column_constraintContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#table_constraint}.
 * @param ctx the parse tree
 */
fn enter_table_constraint(&mut self, _ctx: &Table_constraintContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#table_constraint}.
 * @param ctx the parse tree
 */
fn exit_table_constraint(&mut self, _ctx: &Table_constraintContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#on_delete}.
 * @param ctx the parse tree
 */
fn enter_on_delete(&mut self, _ctx: &On_deleteContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#on_delete}.
 * @param ctx the parse tree
 */
fn exit_on_delete(&mut self, _ctx: &On_deleteContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#on_update}.
 * @param ctx the parse tree
 */
fn enter_on_update(&mut self, _ctx: &On_updateContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#on_update}.
 * @param ctx the parse tree
 */
fn exit_on_update(&mut self, _ctx: &On_updateContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#index_options}.
 * @param ctx the parse tree
 */
fn enter_index_options(&mut self, _ctx: &Index_optionsContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#index_options}.
 * @param ctx the parse tree
 */
fn exit_index_options(&mut self, _ctx: &Index_optionsContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#index_option}.
 * @param ctx the parse tree
 */
fn enter_index_option(&mut self, _ctx: &Index_optionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#index_option}.
 * @param ctx the parse tree
 */
fn exit_index_option(&mut self, _ctx: &Index_optionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#declare_cursor}.
 * @param ctx the parse tree
 */
fn enter_declare_cursor(&mut self, _ctx: &Declare_cursorContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#declare_cursor}.
 * @param ctx the parse tree
 */
fn exit_declare_cursor(&mut self, _ctx: &Declare_cursorContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#declare_set_cursor_common}.
 * @param ctx the parse tree
 */
fn enter_declare_set_cursor_common(&mut self, _ctx: &Declare_set_cursor_commonContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#declare_set_cursor_common}.
 * @param ctx the parse tree
 */
fn exit_declare_set_cursor_common(&mut self, _ctx: &Declare_set_cursor_commonContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#declare_set_cursor_common_partial}.
 * @param ctx the parse tree
 */
fn enter_declare_set_cursor_common_partial(&mut self, _ctx: &Declare_set_cursor_common_partialContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#declare_set_cursor_common_partial}.
 * @param ctx the parse tree
 */
fn exit_declare_set_cursor_common_partial(&mut self, _ctx: &Declare_set_cursor_common_partialContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#fetch_cursor}.
 * @param ctx the parse tree
 */
fn enter_fetch_cursor(&mut self, _ctx: &Fetch_cursorContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#fetch_cursor}.
 * @param ctx the parse tree
 */
fn exit_fetch_cursor(&mut self, _ctx: &Fetch_cursorContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#set_special}.
 * @param ctx the parse tree
 */
fn enter_set_special(&mut self, _ctx: &Set_specialContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#set_special}.
 * @param ctx the parse tree
 */
fn exit_set_special(&mut self, _ctx: &Set_specialContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#constant_LOCAL_ID}.
 * @param ctx the parse tree
 */
fn enter_constant_LOCAL_ID(&mut self, _ctx: &Constant_LOCAL_IDContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#constant_LOCAL_ID}.
 * @param ctx the parse tree
 */
fn exit_constant_LOCAL_ID(&mut self, _ctx: &Constant_LOCAL_IDContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#expression}.
 * @param ctx the parse tree
 */
fn enter_expression(&mut self, _ctx: &ExpressionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#expression}.
 * @param ctx the parse tree
 */
fn exit_expression(&mut self, _ctx: &ExpressionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#primitive_expression}.
 * @param ctx the parse tree
 */
fn enter_primitive_expression(&mut self, _ctx: &Primitive_expressionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#primitive_expression}.
 * @param ctx the parse tree
 */
fn exit_primitive_expression(&mut self, _ctx: &Primitive_expressionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#case_expression}.
 * @param ctx the parse tree
 */
fn enter_case_expression(&mut self, _ctx: &Case_expressionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#case_expression}.
 * @param ctx the parse tree
 */
fn exit_case_expression(&mut self, _ctx: &Case_expressionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#unary_operator_expression}.
 * @param ctx the parse tree
 */
fn enter_unary_operator_expression(&mut self, _ctx: &Unary_operator_expressionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#unary_operator_expression}.
 * @param ctx the parse tree
 */
fn exit_unary_operator_expression(&mut self, _ctx: &Unary_operator_expressionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#bracket_expression}.
 * @param ctx the parse tree
 */
fn enter_bracket_expression(&mut self, _ctx: &Bracket_expressionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#bracket_expression}.
 * @param ctx the parse tree
 */
fn exit_bracket_expression(&mut self, _ctx: &Bracket_expressionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#constant_expression}.
 * @param ctx the parse tree
 */
fn enter_constant_expression(&mut self, _ctx: &Constant_expressionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#constant_expression}.
 * @param ctx the parse tree
 */
fn exit_constant_expression(&mut self, _ctx: &Constant_expressionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#subquery}.
 * @param ctx the parse tree
 */
fn enter_subquery(&mut self, _ctx: &SubqueryContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#subquery}.
 * @param ctx the parse tree
 */
fn exit_subquery(&mut self, _ctx: &SubqueryContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#with_expression}.
 * @param ctx the parse tree
 */
fn enter_with_expression(&mut self, _ctx: &With_expressionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#with_expression}.
 * @param ctx the parse tree
 */
fn exit_with_expression(&mut self, _ctx: &With_expressionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#common_table_expression}.
 * @param ctx the parse tree
 */
fn enter_common_table_expression(&mut self, _ctx: &Common_table_expressionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#common_table_expression}.
 * @param ctx the parse tree
 */
fn exit_common_table_expression(&mut self, _ctx: &Common_table_expressionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#update_elem}.
 * @param ctx the parse tree
 */
fn enter_update_elem(&mut self, _ctx: &Update_elemContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#update_elem}.
 * @param ctx the parse tree
 */
fn exit_update_elem(&mut self, _ctx: &Update_elemContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#search_condition_list}.
 * @param ctx the parse tree
 */
fn enter_search_condition_list(&mut self, _ctx: &Search_condition_listContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#search_condition_list}.
 * @param ctx the parse tree
 */
fn exit_search_condition_list(&mut self, _ctx: &Search_condition_listContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#search_condition}.
 * @param ctx the parse tree
 */
fn enter_search_condition(&mut self, _ctx: &Search_conditionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#search_condition}.
 * @param ctx the parse tree
 */
fn exit_search_condition(&mut self, _ctx: &Search_conditionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#search_condition_and}.
 * @param ctx the parse tree
 */
fn enter_search_condition_and(&mut self, _ctx: &Search_condition_andContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#search_condition_and}.
 * @param ctx the parse tree
 */
fn exit_search_condition_and(&mut self, _ctx: &Search_condition_andContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#search_condition_not}.
 * @param ctx the parse tree
 */
fn enter_search_condition_not(&mut self, _ctx: &Search_condition_notContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#search_condition_not}.
 * @param ctx the parse tree
 */
fn exit_search_condition_not(&mut self, _ctx: &Search_condition_notContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#predicate}.
 * @param ctx the parse tree
 */
fn enter_predicate(&mut self, _ctx: &PredicateContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#predicate}.
 * @param ctx the parse tree
 */
fn exit_predicate(&mut self, _ctx: &PredicateContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#query_expression}.
 * @param ctx the parse tree
 */
fn enter_query_expression(&mut self, _ctx: &Query_expressionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#query_expression}.
 * @param ctx the parse tree
 */
fn exit_query_expression(&mut self, _ctx: &Query_expressionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#sql_union}.
 * @param ctx the parse tree
 */
fn enter_sql_union(&mut self, _ctx: &Sql_unionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#sql_union}.
 * @param ctx the parse tree
 */
fn exit_sql_union(&mut self, _ctx: &Sql_unionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#query_specification}.
 * @param ctx the parse tree
 */
fn enter_query_specification(&mut self, _ctx: &Query_specificationContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#query_specification}.
 * @param ctx the parse tree
 */
fn exit_query_specification(&mut self, _ctx: &Query_specificationContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#top_clause}.
 * @param ctx the parse tree
 */
fn enter_top_clause(&mut self, _ctx: &Top_clauseContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#top_clause}.
 * @param ctx the parse tree
 */
fn exit_top_clause(&mut self, _ctx: &Top_clauseContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#top_percent}.
 * @param ctx the parse tree
 */
fn enter_top_percent(&mut self, _ctx: &Top_percentContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#top_percent}.
 * @param ctx the parse tree
 */
fn exit_top_percent(&mut self, _ctx: &Top_percentContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#top_count}.
 * @param ctx the parse tree
 */
fn enter_top_count(&mut self, _ctx: &Top_countContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#top_count}.
 * @param ctx the parse tree
 */
fn exit_top_count(&mut self, _ctx: &Top_countContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#order_by_clause}.
 * @param ctx the parse tree
 */
fn enter_order_by_clause(&mut self, _ctx: &Order_by_clauseContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#order_by_clause}.
 * @param ctx the parse tree
 */
fn exit_order_by_clause(&mut self, _ctx: &Order_by_clauseContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#for_clause}.
 * @param ctx the parse tree
 */
fn enter_for_clause(&mut self, _ctx: &For_clauseContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#for_clause}.
 * @param ctx the parse tree
 */
fn exit_for_clause(&mut self, _ctx: &For_clauseContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#xml_common_directives}.
 * @param ctx the parse tree
 */
fn enter_xml_common_directives(&mut self, _ctx: &Xml_common_directivesContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#xml_common_directives}.
 * @param ctx the parse tree
 */
fn exit_xml_common_directives(&mut self, _ctx: &Xml_common_directivesContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#order_by_expression}.
 * @param ctx the parse tree
 */
fn enter_order_by_expression(&mut self, _ctx: &Order_by_expressionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#order_by_expression}.
 * @param ctx the parse tree
 */
fn exit_order_by_expression(&mut self, _ctx: &Order_by_expressionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#group_by_item}.
 * @param ctx the parse tree
 */
fn enter_group_by_item(&mut self, _ctx: &Group_by_itemContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#group_by_item}.
 * @param ctx the parse tree
 */
fn exit_group_by_item(&mut self, _ctx: &Group_by_itemContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#option_clause}.
 * @param ctx the parse tree
 */
fn enter_option_clause(&mut self, _ctx: &Option_clauseContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#option_clause}.
 * @param ctx the parse tree
 */
fn exit_option_clause(&mut self, _ctx: &Option_clauseContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#option}.
 * @param ctx the parse tree
 */
fn enter_option(&mut self, _ctx: &OptionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#option}.
 * @param ctx the parse tree
 */
fn exit_option(&mut self, _ctx: &OptionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#optimize_for_arg}.
 * @param ctx the parse tree
 */
fn enter_optimize_for_arg(&mut self, _ctx: &Optimize_for_argContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#optimize_for_arg}.
 * @param ctx the parse tree
 */
fn exit_optimize_for_arg(&mut self, _ctx: &Optimize_for_argContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#select_list}.
 * @param ctx the parse tree
 */
fn enter_select_list(&mut self, _ctx: &Select_listContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#select_list}.
 * @param ctx the parse tree
 */
fn exit_select_list(&mut self, _ctx: &Select_listContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#udt_method_arguments}.
 * @param ctx the parse tree
 */
fn enter_udt_method_arguments(&mut self, _ctx: &Udt_method_argumentsContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#udt_method_arguments}.
 * @param ctx the parse tree
 */
fn exit_udt_method_arguments(&mut self, _ctx: &Udt_method_argumentsContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#asterisk}.
 * @param ctx the parse tree
 */
fn enter_asterisk(&mut self, _ctx: &AsteriskContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#asterisk}.
 * @param ctx the parse tree
 */
fn exit_asterisk(&mut self, _ctx: &AsteriskContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#column_elem}.
 * @param ctx the parse tree
 */
fn enter_column_elem(&mut self, _ctx: &Column_elemContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#column_elem}.
 * @param ctx the parse tree
 */
fn exit_column_elem(&mut self, _ctx: &Column_elemContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#udt_elem}.
 * @param ctx the parse tree
 */
fn enter_udt_elem(&mut self, _ctx: &Udt_elemContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#udt_elem}.
 * @param ctx the parse tree
 */
fn exit_udt_elem(&mut self, _ctx: &Udt_elemContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#expression_elem}.
 * @param ctx the parse tree
 */
fn enter_expression_elem(&mut self, _ctx: &Expression_elemContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#expression_elem}.
 * @param ctx the parse tree
 */
fn exit_expression_elem(&mut self, _ctx: &Expression_elemContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#select_list_elem}.
 * @param ctx the parse tree
 */
fn enter_select_list_elem(&mut self, _ctx: &Select_list_elemContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#select_list_elem}.
 * @param ctx the parse tree
 */
fn exit_select_list_elem(&mut self, _ctx: &Select_list_elemContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#table_sources}.
 * @param ctx the parse tree
 */
fn enter_table_sources(&mut self, _ctx: &Table_sourcesContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#table_sources}.
 * @param ctx the parse tree
 */
fn exit_table_sources(&mut self, _ctx: &Table_sourcesContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#table_source}.
 * @param ctx the parse tree
 */
fn enter_table_source(&mut self, _ctx: &Table_sourceContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#table_source}.
 * @param ctx the parse tree
 */
fn exit_table_source(&mut self, _ctx: &Table_sourceContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#table_source_item_joined}.
 * @param ctx the parse tree
 */
fn enter_table_source_item_joined(&mut self, _ctx: &Table_source_item_joinedContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#table_source_item_joined}.
 * @param ctx the parse tree
 */
fn exit_table_source_item_joined(&mut self, _ctx: &Table_source_item_joinedContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#table_source_item}.
 * @param ctx the parse tree
 */
fn enter_table_source_item(&mut self, _ctx: &Table_source_itemContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#table_source_item}.
 * @param ctx the parse tree
 */
fn exit_table_source_item(&mut self, _ctx: &Table_source_itemContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#open_xml}.
 * @param ctx the parse tree
 */
fn enter_open_xml(&mut self, _ctx: &Open_xmlContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#open_xml}.
 * @param ctx the parse tree
 */
fn exit_open_xml(&mut self, _ctx: &Open_xmlContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#schema_declaration}.
 * @param ctx the parse tree
 */
fn enter_schema_declaration(&mut self, _ctx: &Schema_declarationContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#schema_declaration}.
 * @param ctx the parse tree
 */
fn exit_schema_declaration(&mut self, _ctx: &Schema_declarationContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#column_declaration}.
 * @param ctx the parse tree
 */
fn enter_column_declaration(&mut self, _ctx: &Column_declarationContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#column_declaration}.
 * @param ctx the parse tree
 */
fn exit_column_declaration(&mut self, _ctx: &Column_declarationContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#change_table}.
 * @param ctx the parse tree
 */
fn enter_change_table(&mut self, _ctx: &Change_tableContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#change_table}.
 * @param ctx the parse tree
 */
fn exit_change_table(&mut self, _ctx: &Change_tableContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#join_part}.
 * @param ctx the parse tree
 */
fn enter_join_part(&mut self, _ctx: &Join_partContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#join_part}.
 * @param ctx the parse tree
 */
fn exit_join_part(&mut self, _ctx: &Join_partContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#pivot_clause}.
 * @param ctx the parse tree
 */
fn enter_pivot_clause(&mut self, _ctx: &Pivot_clauseContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#pivot_clause}.
 * @param ctx the parse tree
 */
fn exit_pivot_clause(&mut self, _ctx: &Pivot_clauseContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#unpivot_clause}.
 * @param ctx the parse tree
 */
fn enter_unpivot_clause(&mut self, _ctx: &Unpivot_clauseContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#unpivot_clause}.
 * @param ctx the parse tree
 */
fn exit_unpivot_clause(&mut self, _ctx: &Unpivot_clauseContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#full_column_name_list}.
 * @param ctx the parse tree
 */
fn enter_full_column_name_list(&mut self, _ctx: &Full_column_name_listContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#full_column_name_list}.
 * @param ctx the parse tree
 */
fn exit_full_column_name_list(&mut self, _ctx: &Full_column_name_listContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#table_name_with_hint}.
 * @param ctx the parse tree
 */
fn enter_table_name_with_hint(&mut self, _ctx: &Table_name_with_hintContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#table_name_with_hint}.
 * @param ctx the parse tree
 */
fn exit_table_name_with_hint(&mut self, _ctx: &Table_name_with_hintContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#rowset_function}.
 * @param ctx the parse tree
 */
fn enter_rowset_function(&mut self, _ctx: &Rowset_functionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#rowset_function}.
 * @param ctx the parse tree
 */
fn exit_rowset_function(&mut self, _ctx: &Rowset_functionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#bulk_option}.
 * @param ctx the parse tree
 */
fn enter_bulk_option(&mut self, _ctx: &Bulk_optionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#bulk_option}.
 * @param ctx the parse tree
 */
fn exit_bulk_option(&mut self, _ctx: &Bulk_optionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#derived_table}.
 * @param ctx the parse tree
 */
fn enter_derived_table(&mut self, _ctx: &Derived_tableContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#derived_table}.
 * @param ctx the parse tree
 */
fn exit_derived_table(&mut self, _ctx: &Derived_tableContext) { }

/**
 * Enter a parse tree produced by the {@code BINARY_CHECKSUM}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn enter_BINARY_CHECKSUM(&mut self, _ctx: &BINARY_CHECKSUMContext) { }
/**
 * Exit a parse tree produced by the {@code BINARY_CHECKSUM}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn exit_BINARY_CHECKSUM(&mut self, _ctx: &BINARY_CHECKSUMContext) { }

/**
 * Enter a parse tree produced by the {@code CAST}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn enter_CAST(&mut self, _ctx: &CASTContext) { }
/**
 * Exit a parse tree produced by the {@code CAST}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn exit_CAST(&mut self, _ctx: &CASTContext) { }

/**
 * Enter a parse tree produced by the {@code CONVERT}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn enter_CONVERT(&mut self, _ctx: &CONVERTContext) { }
/**
 * Exit a parse tree produced by the {@code CONVERT}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn exit_CONVERT(&mut self, _ctx: &CONVERTContext) { }

/**
 * Enter a parse tree produced by the {@code CHECKSUM}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn enter_CHECKSUM(&mut self, _ctx: &CHECKSUMContext) { }
/**
 * Exit a parse tree produced by the {@code CHECKSUM}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn exit_CHECKSUM(&mut self, _ctx: &CHECKSUMContext) { }

/**
 * Enter a parse tree produced by the {@code COALESCE}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn enter_COALESCE(&mut self, _ctx: &COALESCEContext) { }
/**
 * Exit a parse tree produced by the {@code COALESCE}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn exit_COALESCE(&mut self, _ctx: &COALESCEContext) { }

/**
 * Enter a parse tree produced by the {@code CURRENT_TIMESTAMP}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn enter_CURRENT_TIMESTAMP(&mut self, _ctx: &CURRENT_TIMESTAMPContext) { }
/**
 * Exit a parse tree produced by the {@code CURRENT_TIMESTAMP}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn exit_CURRENT_TIMESTAMP(&mut self, _ctx: &CURRENT_TIMESTAMPContext) { }

/**
 * Enter a parse tree produced by the {@code CURRENT_USER}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn enter_CURRENT_USER(&mut self, _ctx: &CURRENT_USERContext) { }
/**
 * Exit a parse tree produced by the {@code CURRENT_USER}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn exit_CURRENT_USER(&mut self, _ctx: &CURRENT_USERContext) { }

/**
 * Enter a parse tree produced by the {@code DATEADD}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn enter_DATEADD(&mut self, _ctx: &DATEADDContext) { }
/**
 * Exit a parse tree produced by the {@code DATEADD}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn exit_DATEADD(&mut self, _ctx: &DATEADDContext) { }

/**
 * Enter a parse tree produced by the {@code DATEDIFF}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn enter_DATEDIFF(&mut self, _ctx: &DATEDIFFContext) { }
/**
 * Exit a parse tree produced by the {@code DATEDIFF}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn exit_DATEDIFF(&mut self, _ctx: &DATEDIFFContext) { }

/**
 * Enter a parse tree produced by the {@code DATENAME}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn enter_DATENAME(&mut self, _ctx: &DATENAMEContext) { }
/**
 * Exit a parse tree produced by the {@code DATENAME}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn exit_DATENAME(&mut self, _ctx: &DATENAMEContext) { }

/**
 * Enter a parse tree produced by the {@code DATEPART}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn enter_DATEPART(&mut self, _ctx: &DATEPARTContext) { }
/**
 * Exit a parse tree produced by the {@code DATEPART}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn exit_DATEPART(&mut self, _ctx: &DATEPARTContext) { }

/**
 * Enter a parse tree produced by the {@code GETDATE}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn enter_GETDATE(&mut self, _ctx: &GETDATEContext) { }
/**
 * Exit a parse tree produced by the {@code GETDATE}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn exit_GETDATE(&mut self, _ctx: &GETDATEContext) { }

/**
 * Enter a parse tree produced by the {@code GETUTCDATE}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn enter_GETUTCDATE(&mut self, _ctx: &GETUTCDATEContext) { }
/**
 * Exit a parse tree produced by the {@code GETUTCDATE}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn exit_GETUTCDATE(&mut self, _ctx: &GETUTCDATEContext) { }

/**
 * Enter a parse tree produced by the {@code IDENTITY}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn enter_IDENTITY(&mut self, _ctx: &IDENTITYContext) { }
/**
 * Exit a parse tree produced by the {@code IDENTITY}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn exit_IDENTITY(&mut self, _ctx: &IDENTITYContext) { }

/**
 * Enter a parse tree produced by the {@code MIN_ACTIVE_ROWVERSION}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn enter_MIN_ACTIVE_ROWVERSION(&mut self, _ctx: &MIN_ACTIVE_ROWVERSIONContext) { }
/**
 * Exit a parse tree produced by the {@code MIN_ACTIVE_ROWVERSION}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn exit_MIN_ACTIVE_ROWVERSION(&mut self, _ctx: &MIN_ACTIVE_ROWVERSIONContext) { }

/**
 * Enter a parse tree produced by the {@code NULLIF}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn enter_NULLIF(&mut self, _ctx: &NULLIFContext) { }
/**
 * Exit a parse tree produced by the {@code NULLIF}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn exit_NULLIF(&mut self, _ctx: &NULLIFContext) { }

/**
 * Enter a parse tree produced by the {@code STUFF}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn enter_STUFF(&mut self, _ctx: &STUFFContext) { }
/**
 * Exit a parse tree produced by the {@code STUFF}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn exit_STUFF(&mut self, _ctx: &STUFFContext) { }

/**
 * Enter a parse tree produced by the {@code SESSION_USER}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn enter_SESSION_USER(&mut self, _ctx: &SESSION_USERContext) { }
/**
 * Exit a parse tree produced by the {@code SESSION_USER}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn exit_SESSION_USER(&mut self, _ctx: &SESSION_USERContext) { }

/**
 * Enter a parse tree produced by the {@code SYSTEM_USER}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn enter_SYSTEM_USER(&mut self, _ctx: &SYSTEM_USERContext) { }
/**
 * Exit a parse tree produced by the {@code SYSTEM_USER}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn exit_SYSTEM_USER(&mut self, _ctx: &SYSTEM_USERContext) { }

/**
 * Enter a parse tree produced by the {@code ISNULL}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn enter_ISNULL(&mut self, _ctx: &ISNULLContext) { }
/**
 * Exit a parse tree produced by the {@code ISNULL}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn exit_ISNULL(&mut self, _ctx: &ISNULLContext) { }

/**
 * Enter a parse tree produced by the {@code XML_DATA_TYPE_FUNC}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn enter_XML_DATA_TYPE_FUNC(&mut self, _ctx: &XML_DATA_TYPE_FUNCContext) { }
/**
 * Exit a parse tree produced by the {@code XML_DATA_TYPE_FUNC}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn exit_XML_DATA_TYPE_FUNC(&mut self, _ctx: &XML_DATA_TYPE_FUNCContext) { }

/**
 * Enter a parse tree produced by the {@code IFF}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn enter_IFF(&mut self, _ctx: &IFFContext) { }
/**
 * Exit a parse tree produced by the {@code IFF}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn exit_IFF(&mut self, _ctx: &IFFContext) { }

/**
 * Enter a parse tree produced by the {@code RANKING_WINDOWED_FUNC}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn enter_RANKING_WINDOWED_FUNC(&mut self, _ctx: &RANKING_WINDOWED_FUNCContext) { }
/**
 * Exit a parse tree produced by the {@code RANKING_WINDOWED_FUNC}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn exit_RANKING_WINDOWED_FUNC(&mut self, _ctx: &RANKING_WINDOWED_FUNCContext) { }

/**
 * Enter a parse tree produced by the {@code AGGREGATE_WINDOWED_FUNC}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn enter_AGGREGATE_WINDOWED_FUNC(&mut self, _ctx: &AGGREGATE_WINDOWED_FUNCContext) { }
/**
 * Exit a parse tree produced by the {@code AGGREGATE_WINDOWED_FUNC}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn exit_AGGREGATE_WINDOWED_FUNC(&mut self, _ctx: &AGGREGATE_WINDOWED_FUNCContext) { }

/**
 * Enter a parse tree produced by the {@code ANALYTIC_WINDOWED_FUNC}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn enter_ANALYTIC_WINDOWED_FUNC(&mut self, _ctx: &ANALYTIC_WINDOWED_FUNCContext) { }
/**
 * Exit a parse tree produced by the {@code ANALYTIC_WINDOWED_FUNC}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn exit_ANALYTIC_WINDOWED_FUNC(&mut self, _ctx: &ANALYTIC_WINDOWED_FUNCContext) { }

/**
 * Enter a parse tree produced by the {@code SCALAR_FUNCTION}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn enter_SCALAR_FUNCTION(&mut self, _ctx: &SCALAR_FUNCTIONContext) { }
/**
 * Exit a parse tree produced by the {@code SCALAR_FUNCTION}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn exit_SCALAR_FUNCTION(&mut self, _ctx: &SCALAR_FUNCTIONContext) { }

/**
 * Enter a parse tree produced by the {@code STRINGAGG}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn enter_STRINGAGG(&mut self, _ctx: &STRINGAGGContext) { }
/**
 * Exit a parse tree produced by the {@code STRINGAGG}
 * labeled alternative in {@link TSqlParser#function_call}.
 * @param ctx the parse tree
 */
fn exit_STRINGAGG(&mut self, _ctx: &STRINGAGGContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#xml_data_type_methods}.
 * @param ctx the parse tree
 */
fn enter_xml_data_type_methods(&mut self, _ctx: &Xml_data_type_methodsContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#xml_data_type_methods}.
 * @param ctx the parse tree
 */
fn exit_xml_data_type_methods(&mut self, _ctx: &Xml_data_type_methodsContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#value_method}.
 * @param ctx the parse tree
 */
fn enter_value_method(&mut self, _ctx: &Value_methodContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#value_method}.
 * @param ctx the parse tree
 */
fn exit_value_method(&mut self, _ctx: &Value_methodContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#query_method}.
 * @param ctx the parse tree
 */
fn enter_query_method(&mut self, _ctx: &Query_methodContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#query_method}.
 * @param ctx the parse tree
 */
fn exit_query_method(&mut self, _ctx: &Query_methodContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#exist_method}.
 * @param ctx the parse tree
 */
fn enter_exist_method(&mut self, _ctx: &Exist_methodContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#exist_method}.
 * @param ctx the parse tree
 */
fn exit_exist_method(&mut self, _ctx: &Exist_methodContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#modify_method}.
 * @param ctx the parse tree
 */
fn enter_modify_method(&mut self, _ctx: &Modify_methodContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#modify_method}.
 * @param ctx the parse tree
 */
fn exit_modify_method(&mut self, _ctx: &Modify_methodContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#nodes_method}.
 * @param ctx the parse tree
 */
fn enter_nodes_method(&mut self, _ctx: &Nodes_methodContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#nodes_method}.
 * @param ctx the parse tree
 */
fn exit_nodes_method(&mut self, _ctx: &Nodes_methodContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#switch_section}.
 * @param ctx the parse tree
 */
fn enter_switch_section(&mut self, _ctx: &Switch_sectionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#switch_section}.
 * @param ctx the parse tree
 */
fn exit_switch_section(&mut self, _ctx: &Switch_sectionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#switch_search_condition_section}.
 * @param ctx the parse tree
 */
fn enter_switch_search_condition_section(&mut self, _ctx: &Switch_search_condition_sectionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#switch_search_condition_section}.
 * @param ctx the parse tree
 */
fn exit_switch_search_condition_section(&mut self, _ctx: &Switch_search_condition_sectionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#as_column_alias}.
 * @param ctx the parse tree
 */
fn enter_as_column_alias(&mut self, _ctx: &As_column_aliasContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#as_column_alias}.
 * @param ctx the parse tree
 */
fn exit_as_column_alias(&mut self, _ctx: &As_column_aliasContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#as_table_alias}.
 * @param ctx the parse tree
 */
fn enter_as_table_alias(&mut self, _ctx: &As_table_aliasContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#as_table_alias}.
 * @param ctx the parse tree
 */
fn exit_as_table_alias(&mut self, _ctx: &As_table_aliasContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#table_alias}.
 * @param ctx the parse tree
 */
fn enter_table_alias(&mut self, _ctx: &Table_aliasContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#table_alias}.
 * @param ctx the parse tree
 */
fn exit_table_alias(&mut self, _ctx: &Table_aliasContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#with_table_hints}.
 * @param ctx the parse tree
 */
fn enter_with_table_hints(&mut self, _ctx: &With_table_hintsContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#with_table_hints}.
 * @param ctx the parse tree
 */
fn exit_with_table_hints(&mut self, _ctx: &With_table_hintsContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#insert_with_table_hints}.
 * @param ctx the parse tree
 */
fn enter_insert_with_table_hints(&mut self, _ctx: &Insert_with_table_hintsContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#insert_with_table_hints}.
 * @param ctx the parse tree
 */
fn exit_insert_with_table_hints(&mut self, _ctx: &Insert_with_table_hintsContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#table_hint}.
 * @param ctx the parse tree
 */
fn enter_table_hint(&mut self, _ctx: &Table_hintContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#table_hint}.
 * @param ctx the parse tree
 */
fn exit_table_hint(&mut self, _ctx: &Table_hintContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#index_value}.
 * @param ctx the parse tree
 */
fn enter_index_value(&mut self, _ctx: &Index_valueContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#index_value}.
 * @param ctx the parse tree
 */
fn exit_index_value(&mut self, _ctx: &Index_valueContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#column_alias_list}.
 * @param ctx the parse tree
 */
fn enter_column_alias_list(&mut self, _ctx: &Column_alias_listContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#column_alias_list}.
 * @param ctx the parse tree
 */
fn exit_column_alias_list(&mut self, _ctx: &Column_alias_listContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#column_alias}.
 * @param ctx the parse tree
 */
fn enter_column_alias(&mut self, _ctx: &Column_aliasContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#column_alias}.
 * @param ctx the parse tree
 */
fn exit_column_alias(&mut self, _ctx: &Column_aliasContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#table_value_constructor}.
 * @param ctx the parse tree
 */
fn enter_table_value_constructor(&mut self, _ctx: &Table_value_constructorContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#table_value_constructor}.
 * @param ctx the parse tree
 */
fn exit_table_value_constructor(&mut self, _ctx: &Table_value_constructorContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#expression_list}.
 * @param ctx the parse tree
 */
fn enter_expression_list(&mut self, _ctx: &Expression_listContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#expression_list}.
 * @param ctx the parse tree
 */
fn exit_expression_list(&mut self, _ctx: &Expression_listContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#ranking_windowed_function}.
 * @param ctx the parse tree
 */
fn enter_ranking_windowed_function(&mut self, _ctx: &Ranking_windowed_functionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#ranking_windowed_function}.
 * @param ctx the parse tree
 */
fn exit_ranking_windowed_function(&mut self, _ctx: &Ranking_windowed_functionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#aggregate_windowed_function}.
 * @param ctx the parse tree
 */
fn enter_aggregate_windowed_function(&mut self, _ctx: &Aggregate_windowed_functionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#aggregate_windowed_function}.
 * @param ctx the parse tree
 */
fn exit_aggregate_windowed_function(&mut self, _ctx: &Aggregate_windowed_functionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#analytic_windowed_function}.
 * @param ctx the parse tree
 */
fn enter_analytic_windowed_function(&mut self, _ctx: &Analytic_windowed_functionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#analytic_windowed_function}.
 * @param ctx the parse tree
 */
fn exit_analytic_windowed_function(&mut self, _ctx: &Analytic_windowed_functionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#all_distinct_expression}.
 * @param ctx the parse tree
 */
fn enter_all_distinct_expression(&mut self, _ctx: &All_distinct_expressionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#all_distinct_expression}.
 * @param ctx the parse tree
 */
fn exit_all_distinct_expression(&mut self, _ctx: &All_distinct_expressionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#over_clause}.
 * @param ctx the parse tree
 */
fn enter_over_clause(&mut self, _ctx: &Over_clauseContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#over_clause}.
 * @param ctx the parse tree
 */
fn exit_over_clause(&mut self, _ctx: &Over_clauseContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#row_or_range_clause}.
 * @param ctx the parse tree
 */
fn enter_row_or_range_clause(&mut self, _ctx: &Row_or_range_clauseContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#row_or_range_clause}.
 * @param ctx the parse tree
 */
fn exit_row_or_range_clause(&mut self, _ctx: &Row_or_range_clauseContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#window_frame_extent}.
 * @param ctx the parse tree
 */
fn enter_window_frame_extent(&mut self, _ctx: &Window_frame_extentContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#window_frame_extent}.
 * @param ctx the parse tree
 */
fn exit_window_frame_extent(&mut self, _ctx: &Window_frame_extentContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#window_frame_bound}.
 * @param ctx the parse tree
 */
fn enter_window_frame_bound(&mut self, _ctx: &Window_frame_boundContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#window_frame_bound}.
 * @param ctx the parse tree
 */
fn exit_window_frame_bound(&mut self, _ctx: &Window_frame_boundContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#window_frame_preceding}.
 * @param ctx the parse tree
 */
fn enter_window_frame_preceding(&mut self, _ctx: &Window_frame_precedingContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#window_frame_preceding}.
 * @param ctx the parse tree
 */
fn exit_window_frame_preceding(&mut self, _ctx: &Window_frame_precedingContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#window_frame_following}.
 * @param ctx the parse tree
 */
fn enter_window_frame_following(&mut self, _ctx: &Window_frame_followingContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#window_frame_following}.
 * @param ctx the parse tree
 */
fn exit_window_frame_following(&mut self, _ctx: &Window_frame_followingContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#create_database_option}.
 * @param ctx the parse tree
 */
fn enter_create_database_option(&mut self, _ctx: &Create_database_optionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#create_database_option}.
 * @param ctx the parse tree
 */
fn exit_create_database_option(&mut self, _ctx: &Create_database_optionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#database_filestream_option}.
 * @param ctx the parse tree
 */
fn enter_database_filestream_option(&mut self, _ctx: &Database_filestream_optionContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#database_filestream_option}.
 * @param ctx the parse tree
 */
fn exit_database_filestream_option(&mut self, _ctx: &Database_filestream_optionContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#database_file_spec}.
 * @param ctx the parse tree
 */
fn enter_database_file_spec(&mut self, _ctx: &Database_file_specContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#database_file_spec}.
 * @param ctx the parse tree
 */
fn exit_database_file_spec(&mut self, _ctx: &Database_file_specContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#file_group}.
 * @param ctx the parse tree
 */
fn enter_file_group(&mut self, _ctx: &File_groupContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#file_group}.
 * @param ctx the parse tree
 */
fn exit_file_group(&mut self, _ctx: &File_groupContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#file_spec}.
 * @param ctx the parse tree
 */
fn enter_file_spec(&mut self, _ctx: &File_specContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#file_spec}.
 * @param ctx the parse tree
 */
fn exit_file_spec(&mut self, _ctx: &File_specContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#entity_name}.
 * @param ctx the parse tree
 */
fn enter_entity_name(&mut self, _ctx: &Entity_nameContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#entity_name}.
 * @param ctx the parse tree
 */
fn exit_entity_name(&mut self, _ctx: &Entity_nameContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#entity_name_for_azure_dw}.
 * @param ctx the parse tree
 */
fn enter_entity_name_for_azure_dw(&mut self, _ctx: &Entity_name_for_azure_dwContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#entity_name_for_azure_dw}.
 * @param ctx the parse tree
 */
fn exit_entity_name_for_azure_dw(&mut self, _ctx: &Entity_name_for_azure_dwContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#entity_name_for_parallel_dw}.
 * @param ctx the parse tree
 */
fn enter_entity_name_for_parallel_dw(&mut self, _ctx: &Entity_name_for_parallel_dwContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#entity_name_for_parallel_dw}.
 * @param ctx the parse tree
 */
fn exit_entity_name_for_parallel_dw(&mut self, _ctx: &Entity_name_for_parallel_dwContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#full_table_name}.
 * @param ctx the parse tree
 */
fn enter_full_table_name(&mut self, _ctx: &Full_table_nameContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#full_table_name}.
 * @param ctx the parse tree
 */
fn exit_full_table_name(&mut self, _ctx: &Full_table_nameContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#table_name}.
 * @param ctx the parse tree
 */
fn enter_table_name(&mut self, _ctx: &Table_nameContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#table_name}.
 * @param ctx the parse tree
 */
fn exit_table_name(&mut self, _ctx: &Table_nameContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#simple_name}.
 * @param ctx the parse tree
 */
fn enter_simple_name(&mut self, _ctx: &Simple_nameContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#simple_name}.
 * @param ctx the parse tree
 */
fn exit_simple_name(&mut self, _ctx: &Simple_nameContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#func_proc_name_schema}.
 * @param ctx the parse tree
 */
fn enter_func_proc_name_schema(&mut self, _ctx: &Func_proc_name_schemaContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#func_proc_name_schema}.
 * @param ctx the parse tree
 */
fn exit_func_proc_name_schema(&mut self, _ctx: &Func_proc_name_schemaContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#func_proc_name_database_schema}.
 * @param ctx the parse tree
 */
fn enter_func_proc_name_database_schema(&mut self, _ctx: &Func_proc_name_database_schemaContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#func_proc_name_database_schema}.
 * @param ctx the parse tree
 */
fn exit_func_proc_name_database_schema(&mut self, _ctx: &Func_proc_name_database_schemaContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#func_proc_name_server_database_schema}.
 * @param ctx the parse tree
 */
fn enter_func_proc_name_server_database_schema(&mut self, _ctx: &Func_proc_name_server_database_schemaContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#func_proc_name_server_database_schema}.
 * @param ctx the parse tree
 */
fn exit_func_proc_name_server_database_schema(&mut self, _ctx: &Func_proc_name_server_database_schemaContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#ddl_object}.
 * @param ctx the parse tree
 */
fn enter_ddl_object(&mut self, _ctx: &Ddl_objectContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#ddl_object}.
 * @param ctx the parse tree
 */
fn exit_ddl_object(&mut self, _ctx: &Ddl_objectContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#full_column_name}.
 * @param ctx the parse tree
 */
fn enter_full_column_name(&mut self, _ctx: &Full_column_nameContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#full_column_name}.
 * @param ctx the parse tree
 */
fn exit_full_column_name(&mut self, _ctx: &Full_column_nameContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#column_name_list_with_order}.
 * @param ctx the parse tree
 */
fn enter_column_name_list_with_order(&mut self, _ctx: &Column_name_list_with_orderContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#column_name_list_with_order}.
 * @param ctx the parse tree
 */
fn exit_column_name_list_with_order(&mut self, _ctx: &Column_name_list_with_orderContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#column_name_list}.
 * @param ctx the parse tree
 */
fn enter_column_name_list(&mut self, _ctx: &Column_name_listContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#column_name_list}.
 * @param ctx the parse tree
 */
fn exit_column_name_list(&mut self, _ctx: &Column_name_listContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#cursor_name}.
 * @param ctx the parse tree
 */
fn enter_cursor_name(&mut self, _ctx: &Cursor_nameContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#cursor_name}.
 * @param ctx the parse tree
 */
fn exit_cursor_name(&mut self, _ctx: &Cursor_nameContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#on_off}.
 * @param ctx the parse tree
 */
fn enter_on_off(&mut self, _ctx: &On_offContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#on_off}.
 * @param ctx the parse tree
 */
fn exit_on_off(&mut self, _ctx: &On_offContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#clustered}.
 * @param ctx the parse tree
 */
fn enter_clustered(&mut self, _ctx: &ClusteredContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#clustered}.
 * @param ctx the parse tree
 */
fn exit_clustered(&mut self, _ctx: &ClusteredContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#null_notnull}.
 * @param ctx the parse tree
 */
fn enter_null_notnull(&mut self, _ctx: &Null_notnullContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#null_notnull}.
 * @param ctx the parse tree
 */
fn exit_null_notnull(&mut self, _ctx: &Null_notnullContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#null_or_default}.
 * @param ctx the parse tree
 */
fn enter_null_or_default(&mut self, _ctx: &Null_or_defaultContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#null_or_default}.
 * @param ctx the parse tree
 */
fn exit_null_or_default(&mut self, _ctx: &Null_or_defaultContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#scalar_function_name}.
 * @param ctx the parse tree
 */
fn enter_scalar_function_name(&mut self, _ctx: &Scalar_function_nameContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#scalar_function_name}.
 * @param ctx the parse tree
 */
fn exit_scalar_function_name(&mut self, _ctx: &Scalar_function_nameContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#begin_conversation_timer}.
 * @param ctx the parse tree
 */
fn enter_begin_conversation_timer(&mut self, _ctx: &Begin_conversation_timerContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#begin_conversation_timer}.
 * @param ctx the parse tree
 */
fn exit_begin_conversation_timer(&mut self, _ctx: &Begin_conversation_timerContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#begin_conversation_dialog}.
 * @param ctx the parse tree
 */
fn enter_begin_conversation_dialog(&mut self, _ctx: &Begin_conversation_dialogContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#begin_conversation_dialog}.
 * @param ctx the parse tree
 */
fn exit_begin_conversation_dialog(&mut self, _ctx: &Begin_conversation_dialogContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#contract_name}.
 * @param ctx the parse tree
 */
fn enter_contract_name(&mut self, _ctx: &Contract_nameContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#contract_name}.
 * @param ctx the parse tree
 */
fn exit_contract_name(&mut self, _ctx: &Contract_nameContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#service_name}.
 * @param ctx the parse tree
 */
fn enter_service_name(&mut self, _ctx: &Service_nameContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#service_name}.
 * @param ctx the parse tree
 */
fn exit_service_name(&mut self, _ctx: &Service_nameContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#end_conversation}.
 * @param ctx the parse tree
 */
fn enter_end_conversation(&mut self, _ctx: &End_conversationContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#end_conversation}.
 * @param ctx the parse tree
 */
fn exit_end_conversation(&mut self, _ctx: &End_conversationContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#waitfor_conversation}.
 * @param ctx the parse tree
 */
fn enter_waitfor_conversation(&mut self, _ctx: &Waitfor_conversationContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#waitfor_conversation}.
 * @param ctx the parse tree
 */
fn exit_waitfor_conversation(&mut self, _ctx: &Waitfor_conversationContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#get_conversation}.
 * @param ctx the parse tree
 */
fn enter_get_conversation(&mut self, _ctx: &Get_conversationContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#get_conversation}.
 * @param ctx the parse tree
 */
fn exit_get_conversation(&mut self, _ctx: &Get_conversationContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#queue_id}.
 * @param ctx the parse tree
 */
fn enter_queue_id(&mut self, _ctx: &Queue_idContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#queue_id}.
 * @param ctx the parse tree
 */
fn exit_queue_id(&mut self, _ctx: &Queue_idContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#send_conversation}.
 * @param ctx the parse tree
 */
fn enter_send_conversation(&mut self, _ctx: &Send_conversationContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#send_conversation}.
 * @param ctx the parse tree
 */
fn exit_send_conversation(&mut self, _ctx: &Send_conversationContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#data_type}.
 * @param ctx the parse tree
 */
fn enter_data_type(&mut self, _ctx: &Data_typeContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#data_type}.
 * @param ctx the parse tree
 */
fn exit_data_type(&mut self, _ctx: &Data_typeContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#default_value}.
 * @param ctx the parse tree
 */
fn enter_default_value(&mut self, _ctx: &Default_valueContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#default_value}.
 * @param ctx the parse tree
 */
fn exit_default_value(&mut self, _ctx: &Default_valueContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#constant}.
 * @param ctx the parse tree
 */
fn enter_constant(&mut self, _ctx: &ConstantContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#constant}.
 * @param ctx the parse tree
 */
fn exit_constant(&mut self, _ctx: &ConstantContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#sign}.
 * @param ctx the parse tree
 */
fn enter_sign(&mut self, _ctx: &SignContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#sign}.
 * @param ctx the parse tree
 */
fn exit_sign(&mut self, _ctx: &SignContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#id}.
 * @param ctx the parse tree
 */
fn enter_id(&mut self, _ctx: &IdContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#id}.
 * @param ctx the parse tree
 */
fn exit_id(&mut self, _ctx: &IdContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#simple_id}.
 * @param ctx the parse tree
 */
fn enter_simple_id(&mut self, _ctx: &Simple_idContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#simple_id}.
 * @param ctx the parse tree
 */
fn exit_simple_id(&mut self, _ctx: &Simple_idContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#comparison_operator}.
 * @param ctx the parse tree
 */
fn enter_comparison_operator(&mut self, _ctx: &Comparison_operatorContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#comparison_operator}.
 * @param ctx the parse tree
 */
fn exit_comparison_operator(&mut self, _ctx: &Comparison_operatorContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#assignment_operator}.
 * @param ctx the parse tree
 */
fn enter_assignment_operator(&mut self, _ctx: &Assignment_operatorContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#assignment_operator}.
 * @param ctx the parse tree
 */
fn exit_assignment_operator(&mut self, _ctx: &Assignment_operatorContext) { }

/**
 * Enter a parse tree produced by {@link TSqlParser#file_size}.
 * @param ctx the parse tree
 */
fn enter_file_size(&mut self, _ctx: &File_sizeContext) { }
/**
 * Exit a parse tree produced by {@link TSqlParser#file_size}.
 * @param ctx the parse tree
 */
fn exit_file_size(&mut self, _ctx: &File_sizeContext) { }

}
