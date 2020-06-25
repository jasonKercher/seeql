/*
T-SQL (Transact-SQL, MSSQL) grammar.
The MIT License (MIT).
Copyright (c) 2017, Mark Adams (madams51703@gmail.com)
Copyright (c) 2015-2017, Ivan Kochurkin (kvanttt@gmail.com), Positive Technologies.
Copyright (c) 2016, Scott Ure (scott@redstormsoftware.com).
Copyright (c) 2016, Rui Zhang (ruizhang.ccs@gmail.com).
Copyright (c) 2016, Marcus Henriksson (kuseman80@gmail.com).
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
*/

parser grammar TSqlParser;

options { tokenVocab=TSqlLexer; }

tsqlFile
    : batch* EOF
    ;

batch
    : executeBody goStatement*
    | executeBody? sqlClauses goStatement*
    ;

sqlClauses
    : (sqlClause SEMI?)+
    ;

sqlClause
    : dmlClause

    | ddlClause

    | cflStatement

    | dbccClause

    | emptyStatement

    | anotherStatement

    | backupStatement
    ;

// Data Manipulation Language: https://msdn.microsoft.com/en-us/library/ff848766(v=sql.120).aspx
dmlClause
    : mergeStatement
    | deleteStatement
    | insertStatement
    | selectStatement
    | updateStatement
    ;

// Data Definition Language: https://msdn.microsoft.com/en-us/library/ff848799.aspx)
ddlClause
    : alterApplicationRole
    | alterAssembly
    | alterAsymmetricKey
    | alterAuthorization
    | alterAuthorizationForAzureDw
    | alterAuthorizationForParallelDw
    | alterAuthorizationForSqlDatabase
    | alterAvailabilityGroup
    | alterCertificate
    | alterColumnEncryptionKey
    | alterCredential
    | alterCryptographicProvider
    | alterDatabase
    | alterDbRole
    | alterEndpoint
    | createOrAlterEventSession
    | alterExternalDataSource
    | alterExternalLibrary
    | alterExternalResourcePool
    | alterFulltextCatalog
    | alterFulltextStoplist
    | alterLoginAzureSql
    | alterLoginAzureSqlDwAndPdw
    | alterLoginSqlServer
    | alterMasterKeyAzureSql
    | alterMasterKeySqlServer
    | alterMessageType
    | alterPartitionFunction
    | alterPartitionScheme
    | alterRemoteServiceBinding
    | alterResourceGovernor
    | alterSchemaAzureSqlDwAndPdw
    | alterSchemaSql
    | alterSequence
    | alterServerAudit
    | alterServerAuditSpecification
    | alterServerConfiguration
    | alterServerRole
    | alterServerRolePdw
    | alterService
    | alterServiceMasterKey
    | alterSymmetricKey
    | alterTable
    | alterUser
    | alterUserAzureSql
    | alterWorkloadGroup
    | createApplicationRole
    | createAssembly
    | createAsymmetricKey
    | createColumnEncryptionKey
    | createColumnMasterKey
    | createCredential
    | createCryptographicProvider
    | createDatabase
    | createDbRole
    | createEventNotification
    | createExternalLibrary
    | createExternalResourcePool
    | createFulltextCatalog
    | createFulltextStoplist
    | createIndex
    | createLoginAzureSql
    | createLoginPdw
    | createLoginSqlServer
    | createMasterKeyAzureSql
    | createMasterKeySqlServer
    | createOrAlterBrokerPriority
    | createOrAlterFunction
    | createOrAlterProcedure
    | createOrAlterTrigger
    | createRemoteServiceBinding
    | createResourcePool
    | createRoute
    | createRule
    | createSchema
    | createSchemaAzureSqlDwAndPdw
    | createSearchPropertyList
    | createSecurityPolicy
    | createSequence
    | createServerAudit
    | createServerAuditSpecification
    | createServerRole
    | createService
    | createStatistics
    | createSymmetricKey
    | createSynonym
    | createTable
    | createType
    | createUser
    | createUserAzureSqlDw
    | createView
    | createWorkloadGroup
    | createXmlSchemaCollection
    | dropAggregate
    | dropApplicationRole
    | dropAssembly
    | dropAsymmetricKey
    | dropAvailabilityGroup
    | dropBrokerPriority
    | dropCertificate
    | dropColumnEncryptionKey
    | dropColumnMasterKey
    | dropContract
    | dropCredential
    | dropCryptograhicProvider
    | dropDatabase
    | dropDatabaseAuditSpecification
    | dropDatabaseScopedCredential
    | dropDbRole
    | dropDefault
    | dropEndpoint
    | dropEventNotifications
    | dropEventSession
    | dropExternalDataSource
    | dropExternalFileFormat
    | dropExternalLibrary
    | dropExternalResourcePool
    | dropExternalTable
    | dropFulltextCatalog
    | dropFulltextIndex
    | dropFulltextStoplist
    | dropFunction
    | dropIndex
    | dropLogin
    | dropMasterKey
    | dropMessageType
    | dropPartitionFunction
    | dropPartitionScheme
    | dropProcedure
    | dropQueue
    | dropRemoteServiceBinding
    | dropResourcePool
    | dropRoute
    | dropRule
    | dropSchema
    | dropSearchPropertyList
    | dropSecurityPolicy
    | dropSequence
    | dropServerAudit
    | dropServerAuditSpecification
    | dropServerRole
    | dropService
    | dropSignature
    | dropStatistics
    | dropStatisticsNameAzureDwAndPdw
    | dropSymmetricKey
    | dropSynonym
    | dropTable
    | dropTrigger
    | dropType
    | dropUser
    | dropView
    | dropWorkloadGroup
    | dropXmlSchemaCollection
    | disableTrigger
    | enableTrigger
    | lockTable
    | truncateTable
    | updateStatistics
    ;
backupStatement
    : backupDatabase
    | backupLog
    | backupCertificate
    | backupMasterKey
    | backupServiceMasterKey
    ;

// Control-of-Flow Language: https://docs.microsoft.com/en-us/sql/t-sql/language-elements/control-of-flow
cflStatement
    : blockStatement
    | breakStatement
    | continueStatement
    | gotoStatement
    | ifStatement
    | returnStatement
    | throwStatement
    | tryCatchStatement
    | waitforStatement
    | whileStatement
    | printStatement
    | raiseerrorStatement
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/language-elements/begin-end-transact-sql
blockStatement
    : BEGIN ';'? sqlClauses? END ';'?
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/language-elements/break-transact-sql
breakStatement
    : BREAK ';'?
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/language-elements/continue-transact-sql
continueStatement
    : CONTINUE ';'?
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/language-elements/goto-transact-sql
gotoStatement
    : GOTO id ';'?
    | id ':' ';'?
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/language-elements/return-transact-sql
returnStatement
    : RETURN expression? ';'?
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/language-elements/if-else-transact-sql
ifStatement
    : IF searchCondition sqlClause (ELSE sqlClause)? ';'?
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/language-elements/throw-transact-sql
throwStatement
    : THROW (throwErrorNumber ',' throwMessage ',' throwState)? ';'?
    ;

throwErrorNumber
    : DECIMAL | LOCAL_ID
    ;

throwMessage
    : STRING | LOCAL_ID
    ;

throwState
    : DECIMAL | LOCAL_ID
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/language-elements/try-catch-transact-sql
tryCatchStatement
    : BEGIN TRY ';'? tryClauses=sqlClauses? END TRY ';'? BEGIN CATCH ';'? catchClauses=sqlClauses? END CATCH ';'?
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/language-elements/waitfor-transact-sql
waitforStatement
    : WAITFOR receiveStatement? ','? ((DELAY | TIME | TIMEOUT) time)?  expression? ';'?
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/language-elements/while-transact-sql
whileStatement
    : WHILE searchCondition (sqlClause | BREAK ';'? | CONTINUE ';'?)
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/language-elements/print-transact-sql
printStatement
    : PRINT (expression | DOUBLE_QUOTE_ID) (',' LOCAL_ID)* ';'?
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/language-elements/raiserror-transact-sql
raiseerrorStatement
    : RAISERROR '(' msg=(DECIMAL | STRING | LOCAL_ID) ',' severity=constant_LOCAL_ID ','
    state=constant_LOCAL_ID (',' constant_LOCAL_ID)* ')' (WITH (LOG | SETERROR))? ';'?
    | RAISERROR DECIMAL formatstring=(STRING | LOCAL_ID | DOUBLE_QUOTE_ID) (',' argument=(DECIMAL | STRING | LOCAL_ID))*
    ;

emptyStatement
    : ';'
    ;

anotherStatement
    : declareStatement
    | cursorStatement
    | conversationStatement
    | createContract
    | createQueue
    | alterQueue
    | executeStatement
    | messageStatement
    | securityStatement
    | setStatement
    | transactionStatement
    | useStatement
    | setuserStatement
    ;
// https://docs.microsoft.com/en-us/sql/t-sql/statements/alter-application-role-transact-sql

alterApplicationRole
    : ALTER APPLICATION ROLE applictionRole=id WITH  (COMMA? NAME EQUAL newApplicationRoleName=id)? (COMMA? PASSWORD EQUAL applicationRolePassword=STRING)? (COMMA? DEFAULT_SCHEMA EQUAL appRoleDefaultSchema=id)?
    ;

createApplicationRole
    : CREATE APPLICATION ROLE applictionRole=id WITH   (COMMA? PASSWORD EQUAL applicationRolePassword=STRING)? (COMMA? DEFAULT_SCHEMA EQUAL appRoleDefaultSchema=id)?
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-aggregate-transact-sql

dropAggregate
   : DROP AGGREGATE ( IF EXISTS )? ( schemaName=id DOT )? aggregateName=id
   ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-application-role-transact-sql
dropApplicationRole
    : DROP APPLICATION ROLE rolename=id
    ;

alterAssembly
    : alterAssemblyStart assemblyName=id alterAssemblyClause
    ;

alterAssemblyStart
    :  ALTER ASSEMBLY
    ;

alterAssemblyClause
    : alterAssemblyFromClause? alterAssemblyWithClause? alterAssemblyDropClause? alterAssemblyAddClause?
    ;

alterAssemblyFromClause
    : alterAssemblyFromClauseStart (clientAssemblySpecifier | alterAssemblyFileBits )
    ;

alterAssemblyFromClauseStart
    : FROM
    ;

alterAssemblyDropClause
    : alterAssemblyDrop alterAssemblyDropMultipleFiles
    ;

alterAssemblyDropMultipleFiles
    : ALL
    | multipleLocalFiles
    ;

alterAssemblyDrop
    : DROP
    ;

alterAssemblyAddClause
    : alterAsssemblyAddClauseStart alterAssemblyClientFileClause
    ;

alterAsssemblyAddClauseStart
    : ADD FILE FROM
    ;

// need to implement
alterAssemblyClientFileClause
    :  alterAssemblyFileName (alterAssemblyAs id)?
    ;

alterAssemblyFileName
    : STRING
    ;

//need to implement
alterAssemblyFileBits
    : alterAssemblyAs id
    ;

alterAssemblyAs
    : AS
    ;

alterAssemblyWithClause
    : alterAssemblyWith assemblyOption
    ;

alterAssemblyWith
    : WITH
    ;

clientAssemblySpecifier
    : networkFileShare
    | localFile
    | STRING
    ;

assemblyOption
    : PERMISSION_SET EQUAL (SAFE|EXTERNAL_ACCESS|UNSAFE)
    | VISIBILITY EQUAL (ON | OFF)
    | UNCHECKED DATA
    | assemblyOption COMMA
    ;

networkFileShare
    : networkFileStart networkComputer filePath
    ;

networkComputer
    : computerName=id
    ;

networkFileStart
    : DOUBLE_BACK_SLASH
    ;

filePath
    : fileDirectoryPathSeparator filePath
    | id
    ;

fileDirectoryPathSeparator
    : '\\'
    ;

localFile
    : localDrive filePath
    ;

localDrive
    :
    DISK_DRIVE
    ;
multipleLocalFiles
    :
    multipleLocalFileStart localFile SINGLE_QUOTE COMMA
    | localFile
    ;

multipleLocalFileStart
    : SINGLE_QUOTE
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/create-assembly-transact-sql
createAssembly
    : CREATE ASSEMBLY assemblyName=id (AUTHORIZATION ownerName=id)?
       FROM (COMMA? (STRING|BINARY) )+
       (WITH PERMISSION_SET EQUAL (SAFE|EXTERNAL_ACCESS|UNSAFE) )?

    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-assembly-transact-sql
dropAssembly
    : DROP ASSEMBLY ( IF EXISTS )? (COMMA? assemblyName=id)+
       ( WITH NO DEPENDENTS )?
    ;
// https://docs.microsoft.com/en-us/sql/t-sql/statements/alter-asymmetric-key-transact-sql

alterAsymmetricKey
    :
    alterAsymmetricKeyStart Asym_Key_Name=id (asymmetricKeyOption | REMOVE PRIVATE KEY )
    ;

alterAsymmetricKeyStart
    : ALTER ASYMMETRIC KEY
    ;

asymmetricKeyOption
    : asymmetricKeyOptionStart asymmetricKeyPasswordChangeOption ( COMMA asymmetricKeyPasswordChangeOption)? RR_BRACKET
    ;

asymmetricKeyOptionStart
    : WITH PRIVATE KEY LR_BRACKET
    ;

asymmetricKeyPasswordChangeOption
    : DECRYPTION BY PASSWORD EQUAL STRING
    | ENCRYPTION BY PASSWORD EQUAL STRING
    ;


//https://docs.microsoft.com/en-us/sql/t-sql/statements/create-asymmetric-key-transact-sql

createAsymmetricKey
    : CREATE ASYMMETRIC KEY Asym_Key_Nam=id
       (AUTHORIZATION databasePrincipalName=id)?
       ( FROM (FILE EQUAL STRING |EXECUTABLE_FILE EQUAL STRING|ASSEMBLY Assembly_Name=id | PROVIDER Provider_Name=id) )?
       (WITH (ALGORITHM EQUAL ( RSA_4096 | RSA_3072 | RSA_2048 | RSA_1024 | RSA_512)  |PROVIDER_KEY_NAME EQUAL providerKeyName=STRING | CREATION_DISPOSITION EQUAL (CREATE_NEW|OPEN_EXISTING)  )   )?
       (ENCRYPTION BY PASSWORD EQUAL asymmetricKeyPassword=STRING )?
     ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-asymmetric-key-transact-sql
dropAsymmetricKey
     : DROP ASYMMETRIC KEY keyName=id ( REMOVE PROVIDER KEY )?
     ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/alter-authorization-transact-sql

alterAuthorization
    : alterAuthorizationStart (classType colonColon)? entity=entityName entityTo authorizationGrantee
    ;

authorizationGrantee
    : principalName=id
    | SCHEMA OWNER
    ;

entityTo
    : TO
    ;

colonColon
    : COLON COLON
    ;

alterAuthorizationStart
    : ALTER AUTHORIZATION ON
    ;

alterAuthorizationForSqlDatabase
    : alterAuthorizationStart (classTypeForSqlDatabase colonColon)? entity=entityName entityTo authorizationGrantee
    ;

alterAuthorizationForAzureDw
    : alterAuthorizationStart (classTypeForAzureDw colonColon)? entity=entityNameForAzureDw entityTo authorizationGrantee
    ;

alterAuthorizationForParallelDw
    : alterAuthorizationStart (classTypeForParallelDw colonColon)? entity=entityNameForParallelDw entityTo authorizationGrantee
    ;


classType
    : OBJECT
    | ASSEMBLY
    | ASYMMETRIC KEY
    | AVAILABILITY GROUP
    | CERTIFICATE
    | CONTRACT
    | TYPE
    | DATABASE
    | ENDPOINT
    | FULLTEXT CATALOG
    | FULLTEXT STOPLIST
    | MESSAGE TYPE
    | REMOTE SERVICE BINDING
    | ROLE
    | ROUTE
    | SCHEMA
    | SEARCH PROPERTY LIST
    | SERVER ROLE
    | SERVICE
    | SYMMETRIC KEY
    | XML SCHEMA COLLECTION
    ;

classTypeForSqlDatabase
    :  OBJECT
    | ASSEMBLY
    | ASYMMETRIC KEY
    | CERTIFICATE
    | TYPE
    | DATABASE
    | FULLTEXT CATALOG
    | FULLTEXT STOPLIST
    | ROLE
    | SCHEMA
    | SEARCH PROPERTY LIST
    | SYMMETRIC KEY
    | XML SCHEMA COLLECTION
    ;

classTypeForAzureDw
    : SCHEMA
    | OBJECT
    ;

classTypeForParallelDw
    : DATABASE
    | SCHEMA
    | OBJECT
    ;


// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-availability-group-transact-sql
dropAvailabilityGroup
    :  DROP AVAILABILITY GROUP groupName=id
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/alter-availability-group-transact-sql
alterAvailabilityGroup
    : alterAvailabilityGroupStart alterAvailabilityGroupOptions
    ;

alterAvailabilityGroupStart
    : ALTER AVAILABILITY GROUP groupName=id
    ;

