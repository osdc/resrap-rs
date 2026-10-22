// JavaScript template with functions, classes, arrays, maps and exceptions.
@words msg   = hello, world, resrap, generated, code, javascript, compiler, grammar, ready, done, starting, started, finished, complete, loading, saving, parsing, rendering, building, running, idle, busy, queued, cached, updated, created, deleted, inserted, found, missing, valid, invalid, empty, success, failure, warning, error, retry, pending, active, online, offline, local, remote, fast, slow, secure, public, private, shared, custom, default, primary, secondary, backup, hello_world, build_started, build_finished, parse_error, file_not_found, access_denied, operation_complete, invalid_input, retry_later, connection_lost, cache_miss, cache_hit, request_failed, task_complete
@words digit = 1, 2, 3, 4, 5, 6, 7, 8, 9
@words prefix = compute, process, update, handle, build, check, render, collect, parse, encode, decode, merge, split, find, apply, visit, create, destroy, allocate, release, load, store, fetch, send, receive, open, close, read, write, validate, normalize, transform, convert, serialize, deserialize, filter, reduce, scan, sort, search, insert, remove, replace, compare, calculate, analyze, summarize, generate, evaluate, execute, schedule, dispatch, resolve, connect, disconnect, flush, drain, inspect, measure
@words label  = alpha, beta, gamma, delta, epsilon, zeta, eta, theta, iota, kappa, lambda, mu, nu, xi, omicron, pi, rho, sigma, tau, upsilon, phi, chi, psi, omega, red, green, blue, amber, cyan, magenta, silver, golden
@words entity = user, account, order, item, record, event, session, invoice, payment, product, project, task, ticket, message, cache, buffer, queue, worker, request, response, config, profile, report, metric, log, file, document, group, member, role, permission, setting, address, country, region, category, tag, comment, review, shipment, inventory, warehouse, employee, department, customer, supplier
@words proper = User, Account, Order, Item, Record, Event, Session, Invoice, Payment, Product, Project, Task, Ticket, Message, Cache, Buffer, Queue, Worker, Request, Response, Config, Profile, Report, Metric, Log, File, Document, Group, Member, Role, Permission, Setting, Address, Country, Region, Category, Tag, Comment, Review, Shipment, Inventory, Warehouse, Employee, Department, Customer, Supplier

program: header main declaration^ '\nmain();\n';

header: '// generated JavaScript\n\n';

main: 'function main() {\n    const values = [5, 2, 8, 1];\n    values.sort((a, b) => a - b);\n    const total = values.reduce((a, b) => a + b, 0);\n    const lookup = new Map();\n    lookup.set("start", total);\n    console.log("start:", values, lookup.get("start"));\n}\n';

declaration:
      function<0.46>
    | asyncfunction<0.14>
    | classdecl<0.24>
    | constdecl<0.16>;

function: 'function ' fname '(seed) {\n' functionlocals statement*<0.9> '    return total;\n}\n\n';

asyncfunction: 'async function ' fname '(seed) {\n' functionlocals statement*<0.8> '    await Promise.resolve(count);\n    return total;\n}\n\n';

classdecl: 'class ' tname ' {\n    constructor(seed) {\n        this.seed = seed;\n        this.label = "' label '";\n    }\n\n    ' methodname '(amount) {\n' classbody '        return this.seed + amount;\n    }\n}\n\n';

constdecl: 'const ' cname ' = {\n    id: ' number ',\n    name: "' label '",\n    value: ' float ',\n    items: [1, 2, 3],\n    describe() {\n        return this.name + ":" + this.id;\n    }\n};\n\n';

functionlocals: '    let count = seed;\n    let total = 0;\n    const values = [];\n    const lookup = new Map();\n    const label = "' label '";\n';

classbody: '        let count = amount;\n        let total = this.seed;\n        const values = [];\n        const lookup = new Map();\n        const label = this.label;\n';

statement:
      assign<0.18>
    | printstmt<0.12>
    | ifstmt<0.14>
    | forstmt<0.12>
    | forof<0.10>
    | whilestmt<0.08>
    | switchstmt<0.10>
    | arraystmt<0.10>
    | mapstmt<0.08>
    | trycatch<0.08>;

simple:
      assign<0.40>
    | printstmt<0.30>
    | arraystmt<0.30>;

assign: '    count += ' number ';\n' | '    total += count * ' number ';\n' | '    total = total * 2 + count;\n';

printstmt: '    console.log(label, "' msg '", count, total);\n';

ifstmt: '    if (count > ' number ') {\n' simple+<0.6> '        total += ' number ';\n    } else {\n        total -= 1;\n    }\n';

forstmt: '    for (let i = 0; i < ' number '; i++) {\n        total += i;\n' simple*<0.5> '    }\n';

forof: '    for (const value of values) {\n        total += value;\n    }\n';

whilestmt: '    while (count < ' number ') {\n        count += 1;\n        total += count;\n    }\n';

switchstmt: '    switch (count % 3) {\n        case 0:\n            total += 1;\n            break;\n        case 1:\n            total += 2;\n            break;\n        default:\n            total += 3;\n            break;\n    }\n';

arraystmt: '    values.push(count + ' number ');\n' | '    total += values.length;\n' | '    values.sort((a, b) => a - b);\n';

mapstmt: '    lookup.set(label, count);\n' | '    total += lookup.get(label) || 0;\n';

trycatch: '    try {\n        if (count < 0) throw new Error("negative");\n        total += count;\n    } catch (error) {\n        total += ' number ';\n    }\n';

fname: prefix '_' entity ('_' digit)?;
tname: 'Type' proper ('_' digit)?;
cname: 'CONFIG_' entity ('_' digit)?;
methodname: 'score_' entity ('_' digit)?;
number: digit digit?;
float: number '.' number;
