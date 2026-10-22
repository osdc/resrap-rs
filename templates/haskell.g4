// Haskell template with functions, pattern matching, lists, data and type aliases.
@words msg   = hello, world, resrap, generated, code, haskell, compiler, grammar, ready, done, starting, started, finished, complete, loading, saving, parsing, rendering, building, running, idle, busy, queued, cached, updated, created, deleted, inserted, found, missing, valid, invalid, empty, success, failure, warning, error, retry, pending, active, online, offline, local, remote, fast, slow, secure, public, private, shared, custom, default, primary, secondary, backup, hello_world, build_started, build_finished, parse_error, file_not_found, access_denied, operation_complete, invalid_input, retry_later, connection_lost, cache_miss, cache_hit, request_failed, task_complete
@words digit = 1, 2, 3, 4, 5, 6, 7, 8, 9
@words prefix = compute, process, update, handle, build, check, render, collect, parse, encode, decode, merge, split, find, apply, visit, create, destroy, allocate, free, load, store, fetch, send, receive, open, close, read, write, validate, normalize, transform, convert, serialize, deserialize, filter, reduce, scan, sort, search, insert, remove, replace, compare, calculate, analyze, summarize, generate, evaluate, execute, schedule, dispatch, resolve, connect, disconnect, flush, drain, inspect, measure
@keywords label = alpha, beta, gamma, delta, epsilon, zeta, eta, theta, iota, kappa, lambda, mu, nu, xi, omicron, pi, rho, sigma, tau, upsilon, phi, chi, psi, omega, red, green, blue, amber, cyan, magenta, silver, golden
@words entity = user, account, order, item, record, event, session, invoice, payment, product, project, task, ticket, message, cache, buffer, queue, worker, request, response, config, profile, report, metric, log, file, document, group, member, role, permission, setting, address, country, region, category, tag, comment, review, shipment, inventory, warehouse, employee, department, customer, supplier
@words proper = User, Account, Order, Item, Record, Event, Session, Invoice, Payment, Product, Project, Task, Ticket, Message, Cache, Buffer, Queue, Worker, Request, Response, Config, Profile, Report, Metric, Log, File, Document, Group, Member, Role, Permission, Setting, Address, Country, Region, Category, Tag, Comment, Review, Shipment, Inventory, Warehouse, Employee, Department, Customer, Supplier

program: module imports declaration^ maindecl;

module: 'module Main where\n\n';

imports: 'import Data.List (sort, nub)\nimport Data.Maybe (fromMaybe)\n\n';

maindecl: 'data Counter = Counter { value :: Int }\n\nclass Describable a where\n    describe :: a -> String\n\ninstance Describable Counter where\n    describe (Counter v) = "counter=" ++ show v\n\nmain :: IO ()\nmain = do\n    let values = [5, 2, 8, 1, 3]\n    print (sort values)\n    print (sum values)\n    print (nub [1, 2, 1, 3, 2])\n    print (fromMaybe 0 (Just (sum values)))\n    print (describe (Counter (sum values)))\n    putStrLn ("' label ': ' msg '")\n';

declaration:
      function<0.50>
    | casefunction<0.20>
    | listfunction<0.18>
    | typededecl<0.12>;

function: fname ' seed =\n    let count = seed + ' number '\n        total = count * ' number '\n        values = [1 .. ' number ']\n    in if count > ' number '\n        then total + sum values\n        else total - count\n\n';

casefunction: fname ' seed =\n    case seed `mod` 3 of\n        0 -> seed + ' number '\n        1 -> seed * ' number '\n        _ -> seed - ' number '\n\n';

listfunction: fname ' seed = sum (map (\\x -> x * ' number ' + seed) (filter even [1 .. ' number ']))\n\n';

typededecl: 'type ' tname ' = [Int]\n\n';

fname: prefix '_' entity ('_' digit)?;
tname: 'Type' proper ('_' digit)?;
number: digit digit?;