alterAvailabilityGroupOptions
    : SET LR_BRACKET ( ( AUTOMATED_BACKUP_PREFERENCE EQUAL ( PRIMARY | SECONDARY_ONLY| SECONDARY | NONE )  | FAILURE_CONDITION_LEVEL  EQUAL DECIMAL   | HEALTH_CHECK_TIMEOUT EQUAL milliseconds=DECIMAL  | DB_FAILOVER  EQUAL ( ON | OFF )   | REQUIRED_SYNCHRONIZED_SECONDARIES_TO_COMMIT EQUAL DECIMAL ) RR_BRACKET  )
    | ADD DATABASE databaseName=id
    | REMOVE DATABASE databaseName=id
    | ADD REPLICA ON serverInstance=STRING (WITH LR_BRACKET ( (ENDPOINT_URL EQUAL STRING)?   (COMMA? AVAILABILITY_MODE EQUAL (SYNCHRONOUS_COMMIT| ASYNCHRONOUS_COMMIT))?    (COMMA? FAILOVER_MODE EQUAL (AUTOMATIC|MANUAL) )?  (COMMA?   SEEDING_MODE EQUAL (AUTOMATIC|MANUAL) )?  (COMMA?  BACKUP_PRIORITY EQUAL DECIMAL)?  ( COMMA? PRIMARY_ROLE  LR_BRACKET ALLOW_CONNECTIONS EQUAL ( READ_WRITE | ALL ) RR_BRACKET)?   ( COMMA? SECONDARY_ROLE  LR_BRACKET ALLOW_CONNECTIONS EQUAL ( READ_ONLY  ) RR_BRACKET )? )
)    RR_BRACKET
        |SECONDARY_ROLE LR_BRACKET (ALLOW_CONNECTIONS EQUAL (NO|READ_ONLY|ALL) | READ_ONLY_ROUTING_LIST EQUAL ( LR_BRACKET ( ( STRING) ) RR_BRACKET ) )
        |PRIMARY_ROLE LR_BRACKET (ALLOW_CONNECTIONS EQUAL (NO|READ_ONLY|ALL) | READ_ONLY_ROUTING_LIST EQUAL ( LR_BRACKET ( (COMMA? STRING)*|NONE ) RR_BRACKET )
        | SESSION_TIMEOUT EQUAL sessionTimeout=DECIMAL
)
    | MODIFY REPLICA ON serverInstance=STRING (WITH LR_BRACKET (ENDPOINT_URL EQUAL STRING|  AVAILABILITY_MODE EQUAL (SYNCHRONOUS_COMMIT| ASYNCHRONOUS_COMMIT)  | FAILOVER_MODE EQUAL (AUTOMATIC|MANUAL) |   SEEDING_MODE EQUAL (AUTOMATIC|MANUAL)  |  BACKUP_PRIORITY EQUAL DECIMAL  )
        |SECONDARY_ROLE LR_BRACKET (ALLOW_CONNECTIONS EQUAL (NO|READ_ONLY|ALL) | READ_ONLY_ROUTING_LIST EQUAL ( LR_BRACKET ( ( STRING) ) RR_BRACKET ) )
        |PRIMARY_ROLE LR_BRACKET (ALLOW_CONNECTIONS EQUAL (NO|READ_ONLY|ALL) | READ_ONLY_ROUTING_LIST EQUAL ( LR_BRACKET ( (COMMA? STRING)*|NONE ) RR_BRACKET )
         | SESSION_TIMEOUT EQUAL sessionTimeout=DECIMAL
)   ) RR_BRACKET
    | REMOVE REPLICA ON STRING
    | JOIN
    | JOIN AVAILABILITY GROUP ON (COMMA? agName=STRING WITH LR_BRACKET ( LISTENER_URL EQUAL STRING COMMA AVAILABILITY_MODE EQUAL (SYNCHRONOUS_COMMIT|ASYNCHRONOUS_COMMIT) COMMA FAILOVER_MODE EQUAL MANUAL COMMA SEEDING_MODE EQUAL (AUTOMATIC|MANUAL) RR_BRACKET ) )+
     | MODIFY AVAILABILITY GROUP ON (COMMA? agNameModified=STRING WITH LR_BRACKET (LISTENER_URL EQUAL STRING  (COMMA? AVAILABILITY_MODE EQUAL (SYNCHRONOUS_COMMIT|ASYNCHRONOUS_COMMIT) )? (COMMA? FAILOVER_MODE EQUAL MANUAL )? (COMMA? SEEDING_MODE EQUAL (AUTOMATIC|MANUAL))? RR_BRACKET ) )+
    |GRANT CREATE ANY DATABASE
    | DENY CREATE ANY DATABASE
    | FAILOVER
    | FORCE_FAILOVER_ALLOW_DATA_LOSS
    | ADD LISTENER listenerName=STRING  LR_BRACKET ( WITH DHCP (ON LR_BRACKET (IPV4_ADDR IPV4_ADDR ) RR_BRACKET ) | WITH IP LR_BRACKET (    (COMMA? LR_BRACKET ( IPV4_ADDR  COMMA  IPV4_ADDR  | IPV6_ADDR  ) RR_BRACKET)+ RR_BRACKET  (COMMA PORT EQUAL DECIMAL)? ) ) RR_BRACKET
    | MODIFY LISTENER (ADD IP LR_BRACKET (IPV4_ADDR IPV4_ADDR | IPV6_ADDR) RR_BRACKET | PORT EQUAL DECIMAL )
    |RESTART LISTENER STRING
    |REMOVE LISTENER STRING
    |OFFLINE
    | WITH LR_BRACKET DTC_SUPPORT EQUAL PER_DB RR_BRACKET
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/alter-broker-priority-transact-sql
// https://docs.microsoft.com/en-us/sql/t-sql/statements/create-broker-priority-transact-sql
createOrAlterBrokerPriority
    : (CREATE | ALTER) BROKER PRIORITY ConversationPriorityName=id FOR CONVERSATION
      SET LR_BRACKET
     ( CONTRACT_NAME EQUAL ( ( id) | ANY )  COMMA?  )?
     ( LOCAL_SERVICE_NAME EQUAL (DOUBLE_FORWARD_SLASH? id | ANY ) COMMA? )?
     ( REMOTE_SERVICE_NAME  EQUAL (RemoteServiceName=STRING | ANY ) COMMA? )?
     ( PRIORITY_LEVEL EQUAL ( PriorityValue=DECIMAL | DEFAULT ) ) ?
     RR_BRACKET
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-broker-priority-transact-sql
dropBrokerPriority
    : DROP BROKER PRIORITY ConversationPriorityName=id
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/alter-certificate-transact-sql
alterCertificate
    : ALTER CERTIFICATE certificateName=id (REMOVE PRIVATE_KEY | WITH PRIVATE KEY LR_BRACKET ( FILE EQUAL STRING COMMA? | DECRYPTION BY PASSWORD EQUAL STRING COMMA?| ENCRYPTION BY PASSWORD EQUAL STRING  COMMA?)+ RR_BRACKET | WITH ACTIVE FOR BEGIN_DIALOG EQUAL ( ON | OFF ) )
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/alter-column-encryption-key-transact-sql
alterColumnEncryptionKey
    : ALTER COLUMN ENCRYPTION KEY columnEncryptionKey=id (ADD | DROP) VALUE LR_BRACKET COLUMN_MASTER_KEY EQUAL columnMasterKeyName=id ( COMMA ALGORITHM EQUAL algorithmName=STRING  COMMA ENCRYPTED_VALUE EQUAL BINARY)? RR_BRACKET
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/create-column-encryption-key-transact-sql
createColumnEncryptionKey
    :   CREATE COLUMN ENCRYPTION KEY columnEncryptionKey=id
         WITH VALUES
           (LR_BRACKET COMMA? COLUMN_MASTER_KEY EQUAL columnMasterKeyName=id COMMA
           ALGORITHM EQUAL algorithmName=STRING  COMMA
           ENCRYPTED_VALUE EQUAL encryptedValue=BINARY RR_BRACKET COMMA?)+
     ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-certificate-transact-sql
dropCertificate
     : DROP CERTIFICATE certificateName=id
     ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-column-encryption-key-transact-sql
dropColumnEncryptionKey
    : DROP COLUMN ENCRYPTION KEY keyName=id
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-column-master-key-transact-sql
dropColumnMasterKey
    : DROP COLUMN MASTER KEY keyName=id
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-contract-transact-sql
dropContract
    : DROP CONTRACT droppedContractName=id
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-credential-transact-sql
dropCredential
    : DROP CREDENTIAL credentialName=id
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-cryptographic-provider-transact-sql
dropCryptograhicProvider
    : DROP CRYPTOGRAPHIC PROVIDER providerName=id
    ;


// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-database-transact-sql
dropDatabase
    : DROP DATABASE ( IF EXISTS )? (COMMA? databaseNameOrDatabaseSnapshotName=id)+
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-database-audit-specification-transact-sql
dropDatabaseAuditSpecification
    : DROP DATABASE AUDIT SPECIFICATION auditSpecificationName=id
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-database-scoped-credential-transact-sql
dropDatabaseScopedCredential
   : DROP DATABASE SCOPED CREDENTIAL credentialName=id
   ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-default-transact-sql
dropDefault
    : DROP DEFAULT ( IF EXISTS )? (COMMA? (schemaName=id DOT)? defaultName=id)
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-endpoint-transact-sql
dropEndpoint
    : DROP ENDPOINT endPointName=id
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-external-data-source-transact-sql
dropExternalDataSource
    : DROP EXTERNAL DATA SOURCE externalDataSourceName=id
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-external-file-format-transact-sql
dropExternalFileFormat
    : DROP EXTERNAL FILE FORMAT externalFileFormatName=id
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-external-library-transact-sql
dropExternalLibrary
    : DROP EXTERNAL LIBRARY libraryName=id
( AUTHORIZATION ownerName=id )?
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-external-resource-pool-transact-sql
dropExternalResourcePool
    : DROP EXTERNAL RESOURCE POOL poolName=id
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-external-table-transact-sql
dropExternalTable
    : DROP EXTERNAL TABLE (databaseName=id DOT)? (schemaName=id DOT)? table=id
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-event-notification-transact-sql
dropEventNotifications
    : DROP EVENT NOTIFICATION (COMMA? notificationName=id)+
        ON (SERVER|DATABASE|QUEUE queueName=id)
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-event-session-transact-sql
dropEventSession
    : DROP EVENT SESSION eventSessionName=id
        ON SERVER
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-fulltext-catalog-transact-sql
dropFulltextCatalog
   : DROP FULLTEXT CATALOG catalogName=id
   ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-fulltext-index-transact-sql
dropFulltextIndex
   : DROP FULLTEXT INDEX ON (schema=id DOT)? table=id
   ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-fulltext-stoplist-transact-sql
dropFulltextStoplist
   : DROP FULLTEXT STOPLIST stoplistName=id
   ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-login-transact-sql
dropLogin
     : DROP LOGIN loginName=id
     ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-master-key-transact-sql
dropMasterKey
     : DROP MASTER KEY
     ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-message-type-transact-sql
dropMessageType
     : DROP MESSAGE TYPE messageTypeName=id
     ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-partition-function-transact-sql
dropPartitionFunction
     : DROP PARTITION FUNCTION partitionFunctionName=id
     ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-partition-scheme-transact-sql
dropPartitionScheme
     : DROP PARTITION SCHEME partitionSchemeName=id
     ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-queue-transact-sql
dropQueue
     : DROP QUEUE (databaseName=id DOT)? (schemaName=id DOT)? queueName=id
     ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-remote-service-binding-transact-sql
dropRemoteServiceBinding
     : DROP REMOTE SERVICE BINDING bindingName=id
     ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-resource-pool-transact-sql
dropResourcePool
     : DROP RESOURCE POOL poolName=id
     ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-role-transact-sql
dropDbRole
     : DROP ROLE ( IF EXISTS )? roleName=id
     ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-route-transact-sql
dropRoute
     : DROP ROUTE routeName=id
     ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-rule-transact-sql
dropRule
     : DROP RULE ( IF EXISTS )? (COMMA? (schemaName=id DOT)? ruleName=id)?
     ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-schema-transact-sql
dropSchema
     :  DROP SCHEMA  ( IF EXISTS )? schemaName=id
     ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-search-property-list-transact-sql
dropSearchPropertyList
     : DROP SEARCH PROPERTY LIST propertyListName=id
     ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-security-policy-transact-sql
dropSecurityPolicy
     : DROP SECURITY POLICY ( IF EXISTS )? (schemaName=id DOT )? securityPolicyName=id
     ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-sequence-transact-sql
dropSequence
     : DROP SEQUENCE ( IF EXISTS )? ( COMMA? (databaseName=id DOT)? (schemaName=id DOT)?          sequenceName=id )?
     ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-server-audit-transact-sql
dropServerAudit
     : DROP SERVER AUDIT auditName=id
     ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-server-audit-specification-transact-sql
dropServerAuditSpecification
     : DROP SERVER AUDIT SPECIFICATION auditSpecificationName=id
     ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-server-role-transact-sql
dropServerRole
     : DROP SERVER ROLE roleName=id
     ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-service-transact-sql
dropService
     : DROP SERVICE droppedServiceName=id
     ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-signature-transact-sql
dropSignature
     : DROP ( COUNTER )? SIGNATURE FROM (schemaName=id DOT)? moduleName=id
         BY (COMMA?  CERTIFICATE certName=id
            | COMMA? ASYMMETRIC KEY AsymKeyName=id
            )+
     ;


dropStatisticsNameAzureDwAndPdw
     :  DROP STATISTICS  (schemaName=id DOT)? objectName=id DOT statisticsName=id
     ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-symmetric-key-transact-sql
dropSymmetricKey
     : DROP SYMMETRIC KEY symmetricKeyName=id (REMOVE PROVIDER KEY)?
     ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-synonym-transact-sql
dropSynonym
     : DROP SYNONYM ( IF EXISTS )? ( schema=id DOT )? synonymName=id
     ;


// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-user-transact-sql
dropUser
     : DROP USER ( IF EXISTS )? userName=id
     ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-workload-group-transact-sql
dropWorkloadGroup
     : DROP WORKLOAD GROUP groupName=id
     ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-xml-schema-collection-transact-sql
dropXmlSchemaCollection
     : DROP XML SCHEMA COLLECTION ( relationalSchema=id DOT )?  sqlIdentifier=id
     ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/disable-trigger-transact-sql
disableTrigger
     : DISABLE TRIGGER ( ( COMMA? (schemaName=id DOT)? triggerName=id )+ | ALL)         ON ((schemaId=id DOT)? objectName=id|DATABASE|ALL SERVER)
     ;


// https://docs.microsoft.com/en-us/sql/t-sql/statements/enable-trigger-transact-sql
enableTrigger
     : ENABLE TRIGGER ( ( COMMA? (schemaName=id DOT)? triggerName=id )+ | ALL)         ON ( (schemaId=id DOT)? objectName=id|DATABASE|ALL SERVER)
     ;

lockTable
     : LOCK TABLE tableName IN (SHARE | EXCLUSIVE) MODE (WAIT seconds=DECIMAL | NOWAIT)? ';'?
     ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/truncate-table-transact-sql
truncateTable
     : TRUNCATE TABLE tableName
          ( WITH LR_BRACKET
              PARTITIONS LR_BRACKET
                                (COMMA? (DECIMAL|DECIMAL TO DECIMAL) )+
                         RR_BRACKET

                 RR_BRACKET
          )?
     ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/create-column-master-key-transact-sql
createColumnMasterKey
     : CREATE COLUMN MASTER KEY keyName=id
         WITH LR_BRACKET
            KEY_STORE_PROVIDER_NAME EQUAL  keyStoreProviderName=STRING COMMA
            KEY_PATH EQUAL keyPath=STRING
           RR_BRACKET
      ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/alter-credential-transact-sql
alterCredential
    : ALTER CREDENTIAL credentialName=id
        WITH IDENTITY EQUAL identityName=STRING
         ( COMMA SECRET EQUAL secret=STRING )?
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/create-credential-transact-sql
createCredential
    : CREATE CREDENTIAL credentialName=id
        WITH IDENTITY EQUAL identityName=STRING
         ( COMMA SECRET EQUAL secret=STRING )?
         (  FOR CRYPTOGRAPHIC PROVIDER cryptographicProviderName=id )?
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/alter-cryptographic-provider-transact-sql
alterCryptographicProvider
    : ALTER CRYPTOGRAPHIC PROVIDER providerName=id (FROM FILE EQUAL cryptoProviderDdlFile=STRING)? (ENABLE | DISABLE)?
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/create-cryptographic-provider-transact-sql
createCryptographicProvider
    : CREATE CRYPTOGRAPHIC PROVIDER providerName=id
      FROM FILE EQUAL pathOf_DLL=STRING
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/create-event-notification-transact-sql
createEventNotification
    : CREATE EVENT NOTIFICATION eventNotificationName=id
      ON (SERVER|DATABASE|QUEUE queueName=id)
        (WITH FAN_IN)?
        FOR (COMMA? eventTypeOrGroup=id)+
          TO SERVICE  brokerService=STRING  COMMA
             brokerServiceSpecifierOrCurrentDatabase=STRING
    ;


// https://docs.microsoft.com/en-us/sql/t-sql/statements/alter-event-session-transact-sql
// https://docs.microsoft.com/en-us/sql/t-sql/statements/create-event-session-transact-sql
// todo: not implemented
createOrAlterEventSession
    : (CREATE | ALTER) EVENT SESSION eventSessionName=id ON SERVER
       (COMMA? ADD EVENT ( (eventModuleGuid=id DOT)? eventPackageName=id DOT eventName=id)
        (LR_BRACKET
          (SET ( COMMA? eventCustomizableAttributue=id EQUAL (DECIMAL|STRING) )* )?
          ( ACTION LR_BRACKET (COMMA? (eventModuleGuid=id DOT)? eventPackageName=id DOT actionName=id)+  RR_BRACKET)+
          (WHERE eventSessionPredicateExpression)?
         RR_BRACKET )*
      )*
      (COMMA? DROP EVENT (eventModuleGuid=id DOT)? eventPackageName=id DOT eventName=id )*

      ( (ADD TARGET (eventModuleGuid=id DOT)? eventPackageName=id DOT targetName=id ) ( LR_BRACKET SET (COMMA? targetParameterName=id EQUAL (LR_BRACKET? DECIMAL RR_BRACKET? |STRING) )+ RR_BRACKET )* )*
       (DROP TARGET (eventModuleGuid=id DOT)? eventPackageName=id DOT targetName=id )*


     (WITH
           LR_BRACKET
           (COMMA? MAX_MEMORY EQUAL maxMemory=DECIMAL (KB|MB) )?
           (COMMA? EVENT_RETENTION_MODE EQUAL (ALLOW_SINGLE_EVENT_LOSS | ALLOW_MULTIPLE_EVENT_LOSS | NO_EVENT_LOSS ) )?
           (COMMA? MAX_DISPATCH_LATENCY EQUAL (maxDispatchLatencySeconds=DECIMAL SECONDS | INFINITE) )?
           (COMMA?  MAX_EVENT_SIZE EQUAL maxEventSize=DECIMAL (KB|MB) )?
           (COMMA? MEMORY_PARTITION_MODE EQUAL (NONE | PER_NODE | PER_CPU) )?
           (COMMA? TRACK_CAUSALITY EQUAL (ON|OFF) )?
           (COMMA? STARTUP_STATE EQUAL (ON|OFF) )?
           RR_BRACKET
     )?
     (STATE EQUAL (START|STOP) )?

    ;

eventSessionPredicateExpression
     : ( COMMA? (AND|OR)? NOT? ( eventSessionPredicateFactor | LR_BRACKET eventSessionPredicateExpression RR_BRACKET) )+
     ;

eventSessionPredicateFactor
     : eventSessionPredicateLeaf
     | LR_BRACKET eventSessionPredicateExpression RR_BRACKET
     ;

eventSessionPredicateLeaf
     : (eventFieldName=id | (eventFieldName=id |( (eventModuleGuid=id DOT)?  eventPackageName=id DOT predicateSourceName=id ) ) (EQUAL |(LESS GREATER) | (EXCLAMATION EQUAL) | GREATER  | (GREATER EQUAL)| LESS | LESS EQUAL) (DECIMAL | STRING) )
     | (eventModuleGuid=id DOT)?  eventPackageName=id DOT predicateCompareName=id LR_BRACKET (eventFieldName=id |( (eventModuleGuid=id DOT)?  eventPackageName=id DOT predicateSourceName=id ) COMMA  (DECIMAL | STRING) ) RR_BRACKET
     ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/alter-external-data-source-transact-sql
