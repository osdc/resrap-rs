// Python template with functions, classes, comprehensions, exceptions and loops.
@words msg   = hello, world, resrap, generated, code, python, compiler, grammar, ready, done, starting, started, finished, complete, loading, saving, parsing, rendering, building, running, idle, busy, queued, cached, updated, created, deleted, inserted, found, missing, valid, invalid, empty, success, failure, warning, error, retry, pending, active, online, offline, local, remote, fast, slow, secure, public, private, shared, custom, default, primary, secondary, backup, hello_world, build_started, build_finished, parse_error, file_not_found, access_denied, operation_complete, invalid_input, retry_later, connection_lost, cache_miss, cache_hit, request_failed, task_complete
@words digit = 1, 2, 3, 4, 5, 6, 7, 8, 9
@words prefix = compute, process, update, handle, build, check, render, collect, parse, encode, decode, merge, split, find, apply, visit, create, destroy, allocate, free, load, store, fetch, send, receive, open, close, read, write, validate, normalize, transform, convert, serialize, deserialize, filter, reduce, scan, sort, search, insert, remove, replace, compare, calculate, analyze, summarize, generate, evaluate, execute, schedule, dispatch, resolve, connect, disconnect, flush, drain, inspect, measure
@words label  = alpha, beta, gamma, delta, epsilon, zeta, eta, theta, iota, kappa, lambda, mu, nu, xi, omicron, pi, rho, sigma, tau, upsilon, phi, chi, psi, omega, red, green, blue, amber, cyan, magenta, silver, golden
@words entity = user, account, order, item, record, event, session, invoice, payment, product, project, task, ticket, message, cache, buffer, queue, worker, request, response, config, profile, report, metric, log, file, document, group, member, role, permission, setting, address, country, region, category, tag, comment, review, shipment, inventory, warehouse, employee, department, customer, supplier
@words proper = User, Account, Order, Item, Record, Event, Session, Invoice, Payment, Product, Project, Task, Ticket, Message, Cache, Buffer, Queue, Worker, Request, Response, Config, Profile, Report, Metric, Log, File, Document, Group, Member, Role, Permission, Setting, Address, Country, Region, Category, Tag, Comment, Review, Shipment, Inventory, Warehouse, Employee, Department, Customer, Supplier

program: imports main definition^ footer;

imports: 'import math\nimport random\n\n';

main: 'def main():\n    values = [3, 1, 4, 1, 5]\n    values.sort()\n    total = sum(values)\n    print("start:", values, total)\n    print("sqrt:", round(math.sqrt(total), 2))\n    print("random:", random.randint(1, 9))\n\n';

footer: '\nif __name__ == "__main__":\n    main()\n';

definition:
      function<0.60>
    | classdecl<0.40>;

function: 'def ' fname '(seed):\n' functionlocals statement*<0.9> '    return total\n\n';

classdecl: 'class ' tname ':\n    def __init__(self, seed):\n        self.seed = seed\n        self.label = "' label '"\n\n    def ' methodname '(self, amount):\n        count = self.seed + amount\n        total = 0\n        values = [count, amount, self.seed]\n        for value in values:\n            total += value\n        return total + len(self.label)\n\n';

functionlocals: '    count = seed\n    total = 0\n    values = []\n    lookup = {}\n    label = "' label '"\n';

statement:
      assign<0.18>
    | printstmt<0.12>
    | ifstmt<0.14>
    | forstmt<0.12>
    | foreach<0.10>
    | whilestmt<0.08>
    | trycatch<0.10>
    | liststmt<0.10>
    | dictstmt<0.06>
    | comprehension<0.10>;

nested:
      '        total += count * ' number '\n'<0.35>
    | '        values.append(count + ' number ')\n'<0.30>
    | '        print(label, count, total)\n'<0.35>;

assign: '    count += ' number '\n' | '    total += count * ' number '\n' | '    total = total * 2 + count\n';

printstmt: '    print("' msg '", label, count, total)\n';

ifstmt: '    if count > ' number ':\n' nested*<0.6> '        total += ' number '\n    else:\n        total -= 1\n';

forstmt: '    for i in range(' number '):\n        total += i\n' nested*<0.5>;

foreach: '    for value in values:\n        total += value\n';

whilestmt: '    while count < ' number ':\n        count += 1\n        total += count\n' nested*<0.4>;

trycatch: '    try:\n        total += int("' number '")\n    except ValueError:\n        total += 1\n';

liststmt: '    values.append(count + ' number ')\n' | '    total += len(values)\n';

dictstmt: '    lookup[label] = count\n' | '    total += lookup.get(label, 0)\n';

comprehension: '    total += sum(x * x for x in range(' number '))\n' | '    total += len([x for x in values if x > ' number '])\n';

fname: prefix '_' entity ('_' digit)?;
tname: 'Type' proper ('_' digit)?;
methodname: 'score_' entity ('_' digit)?;
number: digit digit?;
