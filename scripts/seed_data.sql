-- Secrets
INSERT INTO secrets (key, value) VALUES ('postgres_password', 's3cr3t_p4ssw0rd');
INSERT INTO secrets (key, value) VALUES ('mysql_password', 'my5ql_p4ss');
INSERT INTO secrets (key, value) VALUES ('mssql_password', 'mssql_s3cr3t');
INSERT INTO secrets (key, value) VALUES ('oracle_password', 'or4cl3_p4ss');
INSERT INTO secrets (key, value) VALUES ('example_api_key', 'abc-123-xyz-api-key');

-- Database Connection Profiles
-- SQLite
INSERT INTO connection_profiles (name, driver, connection_string_template, connection_type, secret_ref) 
VALUES ('Local SQLite', 'sqlite', 'DSN=SQLiteDB;Database=/var/data/local.db', 'database', NULL);

-- DuckDB
INSERT INTO connection_profiles (name, driver, connection_string_template, connection_type, secret_ref) 
VALUES ('DuckDB Analytics', 'duckdb', 'DSN=DuckDB;Database=/var/data/analytics.duckdb;ReadOnly=false', 'database', NULL);

-- PostgreSQL
INSERT INTO connection_profiles (name, driver, connection_string_template, connection_type, secret_ref) 
VALUES ('PostgreSQL DB', 'postgresql', 'DSN=PostgresDB;Driver=/usr/local/lib/psqlodbcw.so;Server=db.example.com;Port=5432;Database=customers;UID=admin;PWD={{PASSWORD}};SSLMode=require', 'database', 'postgres_password');

-- MySQL
INSERT INTO connection_profiles (name, driver, connection_string_template, connection_type, secret_ref) 
VALUES ('MySQL DB', 'mysql', 'DSN=MySQLDB;Driver=/usr/local/lib/libmyodbc8w.so;Server=db.example.com;Port=3306;Database=warehouse;UID=etl_user;PWD={{PASSWORD}};CharSet=utf8mb4', 'database', 'mysql_password');

-- Microsoft SQL Server
INSERT INTO connection_profiles (name, driver, connection_string_template, connection_type, secret_ref) 
VALUES ('SQL Server DB', 'sqlserver', 'DSN=MSSQLDB;Driver=/opt/microsoft/msodbcsql18/lib64/libmsodbcsql-18.3.so;Server=db.example.com;Port=1433;Database=Enterprise;UID=sa;PWD={{PASSWORD}};Encrypt=yes;TrustServerCertificate=no', 'database', 'mssql_password');

-- Oracle Database
INSERT INTO connection_profiles (name, driver, connection_string_template, connection_type, secret_ref) 
VALUES ('Oracle DB', 'oracle', 'DSN=OracleDB;Driver=/opt/oracle/instantclient_21_1/libsqora.so.21.1;Server=db.example.com;Port=1521;ServiceName=ORCL;UID=system;PWD={{PASSWORD}}', 'database', 'oracle_password');

-- API Connection Profiles
INSERT INTO connection_profiles (name, driver, connection_string_template, connection_type, secret_ref) 
VALUES ('Petstore API', 'openapi', 'https://petstore3.swagger.io/api/v3/openapi.json', 'api', 'example_api_key');

INSERT INTO connection_profiles (name, driver, connection_string_template, connection_type, secret_ref) 
VALUES ('JSONPlaceholder API', 'openapi', 'https://jsonplaceholder.typicode.com/', 'api', NULL);

INSERT INTO connection_profiles (name, driver, connection_string_template, connection_type, secret_ref) 
VALUES ('GitHub API', 'openapi', 'https://raw.githubusercontent.com/github/rest-api-description/main/descriptions/api.github.com/api.github.com.json', 'api', NULL);

-- File Connection Profiles
INSERT INTO connection_profiles (name, driver, connection_string_template, connection_type, secret_ref) 
VALUES ('Sample Text File', 'file', '/data/sample.txt', 'file', NULL);

INSERT INTO connection_profiles (name, driver, connection_string_template, connection_type, secret_ref) 
VALUES ('CSV Data Export', 'file', '/exports/daily_report.csv', 'file', NULL);

-- Data Sources (binding connections to secrets)
INSERT INTO data_sources (name, connection_name, secret_key, is_valid)
VALUES ('PostgreSQL Production', 'PostgreSQL DB', 'postgres_password', 1);

INSERT INTO data_sources (name, connection_name, secret_key, is_valid)
VALUES ('MySQL Warehouse', 'MySQL DB', 'mysql_password', 1);

INSERT INTO data_sources (name, connection_name, secret_key, is_valid)
VALUES ('Local Analytics', 'DuckDB Analytics', 'postgres_password', 1);

-- Example Check Results (History)
INSERT INTO check_results (check_id, status, message, details, executed_at)
VALUES ('example_check', 'Success', 'Check passed successfully', '{"rows": 5}', datetime('now', '-1 hour'));

INSERT INTO check_results (check_id, status, message, details, executed_at)
VALUES ('example_check', 'Failure', 'Connection timeout', '{"error": "timeout"}', datetime('now', '-2 hours'));