alterExternalDataSource
    : ALTER EXTERNAL DATA SOURCE dataSourceName=id  SET
    ( LOCATION EQUAL location=(QUOTED_URL|QUOTED_HOST_AND_PORT) COMMA? |  RESOURCE_MANAGER_LOCATION EQUAL resourceManagerLocation=(QUOTED_URL|QUOTED_HOST_AND_PORT) COMMA? |  CREDENTIAL EQUAL credentialName=id )+
    | ALTER EXTERNAL DATA SOURCE dataSourceName=id WITH LR_BRACKET TYPE EQUAL BLOB_STORAGE COMMA LOCATION EQUAL location=STRING (COMMA CREDENTIAL EQUAL credentialName=id )? RR_BRACKET
    ;


// https://docs.microsoft.com/en-us/sql/t-sql/statements/alter-external-library-transact-sql
alterExternalLibrary
    : ALTER EXTERNAL LIBRARY libraryName=id (AUTHORIZATION ownerName=id)?
       (SET|ADD) ( LR_BRACKET CONTENT EQUAL (clientLibrary=STRING | BINARY | NONE) (COMMA PLATFORM EQUAL (WINDOWS|LINUX)? RR_BRACKET) WITH (COMMA? LANGUAGE EQUAL (R|PYTHON) | DATA_SOURCE EQUAL externalDataSourceName=id )+ RR_BRACKET )
   ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/create-external-library-transact-sql
createExternalLibrary
    : CREATE EXTERNAL LIBRARY libraryName=id (AUTHORIZATION ownerName=id)?
       FROM (COMMA? LR_BRACKET?  (CONTENT EQUAL)? (clientLibrary=STRING | BINARY | NONE) (COMMA PLATFORM EQUAL (WINDOWS|LINUX)? RR_BRACKET)? ) ( WITH (COMMA? LANGUAGE EQUAL (R|PYTHON) | DATA_SOURCE EQUAL externalDataSourceName=id )+ RR_BRACKET  )?
   ;


// https://docs.microsoft.com/en-us/sql/t-sql/statements/alter-external-resource-pool-transact-sql
alterExternalResourcePool
    : ALTER EXTERNAL RESOURCE POOL (poolName=id | DEFAULT_DOUBLE_QUOTE) WITH LR_BRACKET MAX_CPU_PERCENT EQUAL maxCpuPercent=DECIMAL ( COMMA? AFFINITY CPU EQUAL (AUTO|(COMMA? DECIMAL TO DECIMAL |COMMA DECIMAL )+ ) | NUMANODE EQUAL (COMMA? DECIMAL TO DECIMAL| COMMA? DECIMAL )+  ) (COMMA? MAX_MEMORY_PERCENT EQUAL maxMemoryPercent=DECIMAL)? (COMMA? MAX_PROCESSES EQUAL maxProcesses=DECIMAL)?  RR_BRACKET
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/create-external-resource-pool-transact-sql
createExternalResourcePool
    : CREATE EXTERNAL RESOURCE POOL poolName=id  WITH LR_BRACKET MAX_CPU_PERCENT EQUAL maxCpuPercent=DECIMAL ( COMMA? AFFINITY CPU EQUAL (AUTO|(COMMA? DECIMAL TO DECIMAL |COMMA DECIMAL )+ ) | NUMANODE EQUAL (COMMA? DECIMAL TO DECIMAL| COMMA? DECIMAL )+  ) (COMMA? MAX_MEMORY_PERCENT EQUAL maxMemoryPercent=DECIMAL)? (COMMA? MAX_PROCESSES EQUAL maxProcesses=DECIMAL)?  RR_BRACKET
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/alter-fulltext-catalog-transact-sql
alterFulltextCatalog
    : ALTER FULLTEXT CATALOG catalogName=id (REBUILD (WITH ACCENT_SENSITIVITY EQUAL (ON|OFF) )? | REORGANIZE | AS DEFAULT )
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/create-fulltext-catalog-transact-sql
createFulltextCatalog
    : CREATE FULLTEXT CATALOG catalogName=id
        (ON FILEGROUP filegroup=id)?
        (IN PATH rootpath=STRING)?
        (WITH ACCENT_SENSITIVITY EQUAL (ON|OFF) )?
        (AS DEFAULT)?
        (AUTHORIZATION ownerName=id)?
    ;



// https://docs.microsoft.com/en-us/sql/t-sql/statements/alter-fulltext-stoplist-transact-sql
alterFulltextStoplist
    : ALTER FULLTEXT STOPLIST stoplistName=id (ADD stopword=STRING LANGUAGE (STRING|DECIMAL|BINARY) | DROP ( stopword=STRING LANGUAGE (STRING|DECIMAL|BINARY) |ALL (STRING|DECIMAL|BINARY) | ALL ) )
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/create-fulltext-stoplist-transact-sql
createFulltextStoplist
    :   CREATE FULLTEXT STOPLIST stoplistName=id
          (FROM ( (databaseName=id DOT)? sourceStoplistName=id |SYSTEM STOPLIST ) )?
          (AUTHORIZATION ownerName=id)?
    ;


// https://docs.microsoft.com/en-us/sql/t-sql/statements/alter-login-transact-sql
alterLoginSqlServer
    : ALTER LOGIN loginName=id
       ( (ENABLE|DISABLE)?  | WITH ( (PASSWORD EQUAL ( password=STRING | passwordHash=BINARY HASHED ) ) (MUST_CHANGE|UNLOCK)* )? (OLD_PASSWORD EQUAL oldPassword=STRING (MUST_CHANGE|UNLOCK)* )? (DEFAULT_DATABASE EQUAL defaultDatabase=id)? (DEFAULT_LANGUAGE EQUAL defaultLaguage=id)?  (NAME EQUAL loginName=id)? (CHECK_POLICY EQUAL (ON|OFF) )? (CHECK_EXPIRATION EQUAL (ON|OFF) )? (CREDENTIAL EQUAL credentialName=id)? (NO CREDENTIAL)? | (ADD|DROP) CREDENTIAL credentialName=id )
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/create-login-transact-sql
createLoginSqlServer
    : CREATE LOGIN loginName=id
       ( WITH ( (PASSWORD EQUAL ( password=STRING | passwordHash=BINARY HASHED ) ) (MUST_CHANGE|UNLOCK)* )?
       (COMMA? SID EQUAL sid=BINARY)?
       (COMMA? DEFAULT_DATABASE EQUAL defaultDatabase=id)?
       (COMMA? DEFAULT_LANGUAGE EQUAL defaultLaguage=id)?
       (COMMA? CHECK_EXPIRATION EQUAL (ON|OFF) )?
       (COMMA? CHECK_POLICY EQUAL (ON|OFF) )?
       (COMMA? CREDENTIAL EQUAL credentialName=id)?
      |(FROM
	(WINDOWS
          (WITH (COMMA? DEFAULT_DATABASE EQUAL defaultDatabase=id)? (COMMA?  DEFAULT_LANGUAGE EQUAL defaultLanguage=STRING)? )
        | CERTIFICATE certname=id
        | ASYMMETRIC KEY asymKeyName=id
                )
        )
      )
    ;

alterLoginAzureSql
    : ALTER LOGIN loginName=id ( (ENABLE|DISABLE)? | WITH (PASSWORD EQUAL password=STRING (OLD_PASSWORD EQUAL oldPassword=STRING)? | NAME EQUAL loginName=id ) )
    ;

createLoginAzureSql
    : CREATE LOGIN loginName=id
       WITH PASSWORD EQUAL STRING (SID EQUAL sid=BINARY)?
    ;

alterLoginAzureSqlDwAndPdw
    : ALTER LOGIN loginName=id ( (ENABLE|DISABLE)? | WITH (PASSWORD EQUAL password=STRING (OLD_PASSWORD EQUAL oldPassword=STRING (MUST_CHANGE|UNLOCK)* )? | NAME EQUAL loginName=id ) )
    ;

createLoginPdw
    : CREATE LOGIN loginName=id
        (WITH
          ( PASSWORD EQUAL password=STRING (MUST_CHANGE)?
              (CHECK_POLICY EQUAL (ON|OFF)? )?
          )
        | FROM WINDOWS
        )
     ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/alter-master-key-transact-sql
alterMasterKeySqlServer
    : ALTER MASTER KEY ( (FORCE)? REGENERATE WITH ENCRYPTION BY PASSWORD EQUAL password=STRING |(ADD|DROP) ENCRYPTION BY (SERVICE MASTER KEY | PASSWORD EQUAL encryptionPassword=STRING) )
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/create-master-key-transact-sql
createMasterKeySqlServer
    : CREATE MASTER KEY ENCRYPTION BY PASSWORD EQUAL password=STRING
    ;

alterMasterKeyAzureSql
    : ALTER MASTER KEY ( (FORCE)? REGENERATE WITH ENCRYPTION BY PASSWORD EQUAL password=STRING |ADD ENCRYPTION BY (SERVICE MASTER KEY | PASSWORD EQUAL encryptionPassword=STRING) | DROP ENCRYPTION BY  PASSWORD EQUAL encryptionPassword=STRING )
    ;

createMasterKeyAzureSql
    : CREATE MASTER KEY (ENCRYPTION BY PASSWORD EQUAL password=STRING)?
    ;


// https://docs.microsoft.com/en-us/sql/t-sql/statements/alter-message-type-transact-sql
alterMessageType
    : ALTER MESSAGE TYPE messageTypeName=id VALIDATION EQUAL (NONE | EMPTY | WELL_FORMED_XML | VALID_XML WITH SCHEMA COLLECTION schemaCollectionName=id)
    ;


// https://docs.microsoft.com/en-us/sql/t-sql/statements/alter-partition-function-transact-sql
alterPartitionFunction
    : ALTER PARTITION FUNCTION partitionFunctionName=id LR_BRACKET RR_BRACKET        (SPLIT|MERGE) RANGE LR_BRACKET DECIMAL RR_BRACKET
    ;


// https://docs.microsoft.com/en-us/sql/t-sql/statements/alter-partition-scheme-transact-sql
alterPartitionScheme
    : ALTER PARTITION SCHEME partitionSchemeName=id NEXT USED (fileGroupName=id)?
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/alter-remote-service-binding-transact-sql
alterRemoteServiceBinding
    : ALTER REMOTE SERVICE BINDING bindingName=id
        WITH (USER EQUAL userName=id)?
             (COMMA ANONYMOUS EQUAL (ON|OFF) )?
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/create-remote-service-binding-transact-sql
createRemoteServiceBinding
    : CREATE REMOTE SERVICE BINDING bindingName=id
         (AUTHORIZATION ownerName=id)?
         TO SERVICE remoteServiceName=STRING
         WITH (USER EQUAL userName=id)?
              (COMMA ANONYMOUS EQUAL (ON|OFF) )?
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/create-resource-pool-transact-sql
createResourcePool
    : CREATE RESOURCE POOL poolName=id
        (WITH
            LR_BRACKET
               (COMMA? MIN_CPU_PERCENT EQUAL DECIMAL)?
               (COMMA? MAX_CPU_PERCENT EQUAL DECIMAL)?
               (COMMA? CAP_CPU_PERCENT EQUAL DECIMAL)?
               (COMMA? AFFINITY SCHEDULER EQUAL
                                  (AUTO
                                   | LR_BRACKET (COMMA? (DECIMAL|DECIMAL TO DECIMAL) )+ RR_BRACKET
                                   | NUMANODE EQUAL LR_BRACKET (COMMA? (DECIMAL|DECIMAL TO DECIMAL) )+ RR_BRACKET
                                   )
               )?
               (COMMA? MIN_MEMORY_PERCENT EQUAL DECIMAL)?
               (COMMA? MAX_MEMORY_PERCENT EQUAL DECIMAL)?
               (COMMA? MIN_IOPS_PER_VOLUME EQUAL DECIMAL)?
               (COMMA? MAX_IOPS_PER_VOLUME EQUAL DECIMAL)?
            RR_BRACKET
         )?
     ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/alter-resource-governor-transact-sql
alterResourceGovernor
    : ALTER RESOURCE GOVERNOR ( (DISABLE | RECONFIGURE) | WITH LR_BRACKET CLASSIFIER_FUNCTION EQUAL ( schemaName=id DOT functionName=id | NULL ) RR_BRACKET | RESET STATISTICS | WITH LR_BRACKET MAX_OUTSTANDING_IO_PER_VOLUME EQUAL maxOutstandingIoPerVolume=DECIMAL RR_BRACKET )
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/alter-role-transact-sql
alterDbRole
    : ALTER ROLE roleName=id
        ( (ADD|DROP) MEMBER databasePrincipal=id
        | WITH NAME EQUAL newRoleName=id )
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/create-role-transact-sql
createDbRole
    : CREATE ROLE roleName=id (AUTHORIZATION ownerName = id)?
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/create-route-transact-sql
createRoute
    : CREATE ROUTE routeName=id
        (AUTHORIZATION ownerName=id)?
        WITH
          (COMMA? SERVICE_NAME EQUAL routeServiceName=STRING)?
          (COMMA? BROKER_INSTANCE EQUAL brokerInstanceIdentifier=STRING)?
          (COMMA? LIFETIME EQUAL DECIMAL)?
          COMMA? ADDRESS EQUAL (STRING|QUOTED_URL)
          (COMMA MIRROR_ADDRESS EQUAL (STRING|QUOTED_URL) )?
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/create-rule-transact-sql
createRule
    : CREATE RULE (schemaName=id DOT)? ruleName=id
        AS searchCondition
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/alter-schema-transact-sql
alterSchemaSql
    : ALTER SCHEMA schemaName=id TRANSFER ((OBJECT|TYPE|XML SCHEMA COLLECTION) COLON COLON )? id (DOT id)?
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/create-schema-transact-sql
createSchema
    : CREATE SCHEMA
	(schemaName=id
        |AUTHORIZATION ownerName=id
        | schemaName=id AUTHORIZATION ownerName=id
        )
        (createTable
         |createView
         | (GRANT|DENY) (SELECT|INSERT|DELETE|UPDATE) ON (SCHEMA COLON COLON)? objectName=id TO ownerName=id
         | REVOKE (SELECT|INSERT|DELETE|UPDATE) ON (SCHEMA COLON COLON)? objectName=id FROM ownerName=id
        )*
    ;

createSchemaAzureSqlDwAndPdw
    :
CREATE SCHEMA schemaName=id (AUTHORIZATION ownerName=id )?
    ;

alterSchemaAzureSqlDwAndPdw
    : ALTER SCHEMA schemaName=id TRANSFER (OBJECT COLON COLON )? id (DOT ID)?
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/create-search-property-list-transact-sql
createSearchPropertyList
    : CREATE SEARCH PROPERTY LIST newListName=id
        (FROM (databaseName=id DOT)? sourceListName=id )?
        (AUTHORIZATION ownerName=id)?
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/create-security-policy-transact-sql
createSecurityPolicy
   : CREATE SECURITY POLICY (schemaName=id DOT)? securityPolicyName=id
        (COMMA? ADD (FILTER|BLOCK)? PREDICATE tvfSchemaName=id DOT securityPredicateFunctionName=id
            LR_BRACKET (COMMA? columnNameOrArguments=id)+ RR_BRACKET
              ON tableSchemaName=id DOT name=id
                (COMMA? AFTER (INSERT|UPDATE)
                | COMMA? BEFORE (UPDATE|DELETE)
                )*
         )+
            (WITH LR_BRACKET
                     STATE EQUAL (ON|OFF)
		     (SCHEMABINDING (ON|OFF) )?
                  RR_BRACKET
             )?
             (NOT FOR REPLICATION)?
     ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/alter-sequence-transact-sql
alterSequence
    : ALTER SEQUENCE (schemaName=id DOT)? sequenceName=id ( RESTART (WITH DECIMAL)? )? (INCREMENT BY sequnceIncrement=DECIMAL )? ( MINVALUE DECIMAL| NO MINVALUE)? (MAXVALUE DECIMAL| NO MAXVALUE)? (CYCLE|NO CYCLE)? (CACHE DECIMAL | NO CACHE)?
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/create-sequence-transact-sql
createSequence
    : CREATE SEQUENCE (schemaName=id DOT)? sequenceName=id
        (AS dataType  )?
        (START WITH DECIMAL)?
        (INCREMENT BY MINUS? DECIMAL)?
        (MINVALUE DECIMAL? | NO MINVALUE)?
        (MAXVALUE DECIMAL? | NO MAXVALUE)?
        (CYCLE|NO CYCLE)?
        (CACHE DECIMAL? | NO CACHE)?
     ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/alter-server-audit-transact-sql
alterServerAudit
    : ALTER SERVER AUDIT auditName=id
        ( ( TO
              (FILE
                ( LR_BRACKET
                   ( COMMA? FILEPATH EQUAL filepath=STRING
                    | COMMA? MAXSIZE EQUAL ( DECIMAL (MB|GB|TB)
                    |  UNLIMITED
                   )
                   | COMMA? MAX_ROLLOVER_FILES EQUAL maxRolloverFiles=(DECIMAL|UNLIMITED)
                   | COMMA? MAX_FILES EQUAL maxFiles=DECIMAL
                   | COMMA? RESERVE_DISK_SPACE EQUAL (ON|OFF)  )*
                 RR_BRACKET )
                | APPLICATION_LOG
                | SECURITY_LOG
            ) )?
            ( WITH LR_BRACKET
              (COMMA? QUEUE_DELAY EQUAL queueDelay=DECIMAL
              | COMMA? ON_FAILURE EQUAL (CONTINUE | SHUTDOWN|FAIL_OPERATION)
              |COMMA?  STATE EQUAL (ON|OFF) )*
              RR_BRACKET
            )?
            ( WHERE ( COMMA? (NOT?) eventFieldName=id
                                    (EQUAL
                                    |(LESS GREATER)
                                    | (EXCLAMATION EQUAL)
                                    | GREATER
                                    | (GREATER EQUAL)
                                    | LESS
                                    | LESS EQUAL
                                    )
                                      (DECIMAL | STRING)
                    | COMMA? (AND|OR) NOT? (EQUAL
                                           |(LESS GREATER)
                                           | (EXCLAMATION EQUAL)
                                           | GREATER
                                           | (GREATER EQUAL)
                                           | LESS
                                           | LESS EQUAL)
                                             (DECIMAL | STRING) ) )?
        |REMOVE WHERE
        | MODIFY NAME EQUAL newAuditName=id
       )
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/create-server-audit-transact-sql
createServerAudit
    : CREATE SERVER AUDIT auditName=id
        ( ( TO
              (FILE
                ( LR_BRACKET
                   ( COMMA? FILEPATH EQUAL filepath=STRING
                    | COMMA? MAXSIZE EQUAL ( DECIMAL (MB|GB|TB)
                    |  UNLIMITED
                   )
                   | COMMA? MAX_ROLLOVER_FILES EQUAL maxRolloverFiles=(DECIMAL|UNLIMITED)
                   | COMMA? MAX_FILES EQUAL maxFiles=DECIMAL
                   | COMMA? RESERVE_DISK_SPACE EQUAL (ON|OFF)  )*
                 RR_BRACKET )
                | APPLICATION_LOG
                | SECURITY_LOG
            ) )?
            ( WITH LR_BRACKET
              (COMMA? QUEUE_DELAY EQUAL queueDelay=DECIMAL
              | COMMA? ON_FAILURE EQUAL (CONTINUE | SHUTDOWN|FAIL_OPERATION)
              |COMMA?  STATE EQUAL (ON|OFF)
              |COMMA? AUDIT_GUID EQUAL auditGuid=id
            )*

              RR_BRACKET
            )?
            ( WHERE ( COMMA? (NOT?) eventFieldName=id
                                    (EQUAL
                                    |(LESS GREATER)
                                    | (EXCLAMATION EQUAL)
                                    | GREATER
                                    | (GREATER EQUAL)
                                    | LESS
                                    | LESS EQUAL
                                    )
                                      (DECIMAL | STRING)
                    | COMMA? (AND|OR) NOT? (EQUAL
                                           |(LESS GREATER)
                                           | (EXCLAMATION EQUAL)
                                           | GREATER
                                           | (GREATER EQUAL)
                                           | LESS
                                           | LESS EQUAL)
                                             (DECIMAL | STRING) ) )?
        |REMOVE WHERE
        | MODIFY NAME EQUAL newAuditName=id
       )
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/alter-server-audit-specification-transact-sql

