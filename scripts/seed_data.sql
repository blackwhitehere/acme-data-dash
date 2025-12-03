-- Secrets
INSERT INTO secrets (key, value) VALUES ('prod_db_password', 's3cr3t_p4ssw0rd');
INSERT INTO secrets (key, value) VALUES ('staging_api_key', 'abc-123-xyz');

-- Connection Profiles
INSERT INTO connection_profiles (name, driver, connection_string_template, secret_ref) 
VALUES ('Production DB', 'postgres', 'host=prod.example.com user=admin password={{PASSWORD}} dbname=customers', 'prod_db_password');

INSERT INTO connection_profiles (name, driver, connection_string_template, secret_ref) 
VALUES ('Local SQLite', 'sqlite', 'sqlite:local_dev.db', NULL);

INSERT INTO connection_profiles (name, driver, connection_string_template, secret_ref) 
VALUES ('Staging API', 'http', 'https://api.staging.example.com/v1?key={{PASSWORD}}', 'staging_api_key');

-- Example Check Results (History)
INSERT INTO check_results (check_id, status, message, details, executed_at)
VALUES ('example_check', 'Success', 'Check passed successfully', '{"rows": 5}', datetime('now', '-1 hour'));

INSERT INTO check_results (check_id, status, message, details, executed_at)
VALUES ('example_check', 'Failure', 'Connection timeout', '{"error": "timeout"}', datetime('now', '-2 hours'));
