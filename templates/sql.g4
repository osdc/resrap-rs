// SQL template with DDL, DML, joins, grouping, ordering and transactions.
@words table  = users, items, orders, events, records, accounts, products, sessions, invoices, payments, customers, suppliers, categories, tags, comments, reviews, shipments, inventory, warehouses, employees, departments, projects, tasks, tickets, messages, notifications, subscriptions, transactions, refunds, coupons, addresses, countries, regions, reports, audit_logs, metrics, logs, files, documents, images, groups, members, roles, permissions, settings, profiles, activities, downloads, uploads
@words column = id, name, value, status, total, created_at, updated_at, email, amount, quantity, price, cost, currency, description, title, slug, category_id, user_id, order_id, sku, stock, active, verified, priority, score, rating, count, balance, due_date, start_date, end_date, phone, country, city, region, notes, metadata, payload, version, checksum, source, destination, duration, attempts, error_code, external_id, parent_id, position, weight, discount, tax, shipping, billing, is_deleted
@words text   = hello, world, resrap, generated, code, sql, database, query, active, pending, archived, draft, published, hidden, verified, disabled, pending_review, in_progress, completed, cancelled, refunded, shipped, delivered, processing, approved, rejected, failed, succeeded, retry, retry_later, invalid_input, not_found, access_denied, rate_limited, hello_world, build_started, task_complete
@words digit  = 1, 2, 3, 4, 5, 6, 7, 8, 9

program: statement^;

statement:
      selectstmt<0.28>
    | joinstmt<0.18>
    | insertstmt<0.16>
    | createstmt<0.14>
    | updatestmt<0.10>
    | deletestmt<0.06>
    | indexstmt<0.04>
    | transaction<0.04>;

selectstmt: 'SELECT ' selectlist ' FROM ' table (' WHERE ' condition)? (' ORDER BY ' column (' ASC' | ' DESC'))? (' LIMIT ' number)? ';\n';

joinstmt: 'SELECT ' table '.' column ', ' table '.' column ' FROM ' table ' JOIN ' table ' ON ' table '.id = ' table '.id WHERE ' condition ';\n';

selectlist: '*' | column (', ' column)*;

insertstmt: 'INSERT INTO ' table ' (' columnlist ') VALUES (' valuelist ')' (';\n' | ' RETURNING id;\n');

createstmt: 'CREATE TABLE ' table ' (\n    id INTEGER PRIMARY KEY,\n    name TEXT NOT NULL,\n    value INTEGER DEFAULT ' number ',\n    status REAL\n);\n';

updatestmt: 'UPDATE ' table ' SET ' column ' = ' value ', ' column ' = ' value ' WHERE ' condition ';\n';

deletestmt: 'DELETE FROM ' table ' WHERE ' condition ';\n';

indexstmt: 'CREATE INDEX idx_' table '_' column ' ON ' table ' (' column ');\n';

transaction: 'BEGIN;\n' | 'COMMIT;\n' | 'ROLLBACK;\n';

columnlist: column (', ' column)*;

valuelist: value (', ' value)*;

value: integer | float | string | boolean | 'NULL';

condition: column comparator value (logicalop column comparator value)*;

comparator: ' = ' | ' < ' | ' > ' | ' <= ' | ' >= ' | ' != ' | ' LIKE ';

logicalop: ' AND ' | ' OR ';

string: '\'' text '\'';

boolean: 'TRUE' | 'FALSE';

integer: digit digit?;

float: integer '.' integer;

number: digit digit?;