alterServerAuditSpecification
    : ALTER SERVER AUDIT SPECIFICATION auditSpecificationName=id
       (FOR SERVER AUDIT auditName=id)?
       ( (ADD|DROP) LR_BRACKET  auditActionGroupName=id RR_BRACKET )*
         (WITH LR_BRACKET STATE EQUAL (ON|OFF) RR_BRACKET )?
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/create-server-audit-specification-transact-sql
createServerAuditSpecification
    : CREATE SERVER AUDIT SPECIFICATION auditSpecificationName=id
       (FOR SERVER AUDIT auditName=id)?
       ( ADD LR_BRACKET  auditActionGroupName=id RR_BRACKET )*
         (WITH LR_BRACKET STATE EQUAL (ON|OFF) RR_BRACKET )?
    ;


// https://docs.microsoft.com/en-us/sql/t-sql/statements/alter-server-configuration-transact-sql

alterServerConfiguration
    : ALTER SERVER CONFIGURATION
      SET  ( (PROCESS AFFINITY (CPU EQUAL (AUTO | (COMMA? DECIMAL | COMMA? DECIMAL TO DECIMAL)+ ) | NUMANODE EQUAL ( COMMA? DECIMAL |COMMA?  DECIMAL TO DECIMAL)+ ) | DIAGNOSTICS LOG (ON|OFF|PATH EQUAL (STRING | DEFAULT) |MAX_SIZE EQUAL (DECIMAL MB |DEFAULT)|MAX_FILES EQUAL (DECIMAL|DEFAULT) ) | FAILOVER CLUSTER PROPERTY (VERBOSELOGGING EQUAL (STRING|DEFAULT) |SQLDUMPERFLAGS EQUAL (STRING|DEFAULT) | SQLDUMPERPATH EQUAL (STRING|DEFAULT) | SQLDUMPERTIMEOUT (STRING|DEFAULT) | FAILURECONDITIONLEVEL EQUAL (STRING|DEFAULT) | HEALTHCHECKTIMEOUT EQUAL (DECIMAL|DEFAULT) ) | HADR CLUSTER CONTEXT EQUAL (STRING|LOCAL) | BUFFER POOL EXTENSION (ON LR_BRACKET FILENAME EQUAL STRING COMMA SIZE EQUAL DECIMAL (KB|MB|GB)  RR_BRACKET | OFF ) | SET SOFTNUMA (ON|OFF) ) )
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/alter-server-role-transact-sql
alterServerRole
    : ALTER SERVER ROLE serverRoleName=id
      ( (ADD|DROP) MEMBER serverPrincipal=id
      | WITH NAME EQUAL newServerRoleName=id
      )
    ;
// https://docs.microsoft.com/en-us/sql/t-sql/statements/create-server-role-transact-sql
createServerRole
    : CREATE SERVER ROLE serverRole=id (AUTHORIZATION serverPrincipal=id)?
    ;

alterServerRolePdw
    : ALTER SERVER ROLE serverRoleName=id (ADD|DROP) MEMBER login=id
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/alter-service-transact-sql
alterService
    : ALTER SERVICE modifiedServiceName=id (ON QUEUE (schemaName=id DOT) queueName=id)? (COMMA? (ADD|DROP) modifiedContractName=id)*
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/create-service-transact-sql
createService
    : CREATE SERVICE createServiceName=id
        (AUTHORIZATION ownerName=id)?
        ON QUEUE (schemaName=id DOT)? queueName=id
          ( LR_BRACKET (COMMA? (id|DEFAULT) )+ RR_BRACKET )?
    ;


// https://docs.microsoft.com/en-us/sql/t-sql/statements/alter-service-master-key-transact-sql

alterServiceMasterKey
    : ALTER SERVICE MASTER KEY ( FORCE? REGENERATE | (WITH (OLD_ACCOUNT EQUAL acoldAccountName=STRING COMMA OLD_PASSWORD EQUAL oldPassword=STRING | NEW_ACCOUNT EQUAL newAccountName=STRING COMMA NEW_PASSWORD EQUAL newPassword=STRING)?  ) )
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/alter-symmetric-key-transact-sql

alterSymmetricKey
    : ALTER SYMMETRIC KEY keyName=id ( (ADD|DROP) ENCRYPTION BY (CERTIFICATE certificateName=id | PASSWORD EQUAL password=STRING | SYMMETRIC KEY symmetricKeyName=id | ASYMMETRIC KEY AsymKeyName=id  ) )
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/create-symmetric-key-transact-sql
createSymmetricKey
    :  ALTER SYMMETRIC KEY keyName=id
           (AUTHORIZATION ownerName=id)?
           (FROM PROVIDER providerName=id)?
           (WITH ( (KEY_SOURCE EQUAL keyPassPhrase=STRING
                   | ALGORITHM EQUAL (DES | TRIPLE_DES | TRIPLE_DES_3KEY | RC2 | RC4 | RC4_128  | DESX | AES_128 | AES_192 | AES_256)
                   | IDENTITY_VALUE EQUAL identityPhrase=STRING
                   | PROVIDER_KEY_NAME EQUAL providerKeyName=STRING
                   | CREATION_DISPOSITION EQUAL (CREATE_NEW|OPEN_EXISTING)
                   )
                 | ENCRYPTION BY
                     ( CERTIFICATE certificateName=id
                     | PASSWORD EQUAL password=STRING
                     | SYMMETRIC KEY symmetricKeyName=id
                     | ASYMMETRIC KEY asymKeyName=id
                     )
                 )
            )
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/create-synonym-transact-sql
createSynonym
    : CREATE SYNONYM (schemaName_1=id DOT )? synonymName=id
        FOR ( (serverName=id DOT )? (databaseName=id DOT)? (schemaName_2=id DOT)? objectName=id
            | (databaseOrSchema2=id DOT)? (schemaId_2OrObjectName=id DOT)?
            )
    ;


// https://docs.microsoft.com/en-us/sql/t-sql/statements/alter-user-transact-sql
alterUser
    : ALTER USER username=id WITH (COMMA? NAME EQUAL newusername=id | COMMA? DEFAULT_SCHEMA EQUAL ( schemaName=id |NULL ) | COMMA? LOGIN EQUAL loginame=id | COMMA? PASSWORD EQUAL STRING (OLD_PASSWORD EQUAL STRING)+ | COMMA? DEFAULT_LANGUAGE EQUAL (NONE| lcid=DECIMAL| languageNameOrAlias=id) | COMMA? ALLOW_ENCRYPTED_VALUE_MODIFICATIONS EQUAL (ON|OFF) )+
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/create-user-transact-sql
createUser
    : CREATE USER userName=id
         (  (FOR|FROM) LOGIN loginName=id )?
         ( WITH (COMMA? DEFAULT_SCHEMA EQUAL schemaName=id
                |COMMA? ALLOW_ENCRYPTED_VALUE_MODIFICATIONS EQUAL (ON|OFF)
                )*
         )?
    | CREATE USER   ( windowsPrincipal=id
                      (WITH
                        (COMMA? DEFAULT_SCHEMA EQUAL schemaName=id
                        |COMMA? DEFAULT_LANGUAGE EQUAL (NONE
                                                |DECIMAL
                                                |languageNameOrAlias=id                                                      )
                        |COMMA? SID EQUAL BINARY
                        |COMMA? ALLOW_ENCRYPTED_VALUE_MODIFICATIONS EQUAL (ON|OFF)
                        )*
                      )?
                   | userName=id WITH PASSWORD EQUAL password=STRING
                            (COMMA? DEFAULT_SCHEMA EQUAL schemaName=id
                            |COMMA? DEFAULT_LANGUAGE EQUAL (NONE
                                                |DECIMAL
                                                |languageNameOrAlias=id                                                      )
                            |COMMA? SID EQUAL BINARY
                           |COMMA? ALLOW_ENCRYPTED_VALUE_MODIFICATIONS EQUAL (ON|OFF)
                          )*
                   | Azure_Active_DirectoryPrincipal=id FROM EXTERNAL PROVIDER
                   )
     | CREATE USER userName=id
                 ( WITHOUT LOGIN
                   (COMMA? DEFAULT_SCHEMA EQUAL schemaName=id
                   |COMMA? ALLOW_ENCRYPTED_VALUE_MODIFICATIONS EQUAL (ON|OFF)
                   )*
                 | (FOR|FROM) CERTIFICATE certName=id
                 | (FOR|FROM) ASYMMETRIC KEY asymKeyName=id
                 )
     | CREATE USER userName=id
     ;

createUserAzureSqlDw
    : CREATE USER userName=id
        ( (FOR|FROM) LOGIN loginName=id
        | WITHOUT LOGIN
        )?

        ( WITH DEFAULT_SCHEMA EQUAL schemaName=id)?
    | CREATE USER Azure_Active_DirectoryPrincipal=id
        FROM EXTERNAL PROVIDER
        ( WITH DEFAULT_SCHEMA EQUAL schemaName=id)?
    ;


alterUserAzureSql
    : ALTER USER username=id WITH (COMMA? NAME EQUAL newusername=id | COMMA? DEFAULT_SCHEMA EQUAL  schemaName=id | COMMA? LOGIN EQUAL loginame=id  | COMMA? ALLOW_ENCRYPTED_VALUE_MODIFICATIONS EQUAL (ON|OFF) )+
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/alter-workload-group-transact-sql

alterWorkloadGroup
    : ALTER WORKLOAD GROUP
         (workloadGroupGroupName=id
         |DEFAULT_DOUBLE_QUOTE
         )
         (WITH LR_BRACKET
           (IMPORTANCE EQUAL (LOW|MEDIUM|HIGH)
           | COMMA? REQUEST_MAX_MEMORY_GRANT_PERCENT EQUAL requestMaxMemoryGrant=DECIMAL
           | COMMA? REQUEST_MAX_CPU_TIME_SEC EQUAL requestMaxCpuTimeSec=DECIMAL
           | REQUEST_MEMORY_GRANT_TIMEOUT_SEC EQUAL requestMemoryGrantTimeoutSec=DECIMAL
           | MAX_DOP EQUAL maxDop=DECIMAL
           | GROUP_MAX_REQUESTS EQUAL groupMaxRequests=DECIMAL)+
          RR_BRACKET )?
     (USING (workloadGroupPoolName=id | DEFAULT_DOUBLE_QUOTE) )?
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/create-workload-group-transact-sql
createWorkloadGroup
    : CREATE WORKLOAD GROUP workloadGroupGroupName=id
         (WITH LR_BRACKET
           (IMPORTANCE EQUAL (LOW|MEDIUM|HIGH)
           | COMMA? REQUEST_MAX_MEMORY_GRANT_PERCENT EQUAL requestMaxMemoryGrant=DECIMAL
           | COMMA? REQUEST_MAX_CPU_TIME_SEC EQUAL requestMaxCpuTimeSec=DECIMAL
           | REQUEST_MEMORY_GRANT_TIMEOUT_SEC EQUAL requestMemoryGrantTimeoutSec=DECIMAL
           | MAX_DOP EQUAL maxDop=DECIMAL
           | GROUP_MAX_REQUESTS EQUAL groupMaxRequests=DECIMAL)+
          RR_BRACKET )?
     (USING (workloadGroupPoolName=id | DEFAULT_DOUBLE_QUOTE)?
            (COMMA? EXTERNAL externalPoolName=id | DEFAULT_DOUBLE_QUOTE)?
      )?
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/create-xml-schema-collection-transact-sql
createXmlSchemaCollection
    : CREATE XML SCHEMA COLLECTION (relationalSchema=id DOT)? sqlIdentifier=id AS  (STRING|id|LOCAL_ID)
    ;

createQueue
    : CREATE QUEUE (fullTableName | queueName=id) queueSettings?
      (ON filegroup=id | DEFAULT)?
    ;


queueSettings
    :WITH
       (STATUS EQUAL (ON | OFF) COMMA?)?
       (RETENTION EQUAL (ON | OFF) COMMA?)?
       (ACTIVATION
         LR_BRACKET
           (
             (
              (STATUS EQUAL (ON | OFF) COMMA? )?
              (PROCEDURE_NAME EQUAL funcProcNameDatabaseSchema COMMA?)?
              (MAX_QUEUE_READERS EQUAL maxReaders=DECIMAL COMMA?)?
              (EXECUTE AS (SELF | userName=STRING | OWNER) COMMA?)?
             )
             | DROP
           )
         RR_BRACKET COMMA?
       )?
       (POISON_MESSAGE_HANDLING
         LR_BRACKET
           (STATUS EQUAL (ON | OFF))
         RR_BRACKET
       )?
    ;

alterQueue
    : ALTER QUEUE (fullTableName | queueName=id)
      (queueSettings | queueAction)
    ;

queueAction
    : REBUILD ( WITH LR_BRACKET queueRebuildOptions RR_BRACKET)?
    | REORGANIZE (WITH LOB_COMPACTION EQUAL (ON | OFF))?
    | MOVE TO (id | DEFAULT)
    ;
queueRebuildOptions
    : MAXDOP EQUAL DECIMAL
    ;

createContract
    : CREATE CONTRACT contractName
      (AUTHORIZATION ownerName=id)?
      LR_BRACKET ((messageTypeName=id | DEFAULT)
          SENT BY (INITIATOR | TARGET | ANY ) COMMA?)+
      RR_BRACKET
    ;

conversationStatement
    : beginConversationTimer
    | beginConversationDialog
    | endConversation
    | getConversation
    | sendConversation
    | waitforConversation
    ;

messageStatement
    : CREATE MESSAGE TYPE messageTypeName=id
      (AUTHORIZATION ownerName=id)?
      (VALIDATION EQUAL (NONE
      | EMPTY
      | WELL_FORMED_XML
      | VALID_XML WITH SCHEMA COLLECTION schemaCollectionName=id))
    ;

// DML

// https://docs.microsoft.com/en-us/sql/t-sql/statements/merge-transact-sql
mergeStatement
    : withExpression?
      MERGE (TOP '(' expression ')' PERCENT?)?
      INTO? ddlObject insertWithTableHints? asTableAlias?
      USING tableSources
      ON searchCondition
      (WHEN MATCHED (AND searchCondition)?
          THEN mergeMatched)*
      (WHEN NOT MATCHED (BY TARGET)? (AND searchCondition)?
          THEN mergeNotMatched)?
      (WHEN NOT MATCHED BY SOURCE (AND searchCondition)?
          THEN mergeMatched)*
      outputClause?
      optionClause? ';'
    ;


mergeMatched
    : UPDATE SET updateElem (',' updateElem)*
    | DELETE
    ;

mergeNotMatched
    : INSERT ('(' columnNameList ')')?
      (tableValueConstructor | DEFAULT VALUES)
    ;

// https://msdn.microsoft.com/en-us/library/ms189835.aspx
deleteStatement
    : withExpression?
      DELETE (TOP '(' expression ')' PERCENT? | TOP DECIMAL)?
      FROM? deleteStatementFrom
      insertWithTableHints?
      outputClause?
      (FROM tableSources)?
      (WHERE (searchCondition | CURRENT OF (GLOBAL? cursorName | cursorVar=LOCAL_ID)))?
      forClause? optionClause? ';'?
    ;

deleteStatementFrom
    : ddlObject
    | tableAlias
    | rowsetFunctionLimited
    | tableVar=LOCAL_ID
    ;

// https://msdn.microsoft.com/en-us/library/ms174335.aspx
insertStatement
    : withExpression?
      INSERT (TOP '(' expression ')' PERCENT?)?
      INTO? (ddlObject | rowsetFunctionLimited)
      insertWithTableHints?
      ('(' columnNameList ')')?
      outputClause?
      insertStatementValue
      forClause? optionClause? ';'?
    ;

insertStatementValue
    : tableValueConstructor
    | derivedTable
    | executeStatement
    | DEFAULT VALUES
    ;


receiveStatement
    : '('? RECEIVE (ALL | DISTINCT | topClause | '*')
      (LOCAL_ID '=' expression ','?)* FROM fullTableName
      (INTO tableVariable=id (WHERE where_=searchCondition))? ')'?
    ;

// https://msdn.microsoft.com/en-us/library/ms189499.aspx
selectStatement
    : withExpression? queryExpression orderByClause? forClause? optionClause? ';'?
    ;

time
    : (LOCAL_ID | constant)
    ;

// https://msdn.microsoft.com/en-us/library/ms177523.aspx
updateStatement
    : withExpression?
      UPDATE (TOP '(' expression ')' PERCENT?)?
      (ddlObject | rowsetFunctionLimited)
      withTableHints?
      SET updateElem (',' updateElem)*
      outputClause?
      (FROM tableSources)?
      (WHERE (searchConditionList | CURRENT OF (GLOBAL? cursorName | cursorVar=LOCAL_ID)))?
      forClause? optionClause? ';'?
    ;

// https://msdn.microsoft.com/en-us/library/ms177564.aspx
outputClause
    : OUTPUT outputDmlListElem (',' outputDmlListElem)*
      (INTO (LOCAL_ID | tableName) ('(' columnNameList ')')? )?
    ;

outputDmlListElem
    : (outputColumnName | expression) asColumnAlias?  // TODO: scalarExpression
    ;

outputColumnName
    : (DELETED | INSERTED | tableName) '.' ('*' | id)
    | DOLLAR_ACTION
    ;

// DDL

// https://msdn.microsoft.com/en-ie/library/ms176061.aspx
createDatabase
    : CREATE DATABASE (database=id)
    ( CONTAINMENT '=' ( NONE | PARTIAL ) )?
    ( ON PRIMARY? databaseFileSpec ( ',' databaseFileSpec )* )?
    ( LOG ON databaseFileSpec ( ',' databaseFileSpec )* )?
    ( COLLATE collationName = id )?
    ( WITH  createDatabaseOption ( ',' createDatabaseOption )* )?
    ;

