// Java template with classes, methods, collections, exceptions and loops.
@words msg   = hello, world, resrap, generated, code, java, compiler, grammar, ready, done, starting, started, finished, complete, loading, saving, parsing, rendering, building, running, idle, busy, queued, cached, updated, created, deleted, inserted, found, missing, valid, invalid, empty, success, failure, warning, error, retry, pending, active, online, offline, local, remote, fast, slow, secure, public, private, shared, custom, default, primary, secondary, backup, hello_world, build_started, build_finished, parse_error, file_not_found, access_denied, operation_complete, invalid_input, retry_later, connection_lost, cache_miss, cache_hit, request_failed, task_complete
@words digit = 1, 2, 3, 4, 5, 6, 7, 8, 9
@words prefix = compute, process, update, handle, build, check, render, collect, parse, encode, decode, merge, split, find, apply, visit, create, destroy, allocate, release, load, store, fetch, send, receive, open, close, read, write, validate, normalize, transform, convert, serialize, deserialize, filter, reduce, scan, sort, search, insert, remove, replace, compare, calculate, analyze, summarize, generate, evaluate, execute, schedule, dispatch, resolve, connect, disconnect, flush, drain, inspect, measure
@words label  = alpha, beta, gamma, delta, epsilon, zeta, eta, theta, iota, kappa, lambda, mu, nu, xi, omicron, pi, rho, sigma, tau, upsilon, phi, chi, psi, omega, red, green, blue, amber, cyan, magenta, silver, golden
@words entity = user, account, order, item, record, event, session, invoice, payment, product, project, task, ticket, message, cache, buffer, queue, worker, request, response, config, profile, report, metric, log, file, document, group, member, role, permission, setting, address, country, region, category, tag, comment, review, shipment, inventory, warehouse, employee, department, customer, supplier
@words proper = User, Account, Order, Item, Record, Event, Session, Invoice, Payment, Product, Project, Task, Ticket, Message, Cache, Buffer, Queue, Worker, Request, Response, Config, Profile, Report, Metric, Log, File, Document, Group, Member, Role, Permission, Setting, Address, Country, Region, Category, Tag, Comment, Review, Shipment, Inventory, Warehouse, Employee, Department, Customer, Supplier

program: 'import java.util.*;\n\npublic class Main {\n' main member^ '}\n';

main: '    public static void main(String[] args) {\n        List<Integer> values = new ArrayList<>();\n        values.add(5);\n        values.add(2);\n        values.add(8);\n        Collections.sort(values);\n        int total = 0;\n        for (int value : values) {\n            total += value;\n        }\n        System.out.println("start: " + values + " " + total);\n    }\n\n';

member:
      method<0.52>
    | staticmethod<0.24>
    | nestedclass<0.12>
    | interface<0.12>;

method: '    private static int ' fname '(int seed) {\n' methodlocals statement*<0.9> '        return total;\n    }\n\n';

staticmethod: '    static int ' fname '(int seed) {\n' methodlocals statement*<0.9> '        return total + seed;\n    }\n\n';

nestedclass: '    static class ' tname ' {\n        int id;\n        String name = "item";\n        double value = 1.5;\n        int score() { return id + name.length(); }\n    }\n\n';

interface: '    interface ' tname ' {\n        String describe();\n        int score(int seed);\n    }\n\n';

methodlocals: '        int count = seed;\n        int total = 0;\n        List<Integer> values = new ArrayList<>();\n        Map<String, Integer> lookup = new HashMap<>();\n        String label = "' label '";\n';

statement:
      assign<0.18>
    | printstmt<0.12>
    | ifstmt<0.14>
    | forstmt<0.11>
    | enhancedfor<0.11>
    | whilestmt<0.08>
    | switchstmt<0.10>
    | liststmt<0.10>
    | mapstmt<0.08>
    | trycatch<0.08>;

simple:
      assign<0.40>
    | printstmt<0.30>
    | liststmt<0.30>;

assign: '        count += ' number ';\n' | '        total += count * ' number ';\n' | '        total = total * 2 + count;\n';

printstmt: '        System.out.println(label + ": ' msg ' " + count + " " + total);\n';

ifstmt: '        if (count > ' number ') {\n' simple+<0.6> '            total += ' number ';\n        } else {\n            total -= 1;\n        }\n';

forstmt: '        for (int i = 0; i < ' number '; i++) {\n            total += i;\n' simple*<0.5> '        }\n';

enhancedfor: '        for (int value : values) {\n            total += value;\n        }\n';

whilestmt: '        while (count < ' number ') {\n            count += 1;\n            total += count;\n        }\n';

switchstmt: '        switch (count % 3) {\n            case 0: total += 1; break;\n            case 1: total += 2; break;\n            default: total += 3; break;\n        }\n';

liststmt: '        values.add(count + ' number ');\n' | '        total += values.size();\n' | '        Collections.sort(values);\n';

mapstmt: '        lookup.put(label, count);\n' | '        total += lookup.getOrDefault(label, 0);\n';

trycatch: '        try {\n            total += Integer.parseInt("' number '");\n        } catch (NumberFormatException e) {\n            total += 1;\n        }\n';

fname: prefix '_' entity ('_' digit)?;
tname: 'Type' proper ('_' digit)?;
number: digit digit?;