// https://msdn.microsoft.com/en-us/library/ms188783.aspx
createIndex
    : CREATE UNIQUE? clustered? INDEX id ON tableNameWithHint '(' columnNameListWithOrder ')'
    (INCLUDE '(' columnNameList ')' )?
    (WHERE where_=searchCondition)?
    (indexOptions)?
    (ON id)?
    ';'?
    ;

// https://msdn.microsoft.com/en-us/library/ms187926(v=sql.120).aspx
createOrAlterProcedure
    : ((CREATE (OR ALTER)?) | ALTER) proc_=(PROC | PROCEDURE) funcProcNameSchema (';' DECIMAL)?
      ('('? procedureParam (',' procedureParam)* ')'?)?
      (WITH procedureOption (',' procedureOption)*)?
      (FOR REPLICATION)? AS sqlClauses
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/create-trigger-transact-sql
createOrAlterTrigger
    : createOrAlterDmlTrigger
    | createOrAlterDdlTrigger
    ;

createOrAlterDmlTrigger
    : ((CREATE (OR ALTER)?) | ALTER) TRIGGER simpleName
      ON tableName
      (WITH dmlTriggerOption (',' dmlTriggerOption)* )?
      (FOR | AFTER | INSTEAD OF)
      dmlTriggerOperation (',' dmlTriggerOperation)*
      (WITH APPEND)?
      (NOT FOR REPLICATION)?
      AS sqlClauses
    ;

dmlTriggerOption
    : ENCRYPTION
    | executeClause
    ;

dmlTriggerOperation
    : (INSERT | UPDATE | DELETE)
    ;

createOrAlterDdlTrigger
    : ((CREATE (OR ALTER)?) | ALTER) TRIGGER simpleId
      ON (ALL SERVER | DATABASE)
      (WITH dmlTriggerOption (',' dmlTriggerOption)* )?
      (FOR | AFTER) ddlTriggerOperation (',' dmlTriggerOperation)*
      AS sqlClauses
    ;

ddlTriggerOperation
    : simpleId
    ;

// https://msdn.microsoft.com/en-us/library/ms186755.aspx
createOrAlterFunction
    : ((CREATE (OR ALTER)?) | ALTER) FUNCTION funcProcNameSchema
        (('(' procedureParam (',' procedureParam)* ')') | '(' ')') //must have (), but can be empty
        (funcBodyReturnsSelect | funcBodyReturnsTable | funcBodyReturnsScalar) ';'?
    ;

funcBodyReturnsSelect
  :RETURNS TABLE
  (WITH functionOption (',' functionOption)*)?
  AS?
  RETURN ('(' selectStatement ')' | selectStatement)
  ;

funcBodyReturnsTable
  : RETURNS LOCAL_ID tableTypeDefinition
        (WITH functionOption (',' functionOption)*)?
        AS?
        BEGIN
           sqlClause*
           RETURN ';'?
        END ';'?
  ;

funcBodyReturnsScalar
  :RETURNS dataType
       (WITH functionOption (',' functionOption)*)?
       AS?
       BEGIN
           sqlClause*
           RETURN ret=expression ';'?
       END
       ;

procedureParam
    : LOCAL_ID (id '.')? AS? dataType VARYING? ('=' defaultVal=defaultValue)? (OUT | OUTPUT | READONLY)?
    ;

procedureOption
    : ENCRYPTION
    | RECOMPILE
    | executeClause
    ;

functionOption
    : ENCRYPTION
    | SCHEMABINDING
    | RETURNS NULL ON NULL INPUT
    | CALLED ON NULL INPUT
    | executeClause
    ;

// https://msdn.microsoft.com/en-us/library/ms188038.aspx
createStatistics
    : CREATE STATISTICS id ON tableNameWithHint '(' columnNameList ')'
      (WITH (FULLSCAN | SAMPLE DECIMAL (PERCENT | ROWS) | STATS_STREAM)
            (',' NORECOMPUTE)? (',' INCREMENTAL EQUAL onOff)? )? ';'?
    ;

updateStatistics
    : UPDATE (INDEX | ALL)? STATISTICS fullTableName id?  (USING DECIMAL VALUES)?
    ;

// https://msdn.microsoft.com/en-us/library/ms174979.aspx
createTable
    : CREATE TABLE tableName '(' columnDefTableConstraints ','? ')' (LOCK simpleId)? tableOptions* (ON id | DEFAULT)? (TEXTIMAGE_ON id | DEFAULT)?';'?
    ;

tableOptions
    : WITH ('(' indexOption (',' indexOption)* ')' | indexOption (',' indexOption)*)
    ;

// https://msdn.microsoft.com/en-us/library/ms187956.aspx
createView
    : CREATE VIEW simpleName ('(' columnNameList ')')?
      (WITH viewAttribute (',' viewAttribute)*)?
      AS selectStatement (WITH CHECK OPTION)? ';'?
    ;

viewAttribute
    : ENCRYPTION | SCHEMABINDING | VIEW_METADATA
    ;

// https://msdn.microsoft.com/en-us/library/ms190273.aspx
alterTable
    : ALTER TABLE tableName (SET '(' LOCK_ESCALATION '=' (AUTO | TABLE | DISABLE) ')'
                             | ADD columnDefTableConstraint
                             | ALTER COLUMN columnDefinition
                             | DROP COLUMN id
                             | DROP CONSTRAINT constraint=id
                             | WITH CHECK ADD CONSTRAINT constraint=id FOREIGN KEY '(' fk = columnNameList ')' REFERENCES tableName '(' pk = columnNameList')'
                             | CHECK CONSTRAINT constraint=id
                             | (ENABLE | DISABLE) TRIGGER id?
                             | REBUILD tableOptions)
                             ';'?
    ;

// https://msdn.microsoft.com/en-us/library/ms174269.aspx
alterDatabase
    : ALTER DATABASE (database=id | CURRENT)
      (MODIFY NAME '=' newName=id | COLLATE collation=id | SET databaseOptionspec (WITH termination)? ) ';'?
    ;

// https://msdn.microsoft.com/en-us/library/bb522682.aspx
// Runtime check.
databaseOptionspec
    :  autoOption
      | changeTrackingOption
      | containmentOption
      | cursorOption
      | databaseMirroringOption
      | dateCorrelationOptimizationOption
      | dbEncryptionOption
      | dbStateOption
      | dbUpdateOption
      | dbUserAccessOption
      | delayedDurabilityOption
      | externalAccessOption
      | FILESTREAM databaseFilestreamOption
      | hadrOptions
      | mixedPageAllocationOption
      | parameterizationOption
//      | queryStoreOptions
      | recoveryOption
//      | remoteDataArchiveOption
      | serviceBrokerOption
      | snapshotOption
      | sqlOption
      | targetRecoveryTimeOption
      | termination
    ;

autoOption:
     AUTO_CLOSE onOff
      | AUTO_CREATE_STATISTICS  OFF | ON ( INCREMENTAL EQUAL  ON | OFF  )
      | AUTO_SHRINK  onOff
      | AUTO_UPDATE_STATISTICS onOff
      | AUTO_UPDATE_STATISTICS_ASYNC  (ON | OFF )
    ;

changeTrackingOption:
    CHANGE_TRACKING  EQUAL ( OFF | ON (changeTrackingOptionList (',' changeTrackingOptionList)*)*  )
    ;

changeTrackingOptionList:
     AUTO_CLEANUP EQUAL onOff
     | CHANGE_RETENTION EQUAL ( DAYS | HOURS | MINUTES )
    ;

containmentOption:
     CONTAINMENT EQUAL ( NONE | PARTIAL )
    ;

cursorOption:
    CURSOR_CLOSE_ON_COMMIT onOff
    | CURSOR_DEFAULT ( LOCAL | GLOBAL )
  ;



// https://docs.microsoft.com/en-us/sql/t-sql/statements/alter-endpoint-transact-sql
alterEndpoint
   : ALTER ENDPOINT endpointname=id (AUTHORIZATION login=id)?
       ( STATE EQUAL ( state=STARTED | state=STOPPED | state=DISABLED ) )?
            AS TCP LR_BRACKET
               LISTENER_PORT EQUAL port=DECIMAL
                 ( COMMA LISTENER_IP EQUAL
                   (ALL | IPV4_ADDR | IPV6_ADDR) )?
                RR_BRACKET
               (TSQL
               |
                FOR SERVICE_BROKER LR_BRACKET
                   AUTHENTICATION EQUAL
                           ( WINDOWS ( NTLM |KERBEROS | NEGOTIATE )?  (CERTIFICATE certName=id)?
                           | CERTIFICATE certName=id  WINDOWS? ( NTLM |KERBEROS | NEGOTIATE )?
                           )
                   ( COMMA? ENCRYPTION EQUAL ( DISABLED |SUPPORTED | REQUIRED )
                      ( ALGORITHM ( AES | RC4 | AES RC4 | RC4 AES ) )?
                   )?

                   ( COMMA? MESSAGE_FORWARDING EQUAL ( ENABLED | DISABLED ) )?
                   ( COMMA? MESSAGE_FORWARD_SIZE EQUAL DECIMAL)?
                   RR_BRACKET
              |
               FOR DATABASE_MIRRORING LR_BRACKET
                   AUTHENTICATION EQUAL
                           ( WINDOWS ( NTLM |KERBEROS | NEGOTIATE )?  (CERTIFICATE certName=id)?
                           | CERTIFICATE certName=id  WINDOWS? ( NTLM |KERBEROS | NEGOTIATE )?
                           )

                   ( COMMA? ENCRYPTION EQUAL ( DISABLED |SUPPORTED | REQUIRED )
                      ( ALGORITHM ( AES | RC4 | AES RC4 | RC4 AES ) )?
                   )?

                   COMMA? ROLE EQUAL ( WITNESS | PARTNER | ALL )
                   RR_BRACKET
             )
   ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/create-endpoint-transact-sql
// todo: not implemented

/* Will visit later
*/
databaseMirroringOption
   : mirroringSetOption
   ;

mirroringSetOption
   : mirroringPartner  partnerOption
   | mirroringWitness  witnessOption
   ;
mirroringPartner
   : PARTNER
   ;

mirroringWitness
   : WITNESS
   ;

witnessPartnerEqual
   : EQUAL
   ;


partnerOption
   : witnessPartnerEqual partnerServer
   | FAILOVER
   | FORCE_SERVICE_ALLOW_DATA_LOSS
   | OFF
   | RESUME
   | SAFETY (FULL | OFF )
   | SUSPEND
   | TIMEOUT DECIMAL
;

witnessOption
   : witnessPartnerEqual witnessServer
   | OFF
;

witnessServer
   : partnerServer
   ;

partnerServer
   :
   partnerServerTcpPrefix host mirroringHostPortSeperator portNumber
   ;

mirroringHostPortSeperator
   : COLON
   ;

partnerServerTcpPrefix
   : TCP COLON DOUBLE_FORWARD_SLASH
   ;
portNumber
   :
   port=DECIMAL
   ;

host
   : id DOT host
   | (id DOT |id)
   ;

dateCorrelationOptimizationOption:
    DATE_CORRELATION_OPTIMIZATION onOff
    ;

dbEncryptionOption:
     ENCRYPTION onOff
    ;
dbStateOption:
     ( ONLINE | OFFLINE | EMERGENCY )
    ;

dbUpdateOption:
    READ_ONLY | READ_WRITE
    ;

dbUserAccessOption:
    ( SINGLE_USER | RESTRICTED_USER | MULTI_USER )
    ;
delayedDurabilityOption:
     DELAYED_DURABILITY EQUAL ( DISABLED | ALLOWED | FORCED )
    ;

externalAccessOption:
   DB_CHAINING onOff
  | TRUSTWORTHY onOff
  | DEFAULT_LANGUAGE EQUAL ( id | STRING )
  | DEFAULT_FULLTEXT_LANGUAGE EQUAL ( id | STRING )
  | NESTED_TRIGGERS EQUAL ( OFF | ON )
  | TRANSFORM_NOISE_WORDS EQUAL ( OFF | ON )
  | TWO_DIGIT_YEAR_CUTOFF EQUAL DECIMAL
  ;

hadrOptions
   : HADR
      ( ( AVAILABILITY GROUP EQUAL availabilityGroupName=id | OFF ) |(SUSPEND|RESUME) )
    ;

mixedPageAllocationOption:
     MIXED_PAGE_ALLOCATION ( OFF | ON )
    ;

parameterizationOption:
     PARAMETERIZATION ( SIMPLE | FORCED )
    ;

recoveryOption:
     RECOVERY ( FULL | BULK_LOGGED | SIMPLE )
     | TORN_PAGE_DETECTION onOff
     | PAGE_VERIFY ( CHECKSUM | TORN_PAGE_DETECTION | NONE )
    ;

serviceBrokerOption:
    ENABLE_BROKER
    | DISABLE_BROKER
    | NEW_BROKER
    | ERROR_BROKER_CONVERSATIONS
    | HONOR_BROKER_PRIORITY onOff
  ;
snapshotOption:
   ALLOW_SNAPSHOT_ISOLATION onOff
  | READ_COMMITTED_SNAPSHOT (ON | OFF )
  | MEMORY_OPTIMIZED_ELEVATE_TO_SNAPSHOT = (ON | OFF )
  ;

sqlOption:
  ANSI_NULL_DEFAULT onOff
  | ANSI_NULLS onOff
  | ANSI_PADDING onOff
  | ANSI_WARNINGS onOff
  | ARITHABORT onOff
  | COMPATIBILITY_LEVEL EQUAL DECIMAL
  | CONCAT_NULL_YIELDS_NULL onOff
  | NUMERIC_ROUNDABORT onOff
  | QUOTED_IDENTIFIER onOff
  | RECURSIVE_TRIGGERS onOff
  ;

targetRecoveryTimeOption:
     TARGET_RECOVERY_TIME EQUAL DECIMAL ( SECONDS | MINUTES )
    ;

termination:
    ROLLBACK AFTER seconds = DECIMAL
    | ROLLBACK IMMEDIATE
    | NO_WAIT
    ;

// https://msdn.microsoft.com/en-us/library/ms176118.aspx
dropIndex
    : DROP INDEX (IF EXISTS)?
    ( dropRelationalOrXmlOrSpatialIndex (',' dropRelationalOrXmlOrSpatialIndex)*
    | dropBackwardCompatibleIndex (',' dropBackwardCompatibleIndex)*
    )
    ';'?
    ;

dropRelationalOrXmlOrSpatialIndex
    : indexName=id ON fullTableName
    ;

dropBackwardCompatibleIndex
    : (ownerName=id '.')? tableOrViewName=id '.' indexName=id
    ;

// https://msdn.microsoft.com/en-us/library/ms174969.aspx
dropProcedure
    : DROP proc_=(PROC | PROCEDURE) (IF EXISTS)? funcProcNameSchema (',' funcProcNameSchema)* ';'?
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/drop-trigger-transact-sql
dropTrigger
    : dropDmlTrigger
    | dropDdlTrigger
    ;

dropDmlTrigger
    : DROP TRIGGER (IF EXISTS)? simpleName (',' simpleName)* ';'?
    ;

dropDdlTrigger
    : DROP TRIGGER (IF EXISTS)? simpleName (',' simpleName)*
    ON (DATABASE | ALL SERVER) ';'?
    ;

// https://msdn.microsoft.com/en-us/library/ms190290.aspx
dropFunction
    : DROP FUNCTION (IF EXISTS)? funcProcNameSchema (',' funcProcNameSchema)* ';'?
    ;

// https://msdn.microsoft.com/en-us/library/ms175075.aspx
dropStatistics
    : DROP STATISTICS (COMMA? (tableName '.')? name=id)+ ';'
    ;

// https://msdn.microsoft.com/en-us/library/ms173790.aspx
dropTable
    : DROP TABLE (IF EXISTS)? tableName ';'?
    ;

// https://msdn.microsoft.com/en-us/library/ms173492.aspx
dropView
    : DROP VIEW (IF EXISTS)? simpleName (',' simpleName)* ';'?
    ;

createType
    : CREATE TYPE name = simpleName
      (FROM dataType defaultValue)?
      (AS TABLE LR_BRACKET columnDefTableConstraints RR_BRACKET)?
    ;

dropType:
    DROP TYPE ( IF EXISTS )? name = simpleName
    ;

rowsetFunctionLimited
    : openquery
    | opendatasource
    ;

// https://msdn.microsoft.com/en-us/library/ms188427(v=sql.120).aspx
openquery
    : OPENQUERY '(' linkedServer=id ',' query=STRING ')'
    ;

// https://msdn.microsoft.com/en-us/library/ms179856.aspx
opendatasource
    : OPENDATASOURCE '(' provider=STRING ',' init=STRING ')'
     '.' (database=id)? '.' (scheme=id)? '.' (table=id)
    ;

// Other statements.

// https://msdn.microsoft.com/en-us/library/ms188927.aspx
declareStatement
    : DECLARE LOCAL_ID AS? tableTypeDefinition ';'?
    | DECLARE declareLocal (',' declareLocal)* ';'?
    | DECLARE LOCAL_ID AS? xmlTypeDefinition ';'?
    | WITH XMLNAMESPACES '(' xmlNamespaceUri=STRING ','? AS id ')' ';'?
    ;

// https://msdn.microsoft.com/en-us/library/ms181441(v=sql.120).aspx
cursorStatement
    // https://msdn.microsoft.com/en-us/library/ms175035(v=sql.120).aspx
    : CLOSE GLOBAL? cursorName ';'?
    // https://msdn.microsoft.com/en-us/library/ms188782(v=sql.120).aspx
    | DEALLOCATE GLOBAL? CURSOR? cursorName ';'?
    // https://msdn.microsoft.com/en-us/library/ms180169(v=sql.120).aspx
    | declareCursor
    // https://msdn.microsoft.com/en-us/library/ms180152(v=sql.120).aspx
    | fetchCursor
    // https://msdn.microsoft.com/en-us/library/ms190500(v=sql.120).aspx
    | OPEN GLOBAL? cursorName ';'?
    ;
// https://docs.microsoft.com/en-us/sql/t-sql/statements/backup-transact-sql
backupDatabase
    : BACKUP DATABASE ( databaseName=id )
          (READ_WRITE_FILEGROUPS (COMMA? (FILE|FILEGROUP) EQUAL fileOrFilegroup=STRING)* )?
          (COMMA? (FILE|FILEGROUP) EQUAL fileOrFilegroup=STRING)*
           ( TO ( COMMA? logicalDeviceName=id)+
           | TO ( COMMA? (DISK|TAPE|URL) EQUAL (STRING|id) )+
           )

           ( (MIRROR TO ( COMMA? logicalDeviceName=id)+ )+
           | ( MIRROR TO ( COMMA? (DISK|TAPE|URL) EQUAL (STRING|id) )+ )+
           )?

             (WITH ( COMMA? DIFFERENTIAL
                   | COMMA? COPY_ONLY
                   | COMMA? (COMPRESSION|NO_COMPRESSION)
                   | COMMA? DESCRIPTION EQUAL (STRING|id)
                   | COMMA? NAME EQUAL backupSetName=id
                   | COMMA? CREDENTIAL
                   | COMMA? FILE_SNAPSHOT
                   | COMMA? (EXPIREDATE EQUAL (STRING|id) | RETAINDAYS EQUAL (DECIMAL|id) )
                   | COMMA? (NOINIT|INIT)
                   | COMMA? (NOSKIP|SKIP_KEYWORD)
                   | COMMA? (NOFORMAT|FORMAT)
                   | COMMA? MEDIADESCRIPTION EQUAL (STRING|id)
                   | COMMA? MEDIANAME EQUAL (medianame=STRING)
                   | COMMA? BLOCKSIZE EQUAL (DECIMAL|id)
                   | COMMA? BUFFERCOUNT EQUAL (DECIMAL|id)
                   | COMMA? MAXTRANSFER EQUAL (DECIMAL|id)
                   | COMMA? (NO_CHECKSUM|CHECKSUM)
                   | COMMA? (STOP_ON_ERROR|CONTINUE_AFTER_ERROR)
                   | COMMA? RESTART
                   | COMMA? STATS (EQUAL statsPercent=DECIMAL)?
                   | COMMA? (REWIND|NOREWIND)
                   | COMMA? (LOAD|NOUNLOAD)
                   | COMMA? ENCRYPTION LR_BRACKET
                                         ALGORITHM EQUAL
                                         (AES_128
                                         | AES_192
                                         | AES_256
                                         | TRIPLE_DES_3KEY
                                         )
                                         COMMA
                                         SERVER CERTIFICATE EQUAL
                                           (encryptorName=id
                                           | SERVER ASYMMETRIC KEY EQUAL encryptorName=id
                                           )
                  )*
              )?

    ;

backupLog
    : BACKUP LOG ( databaseName=id )
           ( TO ( COMMA? logicalDeviceName=id)+
           | TO ( COMMA? (DISK|TAPE|URL) EQUAL (STRING|id) )+
           )

           ( (MIRROR TO ( COMMA? logicalDeviceName=id)+ )+
           | ( MIRROR TO ( COMMA? (DISK|TAPE|URL) EQUAL (STRING|id) )+ )+
           )?

             (WITH ( COMMA? DIFFERENTIAL
                   | COMMA? COPY_ONLY
                   | COMMA? (COMPRESSION|NO_COMPRESSION)
                   | COMMA? DESCRIPTION EQUAL (STRING|id)
                   | COMMA? NAME EQUAL backupSetName=id
                   | COMMA? CREDENTIAL
                   | COMMA? FILE_SNAPSHOT
                   | COMMA? (EXPIREDATE EQUAL (STRING|id) | RETAINDAYS EQUAL (DECIMAL|id) )
                   | COMMA? (NOINIT|INIT)
                   | COMMA? (NOSKIP|SKIP_KEYWORD)
                   | COMMA? (NOFORMAT|FORMAT)
                   | COMMA? MEDIADESCRIPTION EQUAL (STRING|id)
                   | COMMA? MEDIANAME EQUAL (medianame=STRING)
                   | COMMA? BLOCKSIZE EQUAL (DECIMAL|id)
                   | COMMA? BUFFERCOUNT EQUAL (DECIMAL|id)
                   | COMMA? MAXTRANSFER EQUAL (DECIMAL|id)
                   | COMMA? (NO_CHECKSUM|CHECKSUM)
                   | COMMA? (STOP_ON_ERROR|CONTINUE_AFTER_ERROR)
                   | COMMA? RESTART
                   | COMMA? STATS (EQUAL statsPercent=DECIMAL)?
                   | COMMA? (REWIND|NOREWIND)
                   | COMMA? (LOAD|NOUNLOAD)
                   | COMMA? (NORECOVERY| STANDBY EQUAL undoFileName=STRING)
                   | COMMA? NO_TRUNCATE
                   | COMMA? ENCRYPTION LR_BRACKET
                                         ALGORITHM EQUAL
                                         (AES_128
                                         | AES_192
                                         | AES_256
                                         | TRIPLE_DES_3KEY
                                         )
                                         COMMA
                                         SERVER CERTIFICATE EQUAL
                                           (encryptorName=id
                                           | SERVER ASYMMETRIC KEY EQUAL encryptorName=id
                                           )
                  )*
              )?

    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/backup-certificate-transact-sql
backupCertificate
    : BACKUP CERTIFICATE certname=id TO FILE EQUAL certFile=STRING
       ( WITH PRIVATE KEY
           LR_BRACKET
             (COMMA? FILE EQUAL privateKeyFile=STRING
             |COMMA? ENCRYPTION BY PASSWORD EQUAL encryptionPassword=STRING
             |COMMA? DECRYPTION BY PASSWORD EQUAL decryptionPasword=STRING
             )+
           RR_BRACKET
       )?
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/backup-master-key-transact-sql
backupMasterKey
    : BACKUP MASTER KEY TO FILE EQUAL masterKeyBackupFile=STRING
         ENCRYPTION BY PASSWORD EQUAL encryptionPassword=STRING
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/statements/backup-service-master-key-transact-sql
backupServiceMasterKey
    : BACKUP SERVICE MASTER KEY TO FILE EQUAL serviceMasterKeyBackupFile=STRING
         ENCRYPTION BY PASSWORD EQUAL encryptionPassword=STRING
    ;


// https://msdn.microsoft.com/en-us/library/ms188332.aspx
executeStatement
    : EXECUTE executeBody
    ;

executeBody
    : (returnStatus=LOCAL_ID '=')? (funcProcNameServerDatabaseSchema | expression) (executeStatementArg (',' executeStatementArg)*)? ';'?
    | '(' executeVarString ('+' executeVarString)* ')' (AS? (LOGIN | USER) '=' STRING)? ';'?
    ;

executeStatementArg
    : (parameter=LOCAL_ID '=')? ((constant_LOCAL_ID | id) (OUTPUT | OUT)? | DEFAULT | NULL)
    ;

executeVarString
    : LOCAL_ID
    | STRING
    ;

// https://msdn.microsoft.com/en-us/library/ff848791.aspx
securityStatement
    // https://msdn.microsoft.com/en-us/library/ms188354.aspx
    : executeClause ';'?
    // https://msdn.microsoft.com/en-us/library/ms187965.aspx
    | GRANT (ALL PRIVILEGES? | grantPermission ('(' columnNameList ')')?) (ON onId=tableName)? TO (toPrincipal+=id) (',' toPrincipal+=id)* (WITH GRANT OPTION)? (AS asPrincipal=id)? ';'?
    // https://msdn.microsoft.com/en-us/library/ms178632.aspx
    | REVERT ('(' WITH COOKIE '=' LOCAL_ID ')')? ';'?
    | openKey
    | closeKey
    | createKey
    | createCertificate
    ;

createCertificate
    : CREATE CERTIFICATE certificateName=id (AUTHORIZATION userName=id)?
      (FROM existingKeys | generateNewKeys)
      (ACTIVE FOR BEGIN DIALOG '=' (ON | OFF))?
    ;

existingKeys
    : ASSEMBLY assemblyName=id
    | EXECUTABLE? FILE EQUAL pathToFile=STRING (WITH PRIVATE KEY '(' privateKeyOptions ')')?
    ;

privateKeyOptions
    : (FILE | BINARY) '=' path=STRING (',' (DECRYPTION | ENCRYPTION) BY PASSWORD '=' password=STRING)?
    ;

generateNewKeys
    : (ENCRYPTION BY PASSWORD '=' password=STRING)?
      WITH SUBJECT EQUAL certificateSubjectName=STRING (',' dateOptions)*
    ;

dateOptions
    : (START_DATE | EXPIRY_DATE) EQUAL STRING
    ;

openKey
    : OPEN SYMMETRIC KEY keyName=id DECRYPTION BY decryptionMechanism
    | OPEN MASTER KEY DECRYPTION BY PASSWORD '=' password=STRING
    ;

closeKey
    : CLOSE SYMMETRIC KEY keyName=id
    | CLOSE ALL SYMMETRIC KEYS
    | CLOSE MASTER KEY
    ;

createKey
    : CREATE MASTER KEY ENCRYPTION BY PASSWORD '=' password=STRING
    | CREATE SYMMETRIC KEY keyName=id
      (AUTHORIZATION userName=id)?
      (FROM PROVIDER providerName=id)?
      WITH ((keyOptions | ENCRYPTION BY encryptionMechanism)','?)+
    ;

keyOptions
    : KEY_SOURCE EQUAL passPhrase=STRING
    | ALGORITHM EQUAL algorithm
    | IDENTITY_VALUE EQUAL identityPhrase=STRING
    | PROVIDER_KEY_NAME EQUAL keyNameInProvider=STRING
    | CREATION_DISPOSITION EQUAL (CREATE_NEW | OPEN_EXISTING)
    ;

algorithm
    : DES
    | TRIPLE_DES
    | TRIPLE_DES_3KEY
    | RC2
    | RC4
    | RC4_128
    | DESX
    | AES_128
    | AES_192
    | AES_256
    ;

encryptionMechanism
    : CERTIFICATE certificateName=id
    | ASYMMETRIC KEY asymKeyName=id
    | SYMMETRIC KEY decrypting_KeyName=id
    | PASSWORD '=' STRING
    ;

decryptionMechanism
    : CERTIFICATE certificateName=id (WITH PASSWORD EQUAL STRING)?
    | ASYMMETRIC KEY asymKeyName=id (WITH PASSWORD EQUAL STRING)?
    | SYMMETRIC KEY decrypting_KeyName=id
    | PASSWORD EQUAL STRING
    ;

grantPermission
    : EXECUTE
    | VIEW id // DEFINITION
    | TAKE id // OWNERSHIP
    | CONTROL id? // SERVER
    | CREATE (TABLE | VIEW)
    | SHOWPLAN
    | IMPERSONATE
    | SELECT
    | REFERENCES
    | INSERT
    | ALTER (ANY? (id | DATABASE))?
    ;

// https://msdn.microsoft.com/en-us/library/ms190356.aspx
// https://msdn.microsoft.com/en-us/library/ms189484.aspx
setStatement
    : SET LOCAL_ID ('.' memberName=id)? '=' expression ';'?
    | SET LOCAL_ID assignmentOperator expression ';'?
    | SET LOCAL_ID '='
      CURSOR declareSetCursorCommon (FOR (READ ONLY | UPDATE (OF columnNameList)?))? ';'?
    // https://msdn.microsoft.com/en-us/library/ms189837.aspx
    | setSpecial
    ;

// https://msdn.microsoft.com/en-us/library/ms174377.aspx
transactionStatement
    // https://msdn.microsoft.com/en-us/library/ms188386.aspx
    : BEGIN DISTRIBUTED (TRAN | TRANSACTION) (id | LOCAL_ID)? ';'?
    // https://msdn.microsoft.com/en-us/library/ms188929.aspx
    | BEGIN (TRAN | TRANSACTION) ((id | LOCAL_ID) (WITH MARK STRING)?)? ';'?
    // https://msdn.microsoft.com/en-us/library/ms190295.aspx
    | COMMIT (TRAN | TRANSACTION) ((id | LOCAL_ID) (WITH '(' DELAYED_DURABILITY EQUAL (OFF | ON) ')')?)? ';'?
    // https://msdn.microsoft.com/en-us/library/ms178628.aspx
    | COMMIT WORK? ';'?
    | COMMIT id
    | ROLLBACK id
    // https://msdn.microsoft.com/en-us/library/ms181299.aspx
    | ROLLBACK (TRAN | TRANSACTION) (id | LOCAL_ID)? ';'?
    // https://msdn.microsoft.com/en-us/library/ms174973.aspx
    | ROLLBACK WORK? ';'?
    // https://msdn.microsoft.com/en-us/library/ms188378.aspx
    | SAVE (TRAN | TRANSACTION) (id | LOCAL_ID)? ';'?
    ;

// https://msdn.microsoft.com/en-us/library/ms188037.aspx
goStatement
    : GO (count=DECIMAL)?
    ;

// https://msdn.microsoft.com/en-us/library/ms188366.aspx
useStatement
    : USE database=id ';'?
    ;

setuserStatement
    : SETUSER user=STRING?
    ;

dbccClause
    : DBCC name=simpleId ('(' expressionList ')')? (WITH dbccOptions)? ';'?
    ;

dbccOptions
    :  simpleId (',' simpleId)?
    ;

executeClause
    : EXECUTE AS clause=(CALLER | SELF | OWNER | STRING)
    ;

declareLocal
    : LOCAL_ID AS? dataType ('=' expression)?
    ;

tableTypeDefinition
    : TABLE '(' columnDefTableConstraints ')'
    ;

xmlTypeDefinition
    : XML '(' ( CONTENT | DOCUMENT )? xmlSchemaCollection ')'
    ;

xmlSchemaCollection
    : ID '.' ID
    ;

columnDefTableConstraints
    : columnDefTableConstraint (','? columnDefTableConstraint)*
    ;

columnDefTableConstraint
    : columnDefinition
    | materializedColumnDefinition
    | tableConstraint
    ;

// https://msdn.microsoft.com/en-us/library/ms187742.aspx
columnDefinition
    : id (dataType | AS expression) (COLLATE id)? nullNotnull?
      ((CONSTRAINT constraint=id)? nullOrDefault nullOrDefault?
       | IDENTITY ('(' seed=DECIMAL ',' increment=DECIMAL ')')? (NOT FOR REPLICATION)?)?
      ROWGUIDCOL?
      columnConstraint*
    ;

materializedColumnDefinition
    : id (COMPUTE | AS) expression (MATERIALIZED | NOT MATERIALIZED)?
    ;

// https://msdn.microsoft.com/en-us/library/ms186712.aspx
columnConstraint
    :(CONSTRAINT constraint=id)?
      ((PRIMARY KEY | UNIQUE) clustered? indexOptions?
      | CHECK (NOT FOR REPLICATION)? '(' searchCondition ')'
      | (FOREIGN KEY)? REFERENCES tableName '(' pk = columnNameList')' onDelete? onUpdate?
      | nullNotnull)
    ;

// https://msdn.microsoft.com/en-us/library/ms188066.aspx
tableConstraint
    : (CONSTRAINT constraint=id)?
       ((PRIMARY KEY | UNIQUE) clustered? '(' columnNameListWithOrder ')' indexOptions? (ON id)?
         | CHECK (NOT FOR REPLICATION)? '(' searchCondition ')'
         | DEFAULT '('?  (STRING | PLUS | functionCall | DECIMAL)+ ')'? FOR id
         | FOREIGN KEY '(' fk = columnNameList ')' REFERENCES tableName ('(' pk = columnNameList')')? onDelete? onUpdate?)
    ;

onDelete
    : ON DELETE (NO ACTION | CASCADE | SET NULL | SET DEFAULT)
    ;

onUpdate
    : ON UPDATE (NO ACTION | CASCADE | SET NULL | SET DEFAULT)
    ;

indexOptions
    : WITH '(' indexOption (',' indexOption)* ')'
    ;

// https://msdn.microsoft.com/en-us/library/ms186869.aspx
// Id runtime checking. Id in (PAD_INDEX, FILLFACTOR, IGNORE_DUP_KEY, STATISTICS_NORECOMPUTE, ALLOW_ROW_LOCKS,
// ALLOW_PAGE_LOCKS, SORT_IN_TEMPDB, ONLINE, MAXDOP, DATA_COMPRESSION, ONLINE).
indexOption
    : simpleId '=' (simpleId | onOff | DECIMAL)
    ;

// https://msdn.microsoft.com/en-us/library/ms180169.aspx
declareCursor
    : DECLARE cursorName
      (CURSOR (declareSetCursorCommon (FOR UPDATE (OF columnNameList)?)?)?
      | (SEMI_SENSITIVE | INSENSITIVE)? SCROLL? CURSOR FOR selectStatement (FOR (READ ONLY | UPDATE | (OF columnNameList)))?
      ) ';'?
    ;

declareSetCursorCommon
    : declareSetCursorCommonPartial*
      FOR selectStatement
    ;

declareSetCursorCommonPartial
    : (LOCAL | GLOBAL)
    | (FORWARD_ONLY | SCROLL)
    | (STATIC | KEYSET | DYNAMIC | FAST_FORWARD)
    | (READ_ONLY | SCROLL_LOCKS | OPTIMISTIC)
    | TYPE_WARNING
    ;

fetchCursor
    : FETCH ((NEXT | PRIOR | FIRST | LAST | (ABSOLUTE | RELATIVE) expression)? FROM)?
      GLOBAL? cursorName (INTO LOCAL_ID (',' LOCAL_ID)*)? ';'?
    ;

// https://msdn.microsoft.com/en-us/library/ms190356.aspx
// Runtime check.
setSpecial
    : SET id (id | constant_LOCAL_ID | onOff) ';'?
    // https://msdn.microsoft.com/en-us/library/ms173763.aspx
    | SET TRANSACTION ISOLATION LEVEL
      (READ UNCOMMITTED | READ COMMITTED | REPEATABLE READ | SNAPSHOT | SERIALIZABLE | DECIMAL) ';'?
    // https://msdn.microsoft.com/en-us/library/ms188059.aspx
    | SET IDENTITY_INSERT tableName onOff ';'?
    | SET ANSI_NULLS onOff
    | SET QUOTED_IDENTIFIER onOff
    | SET ANSI_PADDING onOff
    | SET ANSI_WARNINGS onOff
    | SET modifyMethod
    ;

constant_LOCAL_ID
    : constant
    | LOCAL_ID
    ;

// Expression.

// https://docs.microsoft.com/en-us/sql/t-sql/language-elements/expressions-transact-sql
// Operator precendence: https://docs.microsoft.com/en-us/sql/t-sql/language-elements/operator-precedence-transact-sql
expression
    : primitiveExpression
    | functionCall
    | expression COLLATE id
    | caseExpression
    | fullColumnName
    | bracketExpression
    | unaryOperatorExpression
    | expression op=('*' | '/' | '%') expression
    | expression op=('+' | '-' | '&' | '^' | '|' | '||') expression
    | expression comparisonOperator expression
    | expression assignmentOperator expression
    | overClause
    ;

primitiveExpression
    : DEFAULT | NULL | LOCAL_ID | constant
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/language-elements/case-transact-sql
caseExpression
    : CASE caseExpr=expression switchSection+ (ELSE elseExpr=expression)? END
    | CASE switchSearchConditionSection+ (ELSE elseExpr=expression)? END
    ;

unaryOperatorExpression
    : '~' expression
    | op=('+' | '-') expression
    ;

bracketExpression
    : '(' expression ')' | '(' subquery ')'
    ;

constantExpression
    : NULL
    | constant
    // system functions: https://msdn.microsoft.com/en-us/library/ms187786.aspx
    | functionCall
    | LOCAL_ID         // TODO: remove.
    | '(' constantExpression ')'
    ;

subquery
    : selectStatement
    ;

// https://msdn.microsoft.com/en-us/library/ms175972.aspx
withExpression
    : WITH (XMLNAMESPACES ',')? commonTableExpression (',' commonTableExpression)*
    | WITH BLOCKING_HIERARCHY ('(' fullColumnNameList ')')? AS '(' selectStatement ')'
    ;

commonTableExpression
    : expressionName=id ('(' columnNameList ')')? AS '(' selectStatement ')'
    ;

updateElem
    : (fullColumnName | LOCAL_ID) ('=' | assignmentOperator) expression
    | udtColumnName=id '.' methodName=id '(' expressionList ')'
    //| fullColumnName '.' WRITE (expression, )
    ;

// https://msdn.microsoft.com/en-us/library/ms173545.aspx
searchConditionList
    : searchCondition (',' searchCondition)*
    ;

searchCondition
    : searchConditionAnd (OR searchConditionAnd)*
    ;

searchConditionAnd
    : searchConditionNot (AND searchConditionNot)*
    ;

searchConditionNot
    : NOT? predicate
    ;

predicate
    : EXISTS '(' subquery ')'
    | expression comparisonOperator expression
    | expression comparisonOperator (ALL | SOME | ANY) '(' subquery ')'
    | expression NOT? BETWEEN expression AND expression
    | expression NOT? IN '(' (subquery | expressionList) ')'
    | expression NOT? LIKE expression (ESCAPE expression)?
    | expression IS nullNotnull
    | '(' searchCondition ')'
    ;

// Changed union rule to sqlUnion to avoid union construct with C++ target.  Issue reported by person who generates into C++.  This individual reports change causes generated code to work

queryExpression
    : (querySpecification | '(' queryExpression ')') sqlUnion*
    ;

sqlUnion
    : (UNION ALL? | EXCEPT | INTERSECT) (querySpecification | ('(' queryExpression ')'))
    ;

// https://msdn.microsoft.com/en-us/library/ms176104.aspx
querySpecification
    : SELECT (ALL | DISTINCT)? topClause?
      selectList
      // https://msdn.microsoft.com/en-us/library/ms188029.aspx
      (INTO tableName)?
      (FROM tableSources)?
      (WHERE where_=searchCondition)?
      // https://msdn.microsoft.com/en-us/library/ms177673.aspx
      (GROUP BY (ALL)? groupByItem (',' groupByItem)*)?
      (HAVING having=searchCondition)?
    ;

// https://msdn.microsoft.com/en-us/library/ms189463.aspx
topClause
    : TOP (topPercent | topCount) (WITH TIES)?
    ;

topPercent
    : (REAL | FLOAT) PERCENT
    | '(' expression ')' PERCENT
    ;

topCount
    : DECIMAL
    | '(' expression ')'
    ;

// https://msdn.microsoft.com/en-us/library/ms188385.aspx
orderByClause
    : ORDER BY orderByExpression (',' orderByExpression)*
      (OFFSET expression (ROW | ROWS) (FETCH (FIRST | NEXT) expression (ROW | ROWS) ONLY)?)?
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/queries/select-for-clause-transact-sql
forClause
    : FOR BROWSE
    | FOR XML (RAW ('(' STRING ')')? | AUTO) xmlCommonDirectives*
      (COMMA (XMLDATA | XMLSCHEMA ('(' STRING ')')?))?
      (COMMA ELEMENTS (XSINIL | ABSENT))?
    | FOR XML EXPLICIT xmlCommonDirectives*
      (COMMA XMLDATA)?
    | FOR XML PATH ('(' STRING ')')? xmlCommonDirectives*
      (COMMA ELEMENTS (XSINIL | ABSENT))?
    | FOR JSON (AUTO | PATH)
      (COMMA ROOT ('(' STRING ')')?)?
      (COMMA INCLUDE_NULL_VALUES)?
      (COMMA WITHOUT_ARRAY_WRAPPER)?
    ;

xmlCommonDirectives
    : ',' (BINARY_BASE64 | TYPE | ROOT)
    ;

orderByExpression
    : expression (ASC | DESC)?
    ;

groupByItem
    : expression
    /*| rollupSpec
    | cubeSpec
    | groupingSetsSpec
    | grandTotal*/
    ;

optionClause
    // https://msdn.microsoft.com/en-us/library/ms181714.aspx
    : OPTION '(' option (',' option)* ')'
    ;

option
    : FAST numberRows=DECIMAL
    | (HASH | ORDER) GROUP
    | (MERGE | HASH | CONCAT) UNION
    | (LOOP | MERGE | HASH) JOIN
    | EXPAND VIEWS
    | FORCE ORDER
    | IGNORE_NONCLUSTERED_COLUMNSTORE_INDEX
    | KEEP PLAN
    | KEEPFIXED PLAN
    | MAXDOP numberOfProcessors=DECIMAL
    | MAXRECURSION numberRecursion=DECIMAL
    | OPTIMIZE FOR '(' optimizeForArg (',' optimizeForArg)* ')'
    | OPTIMIZE FOR UNKNOWN
    | PARAMETERIZATION (SIMPLE | FORCED)
    | RECOMPILE
    | ROBUST PLAN
    | USE PLAN STRING
    ;

optimizeForArg
    : LOCAL_ID (UNKNOWN | '=' (constant | NULL))
    ;

// https://msdn.microsoft.com/en-us/library/ms176104.aspx
selectList
    : selectListElem (',' selectListElem)*
    ;

udtMethodArguments
    : '(' executeVarString (',' executeVarString)* ')'
    ;

// https://docs.microsoft.com/ru-ru/sql/t-sql/queries/select-clause-transact-sql
asterisk
    : '*'
    | tableName '.' asterisk
    ;

columnElem
    : (tableName '.')? (columnName=id | '$' IDENTITY | '$' ROWGUID) asColumnAlias?
    ;

udtElem
    : udtColumnName=id '.' nonStaticAttr=id udtMethodArguments asColumnAlias?
    | udtColumnName=id ':' ':' staticAttr=id udtMethodArguments? asColumnAlias?
    ;

expressionElem
    : columnAlias eq='=' expression
    | expression asColumnAlias?
    ;

selectListElem
    : asterisk
    | columnElem
    | udtElem
    | expressionElem
    ;

tableSources
    : tableSource (',' tableSource)*
    ;

// https://msdn.microsoft.com/en-us/library/ms177634.aspx
tableSource
    : tableSourceItemJoined
    | '(' tableSourceItemJoined ')'
    ;

tableSourceItemJoined
    : tableSourceItem joinPart*
    ;

tableSourceItem
    : tableNameWithHint        asTableAlias?
    | fullTableName             asTableAlias?
    | rowsetFunction             asTableAlias?
    | derivedTable              (asTableAlias columnAliasList?)?
    | changeTable                asTableAlias
    | functionCall               asTableAlias?
    | LOCAL_ID                    asTableAlias?
    | LOCAL_ID '.' functionCall (asTableAlias columnAliasList?)?
    | openXml
    | ':' ':' functionCall       asTableAlias? // Build-in function (old syntax)
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/functions/openxml-transact-sql
openXml
    : OPENXML '(' expression ',' expression (',' expression)? ')'
    (WITH '(' schemaDeclaration ')' )?
    ;

schemaDeclaration
    : columnDeclaration (',' columnDeclaration)*
    ;

columnDeclaration
    : ID dataType (STRING)?
    ;

changeTable
    : CHANGETABLE '(' CHANGES tableName ',' (NULL | DECIMAL | LOCAL_ID) ')'
    ;

// https://msdn.microsoft.com/en-us/library/ms191472.aspx
joinPart
    // https://msdn.microsoft.com/en-us/library/ms173815(v=sql.120).aspx
    : (INNER? |
       joinType=(LEFT | RIGHT | FULL) OUTER?) (joinHint=(LOOP | HASH | MERGE | REMOTE))?
       JOIN tableSource ON searchCondition
    | CROSS JOIN tableSource
    | CROSS APPLY tableSource
    | OUTER APPLY tableSource
    | PIVOT pivotClause asTableAlias
    | UNPIVOT unpivotClause asTableAlias
    ;

pivotClause
    : '(' aggregateWindowedFunction FOR fullColumnName IN columnAliasList ')'
    ;

unpivotClause
    : '(' expression FOR fullColumnName IN '(' fullColumnNameList ')' ')'
    ;

fullColumnNameList
    : fullColumnName (',' fullColumnName)*
    ;

tableNameWithHint
    : tableName withTableHints?
    ;

// https://msdn.microsoft.com/en-us/library/ms190312.aspx
rowsetFunction
    :  (
        OPENROWSET LR_BRACKET providerName = STRING COMMA connectionString = STRING COMMA sql = STRING RR_BRACKET
     )
     | ( OPENROWSET '(' BULK dataFile=STRING ',' (bulkOption (',' bulkOption)* | id)')' )
    ;

// runtime check.
bulkOption
    : id '=' bulkOptionValue=(DECIMAL | STRING)
    ;

derivedTable
    : subquery
    | '(' subquery ')'
    | tableValueConstructor
    | '(' tableValueConstructor ')'
    ;

functionCall
    : rankingWindowedFunction                         #RANKING_WINDOWED_FUNC
    | aggregateWindowedFunction                       #AGGREGATE_WINDOWED_FUNC
    | analyticWindowedFunction                        #ANALYTIC_WINDOWED_FUNC
    | scalarFunctionName '(' expressionList? ')'     #SCALAR_FUNCTION
    // https://msdn.microsoft.com/en-us/library/ms173784.aspx
    | BINARY_CHECKSUM '(' '*' ')'                       #BINARY_CHECKSUM
    // https://msdn.microsoft.com/en-us/library/hh231076.aspx
    // https://msdn.microsoft.com/en-us/library/ms187928.aspx
    | CAST '(' expression AS dataType ')'              #CAST
    | CONVERT '(' convertDataType=dataType ','convertExpression=expression (',' style=expression)? ')'                              #CONVERT
    // https://msdn.microsoft.com/en-us/library/ms189788.aspx
    | CHECKSUM '(' '*' ')'                              #CHECKSUM
    // https://msdn.microsoft.com/en-us/library/ms190349.aspx
    | COALESCE '(' expressionList ')'                  #COALESCE
    // https://msdn.microsoft.com/en-us/library/ms188751.aspx
    | CURRENT_TIMESTAMP                                 #CURRENT_TIMESTAMP
    // https://msdn.microsoft.com/en-us/library/ms176050.aspx
    | CURRENT_USER                                      #CURRENT_USER
    // https://msdn.microsoft.com/en-us/library/ms186819.aspx
    | DATEADD '(' ID ',' expression ',' expression ')'  #DATEADD
    // https://msdn.microsoft.com/en-us/library/ms189794.aspx
    | DATEDIFF '(' ID ',' expression ',' expression ')' #DATEDIFF
    // https://msdn.microsoft.com/en-us/library/ms174395.aspx
    | DATENAME '(' ID ',' expression ')'                #DATENAME
    // https://msdn.microsoft.com/en-us/library/ms174420.aspx
    | DATEPART '(' ID ',' expression ')'                #DATEPART
    // https://docs.microsoft.com/en-us/sql/t-sql/functions/getdate-transact-sql
    | GETDATE '(' ')'                                   #GETDATE
    // https://docs.microsoft.com/en-us/sql/t-sql/functions/getdate-transact-sql
    | GETUTCDATE '(' ')'                                #GETUTCDATE
    // https://msdn.microsoft.com/en-us/library/ms189838.aspx
    | IDENTITY '(' dataType (',' seed=DECIMAL)? (',' increment=DECIMAL)? ')'                                                           #IDENTITY
    // https://msdn.microsoft.com/en-us/library/bb839514.aspx
    | MIN_ACTIVE_ROWVERSION                             #MIN_ACTIVE_ROWVERSION
    // https://msdn.microsoft.com/en-us/library/ms177562.aspx
    | NULLIF '(' expression ',' expression ')'          #NULLIF
    // https://msdn.microsoft.com/fr-fr/library/ms188043.aspx
    | STUFF '(' expression ',' DECIMAL ',' DECIMAL ',' expression ')'                                                                   #STUFF
    // https://msdn.microsoft.com/en-us/library/ms177587.aspx
    | SESSION_USER                                      #SESSION_USER
    // https://msdn.microsoft.com/en-us/library/ms179930.aspx
    | SYSTEM_USER                                       #SYSTEM_USER
    // https://msdn.microsoft.com/en-us/library/ms184325.aspx
    | ISNULL '(' expression ',' expression ')'          #ISNULL
    // https://docs.microsoft.com/en-us/sql/t-sql/xml/xml-data-type-methods
    | xmlDataTypeMethods                             #XML_DATA_TYPE_FUNC
    // https://docs.microsoft.com/en-us/sql/t-sql/functions/logical-functions-iif-transact-sql
    | IIF '(' searchCondition ',' expression ',' expression ')'   #IFF
    ;

xmlDataTypeMethods
    : valueMethod
    | queryMethod
    | existMethod
    | modifyMethod
    | nodesMethod
    ;

valueMethod
    : (LOCAL_ID | ID | EVENTDATA | queryMethod) '.' VALUE '(' xquery=STRING ',' sqltype=STRING ')'
    | (LOCAL_ID | ID | EVENTDATA | queryMethod) '.' ROW '.' VALUE '(' xquery=STRING ',' sqltype=STRING ')'
    | (LOCAL_ID | ID | EVENTDATA | queryMethod) '.' PARAM_NODE '.' VALUE '(' xquery=STRING ',' sqltype=STRING ')'
    ;

queryMethod
    : (LOCAL_ID | ID | fullTableName) '.' QUERY '(' xquery=STRING ')'
    | (LOCAL_ID | ID | fullTableName) '.' ROW '.' QUERY '(' xquery=STRING ')'
    ;

existMethod
    : (LOCAL_ID | ID) '.' EXIST '(' xquery=STRING ')'
    ;

modifyMethod
    : (LOCAL_ID | ID) '.' MODIFY '(' xmlDml=STRING ')'
    ;

nodesMethod
    : (LOCAL_ID | ID) '.' NODES '(' xquery=STRING ')'
    ;


switchSection
    : WHEN expression THEN expression
    ;

switchSearchConditionSection
    : WHEN searchCondition THEN expression
    ;

asColumnAlias
    : AS? columnAlias
    ;

asTableAlias
    : AS? tableAlias
    ;

tableAlias
    : id withTableHints?
    ;

// https://msdn.microsoft.com/en-us/library/ms187373.aspx
withTableHints
    : WITH? '(' tableHint (','? tableHint)* ')'
    ;

// https://msdn.microsoft.com/en-us/library/ms187373.aspx
insertWithTableHints
    : WITH '(' tableHint (','? tableHint)* ')'
    ;

// Id runtime check. Id can be (FORCESCAN, HOLDLOCK, NOLOCK, NOWAIT, PAGLOCK, READCOMMITTED,
// READCOMMITTEDLOCK, READPAST, READUNCOMMITTED, REPEATABLEREAD, ROWLOCK, TABLOCK, TABLOCKX
// UPDLOCK, XLOCK)
tableHint
    : NOEXPAND? ( INDEX ('(' indexValue (',' indexValue)* ')' | indexValue (',' indexValue)*)
                | INDEX '=' indexValue
                | FORCESEEK ('(' indexValue '(' ID  (',' ID)* ')' ')')?
                | SERIALIZABLE
                | SNAPSHOT
                | SPATIAL_WINDOW_MAX_CELLS '=' DECIMAL
                | ID)
    ;

indexValue
    : id | DECIMAL
    ;

columnAliasList
    : '(' columnAlias (',' columnAlias)* ')'
    ;

columnAlias
    : id
    | STRING
    ;

tableValueConstructor
    : VALUES '(' expressionList ')' (',' '(' expressionList ')')*
    ;

expressionList
    : expression (',' expression)*
    ;

// https://msdn.microsoft.com/en-us/library/ms189798.aspx
rankingWindowedFunction
    : (RANK | DENSE_RANK | ROW_NUMBER) '(' ')' overClause
    | NTILE '(' expression ')' overClause
    ;

// https://msdn.microsoft.com/en-us/library/ms173454.aspx
aggregateWindowedFunction
    : (AVG | MAX | MIN | SUM | STDEV | STDEVP | VAR | VARP)
      '(' allDistinctExpression ')' overClause?
    | (COUNT | COUNT_BIG)
      '(' ('*' | allDistinctExpression) ')' overClause?
    | CHECKSUM_AGG '(' allDistinctExpression ')'
    | GROUPING '(' expression ')'
    | GROUPING_ID '(' expressionList ')'
    ;

// https://docs.microsoft.com/en-us/sql/t-sql/functions/analytic-functions-transact-sql
analyticWindowedFunction
    : (FIRST_VALUE | LAST_VALUE) '(' expression ')' overClause
    | (LAG | LEAD) '(' expression  (',' expression (',' expression)? )? ')' overClause
    ;

allDistinctExpression
    : (ALL | DISTINCT)? expression
    ;

// https://msdn.microsoft.com/en-us/library/ms189461.aspx
overClause
    : OVER '(' (PARTITION BY expressionList)? orderByClause? rowOrRangeClause? ')'
    ;

rowOrRangeClause
    : (ROWS | RANGE) windowFrameExtent
    ;

windowFrameExtent
    : windowFramePreceding
    | BETWEEN windowFrameBound AND windowFrameBound
    ;

windowFrameBound
    : windowFramePreceding
    | windowFrameFollowing
    ;

windowFramePreceding
    : UNBOUNDED PRECEDING
    | DECIMAL PRECEDING
    | CURRENT ROW
    ;

windowFrameFollowing
    : UNBOUNDED FOLLOWING
    | DECIMAL FOLLOWING
    ;

createDatabaseOption:
    FILESTREAM ( databaseFilestreamOption (',' databaseFilestreamOption)* )
    | DEFAULT_LANGUAGE EQUAL ( id | STRING )
    | DEFAULT_FULLTEXT_LANGUAGE EQUAL ( id | STRING )
    | NESTED_TRIGGERS EQUAL ( OFF | ON )
    | TRANSFORM_NOISE_WORDS EQUAL ( OFF | ON )
    | TWO_DIGIT_YEAR_CUTOFF EQUAL DECIMAL
    | DB_CHAINING ( OFF | ON )
    | TRUSTWORTHY ( OFF | ON )
    ;

databaseFilestreamOption:
     LR_BRACKET
     (
         ( NON_TRANSACTED_ACCESS EQUAL ( OFF | READ_ONLY | FULL ) )
         |
         ( DIRECTORY_NAME EQUAL STRING )
     )
     RR_BRACKET
    ;

databaseFileSpec:
    fileGroup | fileSpec;

fileGroup:
     FILEGROUP id
     ( CONTAINS FILESTREAM )?
     ( DEFAULT )?
     ( CONTAINS MEMORY_OPTIMIZED_DATA )?
     fileSpec ( ',' fileSpec )*
    ;
fileSpec
    : LR_BRACKET
      NAME EQUAL ( id | STRING ) ','?
      FILENAME EQUAL file = STRING ','?
      ( SIZE EQUAL fileSize ','? )?
      ( MAXSIZE EQUAL (fileSize | UNLIMITED )','? )?
      ( FILEGROWTH EQUAL fileSize ','? )?
      RR_BRACKET
    ;


// Primitive.
entityName
      : (server=id '.' database=id '.'  schema=id   '.'
      |              database=id '.' (schema=id)? '.'
      |                               schema=id   '.')? table=id
    ;


entityNameForAzureDw
      : schema=id
      | schema=id '.' objectName=id
      ;

entityNameForParallelDw
      : schemaDatabase=id
      | schema=id '.' objectName=id
      ;

fullTableName
    : (server=id '.' database=id '.'  schema=id   '.'
      |              database=id '.' (schema=id)? '.'
      |                               schema=id   '.')? table=id
    ;

tableName
    : (database=id '.' (schema=id)? '.' | schema=id '.')? table=id
    | (database=id '.' (schema=id)? '.' | schema=id '.')? BLOCKING_HIERARCHY
    ;

simpleName
    : (schema=id '.')? name=id
    ;

funcProcNameSchema
    : ((schema=id) '.')? procedure=id
    ;

funcProcNameDatabaseSchema
    : funcProcNameSchema
    | (database=id '.' (schema=id)? '.')? procedure=id
    ;

funcProcNameServerDatabaseSchema
    : funcProcNameDatabaseSchema
    | (server=id '.' database=id '.' (schema=id)? '.')? procedure=id
    ;

ddlObject
    : fullTableName
    | LOCAL_ID
    ;
/*  There are some RESERVED WORDS that can be column names */
fullColumnName
    : (tableName '.')? columnName=id
    | (tableName '.')? COMPATIBILITY_LEVEL
    | (tableName '.')? STATUS
    | (tableName '.')? QUOTED_IDENTIFIER
    | (tableName '.')? ARITHABORT
    | (tableName '.')? ANSI_WARNINGS
    | (tableName '.')? ANSI_PADDING
    | (tableName '.')? ANSI_NULLS

    ;

columnNameListWithOrder
    : id (ASC | DESC)? (',' id (ASC | DESC)?)*
    ;

columnNameList
    : id (',' id)*
    ;

cursorName
    : id
    | LOCAL_ID
    ;

onOff
    : ON
    | OFF
    ;

clustered
    : CLUSTERED
    | NONCLUSTERED
    ;

nullNotnull
    : NOT? NULL
    ;

nullOrDefault
    :(nullNotnull | DEFAULT constantExpression (WITH VALUES)?)
    ;

scalarFunctionName
    : funcProcNameServerDatabaseSchema
    | RIGHT
    | LEFT
    | BINARY_CHECKSUM
    | ABS
    | ASCII
    | CEILING
    | CHAR
    | CHARINDEX
    | CHECKSUM
    | DATALENGTH
    | DAY
    | FLOOR
    | ISDATE
    | ISNUMERIC
    | LEN
    | LOWER
    | LTRIM
    | MONTH
    | NCHAR
    | PATINDEX
    | RAND
    | REPLACE
    | ROUND
    | RTRIM
    | SIGN
    | SPACE
    | STR
    | SUBSTRING
    | UPPER
    | USER_NAME
    | YEAR
    ;

beginConversationTimer
    : BEGIN CONVERSATION TIMER '(' LOCAL_ID ')' TIMEOUT '=' time ';'?
    ;

beginConversationDialog
    : BEGIN DIALOG (CONVERSATION)? dialogHandle=LOCAL_ID
      FROM SERVICE initiatorServiceName=serviceName
      TO SERVICE targetServiceName=serviceName (',' serviceBrokerGuid=STRING)?
      ON CONTRACT contractName
      (WITH
        ((RELATED_CONVERSATION | RELATED_CONVERSATION_GROUP) '=' LOCAL_ID ','?)?
        (LIFETIME '=' (DECIMAL | LOCAL_ID) ','?)?
        (ENCRYPTION '=' (ON | OFF))? )?
      ';'?
    ;

contractName
    : (id | expression)
    ;

serviceName
    : (id | expression)
    ;

endConversation
    : END CONVERSATION conversationHandle=LOCAL_ID ';'?
      (WITH (ERROR '=' faliureCode=(LOCAL_ID | STRING) DESCRIPTION '=' failureText=(LOCAL_ID | STRING))? CLEANUP? )?
    ;

waitforConversation
    : WAITFOR? '(' getConversation ')' (','? TIMEOUT timeout=time)? ';'?
    ;

getConversation
    :GET CONVERSATION GROUP conversationGroupId=(STRING | LOCAL_ID) FROM queue=queueId ';'?
    ;

queueId
    : (databaseName=id '.' schemaName=id '.' name=id)
    | id
    ;

sendConversation
    : SEND ON CONVERSATION conversationHandle=(STRING | LOCAL_ID)
      MESSAGE TYPE messageTypeName=expression
      ('(' messageBodyExpression=(STRING | LOCAL_ID) ')' )?
      ';'?
    ;

// https://msdn.microsoft.com/en-us/library/ms187752.aspx
// TODO: implement runtime check or add new tokens.
dataType
    /*: BIGINT
    | BINARY '(' DECIMAL ')'
    | BIT
    | CHAR '(' DECIMAL ')'
    | DATE
    | DATETIME
    | DATETIME2
    | DATETIMEOFFSET '(' DECIMAL ')'
    | DECIMAL '(' DECIMAL ',' DECIMAL ')'
    | DOUBLE PRECISION?
    | FLOAT
    | GEOGRAPHY
    | GEOMETRY
    | HIERARCHYID
    | IMAGE
    | INT
    | MONEY
    | NCHAR '(' DECIMAL ')'
    | NTEXT
    | NUMERIC '(' DECIMAL ',' DECIMAL ')'
    | NVARCHAR '(' DECIMAL | MAX ')'
    | REAL
    | SMALLDATETIME
    | SMALLINT
    | SMALLMONEY
    | SQL_VARIANT
    | TEXT
    | TIME '(' DECIMAL ')'
    | TIMESTAMP
    | TINYINT
    | UNIQUEIDENTIFIER
    | VARBINARY '(' DECIMAL | MAX ')'
    | VARCHAR '(' DECIMAL | MAX ')'
    | XML*/
    : id IDENTITY? ('(' (DECIMAL | MAX) (',' DECIMAL)? ')')?
    | DOUBLE PRECISION?
    | INT
    | TINYINT
    | SMALLINT
    | BIGINT
    ;

defaultValue
    : NULL
    | DEFAULT
    | constant
    ;

// https://msdn.microsoft.com/en-us/library/ms179899.aspx
constant
    : STRING // string, datetime or uniqueidentifier
    | BINARY
    | sign? DECIMAL
    | sign? (REAL | FLOAT)  // float or decimal
    | sign? dollar='$' (DECIMAL | FLOAT)       // money
    ;

sign
    : '+'
    | '-'
    ;

// https://msdn.microsoft.com/en-us/library/ms175874.aspx
id
    : simpleId
    | DOUBLE_QUOTE_ID
    | SQUARE_BRACKET_ID
    ;

simpleId
    : ID
    | ABSOLUTE
    | ACCENT_SENSITIVITY
    | ACTION
    | ACTIVATION
    | ACTIVE
    | ADDRESS
    | AES_128
    | AES_192
    | AES_256
    | AFFINITY
    | AFTER
    | AGGREGATE
    | ALGORITHM
    | ALLOW_ENCRYPTED_VALUE_MODIFICATIONS
    | ALLOW_SNAPSHOT_ISOLATION
    | ALLOWED
    | ANSI_NULL_DEFAULT
    | ANSI_NULLS
    | ANSI_PADDING
    | ANSI_WARNINGS
    | APPLICATION_LOG
    | APPLY
    | ARITHABORT
    | ASSEMBLY
    | AUDIT
    | AUDIT_GUID
    | AUTO
    | AUTO_CLEANUP
    | AUTO_CLOSE
    | AUTO_CREATE_STATISTICS
    | AUTO_SHRINK
    | AUTO_UPDATE_STATISTICS
    | AUTO_UPDATE_STATISTICS_ASYNC
    | AVAILABILITY
    | AVG
    | BACKUP_PRIORITY
    | BEGIN_DIALOG
    | BIGINT
    | BINARY_BASE64
    | BINARY_CHECKSUM
    | BINDING
    | BLOB_STORAGE
    | BROKER
    | BROKER_INSTANCE
    | BULK_LOGGED
    | CALLED
    | CALLER
    | CAP_CPU_PERCENT
    | CAST
    | CATALOG
    | CATCH
    | CHANGE_RETENTION
    | CHANGE_TRACKING
    | CHECKSUM
    | CHECKSUM_AGG
    | CLEANUP
    | COLLECTION
    | COLUMN_MASTER_KEY
    | COMMITTED
    | COMPATIBILITY_LEVEL
    | CONCAT
    | CONCAT_NULL_YIELDS_NULL
    | CONTENT
    | CONTROL
    | COOKIE
    | COUNT
    | COUNT_BIG
    | COUNTER
    | CPU
    | CREATE_NEW
    | CREATION_DISPOSITION
    | CREDENTIAL
    | CRYPTOGRAPHIC
    | CURSOR_CLOSE_ON_COMMIT
    | CURSOR_DEFAULT
    | DATA_COMPRESSION
    | DATE_CORRELATION_OPTIMIZATION
    | DATEADD
    | DATEDIFF
    | DATENAME
    | DATEPART
    | DAYS
    | DB_CHAINING
    | DB_FAILOVER
    | DECRYPTION
    | DEFAULT_DOUBLE_QUOTE
    | DEFAULT_FULLTEXT_LANGUAGE
    | DEFAULT_LANGUAGE
    | DELAY
    | DELAYED_DURABILITY
    | DELETED
    | DENSE_RANK
    | DEPENDENTS
    | DES
    | DESCRIPTION
    | DESX
    | DHCP
    | DIALOG
    | DIRECTORY_NAME
    | DISABLE
    | DISABLE_BROKER
    | DISABLED
    | DISK_DRIVE
    | DOCUMENT
    | DYNAMIC
    | EMERGENCY
    | EMPTY
    | ENABLE
    | ENABLE_BROKER
    | ENCRYPTED_VALUE
    | ENCRYPTION
    | ENDPOINT_URL
    | ERROR_BROKER_CONVERSATIONS
    | EVENTDATA
    | EXCLUSIVE
    | EXECUTABLE
    | EXIST
    | EXPAND
    | EXPIRY_DATE
    | EXPLICIT
    | FAIL_OPERATION
    | FAILOVER_MODE
    | FAILURE
    | FAILURE_CONDITION_LEVEL
    | FAST
    | FAST_FORWARD
    | FILEGROUP
    | FILEGROWTH
    | FILENAME
    | FILEPATH
    | FILESTREAM
    | FILLFACTOR
    | FILTER
    | FIRST
    | FIRST_VALUE
    | FOLLOWING
    | FORCE
    | FORCE_FAILOVER_ALLOW_DATA_LOSS
    | FORCED
    | FORCESEEK
    | FORMAT
    | FORWARD_ONLY
    | FULLSCAN
    | FULLTEXT
    | GB
    | GETDATE
    | GETUTCDATE
    | GLOBAL
    | GO
    | GROUP_MAX_REQUESTS
    | GROUPING
    | GROUPING_ID
    | HADR
    | HASH
    | HEALTH_CHECK_TIMEOUT
    | HIGH
    | HONOR_BROKER_PRIORITY
    | HOURS
    | IDENTITY_VALUE
    | IGNORE_NONCLUSTERED_COLUMNSTORE_INDEX
    | IMMEDIATE
    | IMPERSONATE
    | IMPORTANCE
    | INCREMENTAL
    | INIT
    | INITIATOR
    | INPUT
    | INSENSITIVE
    | INSERTED
    | INT
    | IP
    | ISOLATION
    | KB
    | KEEP
    | KEEPFIXED
    | KEY
    | KEY_SOURCE
    | KEYS
    | KEYSET
    | LAG
    | LAST
    | LAST_VALUE
    | LEAD
    | LEVEL
    | LIST
    | LISTENER
    | LISTENER_URL
    | LOB_COMPACTION
    | LOCAL
    | LOCATION
    | LOCK
    | LOCK_ESCALATION
    | LOGIN
    | LOOP
    | LOW
    | MANUAL
    | MARK
    | MASTER
    | MATERIALIZED
    | MAX
    | MAX_CPU_PERCENT
    | MAX_DOP
    | MAX_FILES
    | MAX_IOPS_PER_VOLUME
    | MAX_MEMORY
    | MAX_MEMORY_PERCENT
    | MAX_PROCESSES
    | MAX_QUEUE_READERS
    | MAX_ROLLOVER_FILES
    | MAXDOP
    | MAXRECURSION
    | MAXSIZE
    | MB
    | MEDIUM
    | MEMORY_OPTIMIZED_DATA
    | MESSAGE
    | MIN
    | MIN_ACTIVE_ROWVERSION
    | MIN_CPU_PERCENT
    | MIN_IOPS_PER_VOLUME
    | MIN_MEMORY_PERCENT
    | MINUTES
    | MIRROR_ADDRESS
    | MIXED_PAGE_ALLOCATION
    | MODE
    | MODIFY
    | MOVE
    | MULTI_USER
    | NAME
    | NESTED_TRIGGERS
    | NEW_ACCOUNT
    | NEW_BROKER
    | NEW_PASSWORD
    | NEXT
    | NO
    | NO_TRUNCATE
    | NO_WAIT
    | NOCOUNT
    | NODES
    | NOEXPAND
    | NON_TRANSACTED_ACCESS
    | NORECOMPUTE
    | NORECOVERY
    | NOWAIT
    | NTILE
    | NUMANODE
    | NUMBER
    | NUMERIC_ROUNDABORT
    | OBJECT
    | OFFLINE
    | OFFSET
    | OFFSETS
    | OLD_ACCOUNT
    | ONLINE
    | ONLY
    | OPEN_EXISTING
    | OPTIMISTIC
    | OPTIMIZE
    | OUT
    | OUTPUT
    | OWNER
    | PAGE
    | PAGE_VERIFY
    | PARAMETERIZATION
    | PARTITION
    | PARTITIONS
    | PARTNER
    | PATH
    | POISON_MESSAGE_HANDLING
    | POOL
    | PORT
    | PRECEDING
    | PRIMARY_ROLE
    | PRIOR
    | PRIORITY
    | PRIORITY_LEVEL
    | PRIVATE
    | PRIVATE_KEY
    | PRIVILEGES
    | PROCEDURE_NAME
    | PROPERTY
    | PROVIDER
    | PROVIDER_KEY_NAME
    | PUBLIC
    | QUERY
    | QUEUE
    | QUEUE_DELAY
    | QUOTED_IDENTIFIER
    | R
    | RANGE
    | RANK
    | RAW
    | RC2
    | RC4
    | RC4_128
    | READ_COMMITTED_SNAPSHOT
    | READ_ONLY
    | READ_ONLY_ROUTING_LIST
    | READ_WRITE
    | READONLY
    | REBUILD
    | RECEIVE
    | RECOMPILE
    | RECOVERY
    | RECURSIVE_TRIGGERS
    | RELATIVE
    | REMOTE
    | REMOTE_SERVICE_NAME
    | REMOVE
    | REORGANIZE
    | REPEATABLE
    | REPLICA
    | REQUEST_MAX_CPU_TIME_SEC
    | REQUEST_MAX_MEMORY_GRANT_PERCENT
    | REQUEST_MEMORY_GRANT_TIMEOUT_SEC
    | REQUIRED_SYNCHRONIZED_SECONDARIES_TO_COMMIT
    | RESERVE_DISK_SPACE
    | RESOURCE
    | RESOURCE_MANAGER_LOCATION
    | RESTRICTED_USER
    | RETENTION
    | RETURN
    | RETURNS
    | ROBUST
    | ROOT
    | ROUTE
    | ROW
    | ROW_NUMBER
    | ROWCOUNT
    | ROWGUID
    | ROWS
    | SAFETY
    | SAMPLE
    | SCHEMABINDING
    | SCOPED
    | SCROLL
    | SCROLL_LOCKS
    | SEARCH
    | SECONDARY
    | SECONDARY_ONLY
    | SECONDARY_ROLE
    | SECONDS
    | SECRET
    | SECURITY_LOG
    | SEEDING_MODE
    | SELF
    | SEMI_SENSITIVE
    | SEND
    | SENT
    | SERIALIZABLE
    | SERVER
    | SESSION_TIMEOUT
    | SETERROR
    | SHARE
    | SHOWPLAN
    | SID
    | SID
    | SIGNATURE
    | SIMPLE
    | SINGLE_USER
    | SIZE
    | SMALLINT
    | SNAPSHOT
    | SOURCE
    | SPATIAL_WINDOW_MAX_CELLS
    | STANDBY
    | START
    | START_DATE
    | STATE
    | STATIC
    | STATS_STREAM
    | STATUS
    | STDEV
    | STDEVP
    | STOPLIST
    | STUFF
    | SUBJECT
    | SUM
    | SUSPEND
    | SYMMETRIC
    | SYNCHRONOUS_COMMIT
    | SYNONYM
    | TAKE
    | TARGET
    | TARGET_RECOVERY_TIME
    | TB
    | TEXTIMAGE_ON
    | THROW
    | TIES
    | TIME
    | TIMEOUT
    | TIMER
    | TINYINT
    | TORN_PAGE_DETECTION
    | TRANSFORM_NOISE_WORDS
    | TRIPLE_DES
    | TRIPLE_DES_3KEY
    | TRUSTWORTHY
    | TRY
    | TSQL
    | TWO_DIGIT_YEAR_CUTOFF
    | TYPE
    | TYPE_WARNING
    | UNBOUNDED
    | UNCOMMITTED
    | UNKNOWN
    | UNLIMITED
    | USING
    | VALID_XML
    | VALIDATION
    | VALUE
    | VAR
    | VARP
    | VIEW_METADATA
    | VIEWS
    | WAIT
    | WELL_FORMED_XML
    | WORK
    | WORKLOAD
    | XML
    | XMLNAMESPACES
    ;

// https://msdn.microsoft.com/en-us/library/ms188074.aspx
// Spaces are allowed for comparison operators.
comparisonOperator
    : '=' | '>' | '<' | '<' '=' | '>' '=' | '<' '>' | '!' '=' | '!' '>' | '!' '<'
    ;

assignmentOperator
    : '+=' | '-=' | '*=' | '/=' | '%=' | '&=' | '^=' | '|='
    ;

fileSize
    : DECIMAL( KB | MB | GB | TB | '%' )?
    ;
