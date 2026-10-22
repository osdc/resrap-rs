// Inline cases are generated from the Go reference implementation.
// Template cases are Rust regression snapshots: the embedded templates use
// expanded word lists and therefore no longer match the Go package byte-for-byte.
// Regenerate template expectations after intentional template changes.
pub static CASES: &[ParityCase] = &[
    ParityCase {
        name: "simple-random-1",
        kind: "inline",
        source: r#"program:function^;
function:'f' '(' ')';
"#,
        start: "program",
        seed: 1,
        tokens: 20,
        complete: false,
        expected: r#"f()f()f()f()f()f()f("#,
    },
    ParityCase {
        name: "simple-random-42",
        kind: "inline",
        source: r#"program:function^;
function:'f' '(' ')';
"#,
        start: "program",
        seed: 42,
        tokens: 20,
        complete: false,
        expected: r#"f()f()f()f()f()f()f("#,
    },
    ParityCase {
        name: "simple-complete-999",
        kind: "inline",
        source: r#"program:function^;
function:'f' '(' ')';
"#,
        start: "program",
        seed: 999,
        tokens: 20,
        complete: true,
        expected: r#"f()f()f()f()f()f()f()"#,
    },
    ParityCase {
        name: "words",
        kind: "inline",
        source: r#"@words identifier = alpha, beta, gamma
program: identifier ' ' [a-z] ';\n';"#,
        start: "program",
        seed: 7,
        tokens: 10,
        complete: false,
        expected: r#"beta fotgxs;
"#,
    },
    ParityCase {
        name: "complete-boundary",
        kind: "inline",
        source: r#"program:'begin\n' item* 'end\n';
item:'item\n';"#,
        start: "program",
        seed: 3,
        tokens: 3,
        complete: true,
        expected: r#"begin
end
"#,
    },
    ParityCase {
        name: "complete-boundary-2",
        kind: "inline",
        source: r#"program:'begin\n' item* 'end\n';
item:'item\n';"#,
        start: "program",
        seed: 5,
        tokens: 3,
        complete: true,
        expected: r#"begin
end
"#,
    },
    ParityCase {
        name: "probabilities",
        kind: "inline",
        source: r#"program:('a'<0.2>) | ('b'<0.8>);"#,
        start: "program",
        seed: 11,
        tokens: 30,
        complete: false,
        expected: r#"a"#,
    },
    ParityCase {
        name: "escapes",
        kind: "inline",
        source: r#"program:'it\'s' '\n' [a-c0-2] ';';"#,
        start: "program",
        seed: 4,
        tokens: 10,
        complete: false,
        expected: r#"it's
a1c;"#,
    },
    ParityCase {
        name: "regex-only",
        kind: "inline",
        source: r#"program:[a-z]^;"#,
        start: "program",
        seed: 7,
        tokens: 5,
        complete: true,
        expected: r#"rlkpfsoxlnewwenrrtynzdveapem"#,
    },
    ParityCase {
        name: "unknown-start",
        kind: "inline",
        source: r#"program:'a';"#,
        start: "missing",
        seed: 1,
        tokens: 10,
        complete: false,
        expected: r#""#,
    },
    ParityCase {
        name: "template-c-1",
        kind: "template",
        source: r#"c"#,
        start: "program",
        seed: 1,
        tokens: 60,
        complete: true,
        expected: r#"#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <stdbool.h>

static int render_log(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    char label[32] = "nu";
    int items[8] = {0};
    for (int i = 0; i < 8; ++i) {
        items[i] = i + seed;
    }
    if (count > 3) {
    count += 9;
    printf("%s: %d %.2f\n", label, count, value);
        total += 16;
    } else {
        total -= 1;
    }
    count = (count > 2) ? count - 16 : count + 2;
    return total;
}

typedef struct TypeCustomer_1 {
    int id;
    double value;
    char label[32];
    int items[8];
} TypeRecord_1;

typedef struct TypeRequest_7 {
    int id;
    double value;
    char label[32];
    int items[8];
} TypeAddress;

int main(void) {
    int values[8] = {3, 1, 4, 1, 5, 9, 2, 6};
    int total = 0;
    for (int i = 0; i < 8; ++i) {
        total += values[i];
    }
    printf("start: %d\n", total);
    printf("done: %s\n", "pi");
    return 0;
}

int resolve_worker_6(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    char label[32] = "tau";
    int items[8] = {0};
    for (int i = 0; i < 8; ++i) {
        items[i] = i + seed;
    }
    items[count % 8] = count + 3;
    return total;
}

"#,
    },
    ParityCase {
        name: "template-c-7",
        kind: "template",
        source: r#"c"#,
        start: "program",
        seed: 7,
        tokens: 60,
        complete: true,
        expected: r#"#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <stdbool.h>

static int drain_report(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    char label[32] = "iota";
    int items[8] = {0};
    for (int i = 0; i < 8; ++i) {
        items[i] = i + seed;
    }
    return total;
}

static int insert_address_3(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    char label[32] = "upsilon";
    int items[8] = {0};
    for (int i = 0; i < 8; ++i) {
        items[i] = i + seed;
    }
    items[count % 8] = count + 93;
    for (int i = 0; i < 8; ++i) {
        total += items[i % 8];
    }
    total += items[count % 8];
    return total;
}

enum KindMetric_1 { VALUE_A = 0, VALUE_B = 1, VALUE_C = 2 };

int main(void) {
    int values[8] = {3, 1, 4, 1, 5, 9, 2, 6};
    int total = 0;
    for (int i = 0; i < 8; ++i) {
        total += values[i];
    }
    printf("start: %d\n", total);
    printf("done: %s\n", "alpha");
    return 0;
}

int free_order_2(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    char label[32] = "psi";
    int items[8] = {0};
    for (int i = 0; i < 8; ++i) {
        items[i] = i + seed;
    }
    return total;
}

int update_supplier(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    char label[32] = "silver";
    int items[8] = {0};
    for (int i = 0; i < 8; ++i) {
        items[i] = i + seed;
    }
    if (count > 2) {
    items[count % 8] = count + 89;
    value = value * 9.9;
    printf("%s: %d %.2f\n", label, count, value);
    snprintf(label, sizeof(label), "%s", "resrap");
    count += 95;
    total += count * 1;
    printf("%s: %d %.2f\n", label, count, value);
    count = (count > 1) ? count - 6 : count + 43;
        total += 9;
    } else {
        total -= 1;
    }
    return total;
}

"#,
    },
    ParityCase {
        name: "template-c-42",
        kind: "template",
        source: r#"c"#,
        start: "program",
        seed: 42,
        tokens: 60,
        complete: true,
        expected: r#"#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <stdbool.h>

typedef unsigned long TypeCache_3;

int main(void) {
    int values[8] = {3, 1, 4, 1, 5, 9, 2, 6};
    int total = 0;
    for (int i = 0; i < 8; ++i) {
        total += values[i];
    }
    printf("start: %d\n", total);
    printf("done: %s\n", "eta");
    return 0;
}

int schedule_request(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    char label[32] = "amber";
    int items[8] = {0};
    for (int i = 0; i < 8; ++i) {
        items[i] = i + seed;
    }
    total += items[count % 8];
    return total;
}

int remove_config_5(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    char label[32] = "nu";
    int items[8] = {0};
    for (int i = 0; i < 8; ++i) {
        items[i] = i + seed;
    }
    total += items[count % 8];
    for (int i = 0; i < 67; ++i) {
        total += items[i % 8];
    }
    return total;
}

int close_permission_7(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    char label[32] = "gamma";
    int items[8] = {0};
    for (int i = 0; i < 8; ++i) {
        items[i] = i + seed;
    }
    return total;
}

int destroy_region_4(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    char label[32] = "beta";
    int items[8] = {0};
    for (int i = 0; i < 8; ++i) {
        items[i] = i + seed;
    }
    value = value * 7.21;
    printf("%s: %d %.2f\n", label, count, value);
    printf("%s: %d %.2f\n", label, count, value);
    return total;
}

"#,
    },
    ParityCase {
        name: "template-cpp-1",
        kind: "template",
        source: r#"cpp"#,
        start: "program",
        seed: 1,
        tokens: 60,
        complete: true,
        expected: r#"#include <iostream>
#include <string>
#include <vector>
#include <map>
#include <memory>
#include <algorithm>
#include <numeric>
#include <sstream>
#include <stdexcept>

class Box {
public:
    int value;
    std::string name;
    explicit Box(int v, std::string n) : value(v), name(std::move(n)) {}
    int get() const { return value; }
    void add(int n) { value += n; }
    std::string describe() const { return name + ":" + std::to_string(value); }
};

int helper(int x) {
    return x * 2 + 1;
}

template <typename T>
T clamp_value(T value, T low, T high) {
    return value < low ? low : (value > high ? high : value);
}

int main() {
    Box box(helper(3), "delta");
    std::vector<int> values = {3, 1, 4, 1, 5};
    std::sort(values.begin(), values.end());
    int total = clamp_value(std::accumulate(values.begin(), values.end(), 0), 0, 100);
    std::cout << "start: " << box.describe() << " " << total << std::endl;
    std::cout << "done: " << "cache_miss" << std::endl;
    return 0;
}

int store_project(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    std::string label = "eta";
    std::vector<int> items;
    std::map<std::string, int> lookup;
    std::unique_ptr<int> held = std::make_unique<int>(seed);
    bool ready = true;
    int guard = 0;
    total += *held + 24;
    return total;
}

int load_role(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    std::string label = "theta";
    std::vector<int> items;
    std::map<std::string, int> lookup;
    std::unique_ptr<int> held = std::make_unique<int>(seed);
    bool ready = true;
    int guard = 0;
    items.push_back(2);
    return total;
}

int close_customer_1(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    std::string label = "delta";
    std::vector<int> items;
    std::map<std::string, int> lookup;
    std::unique_ptr<int> held = std::make_unique<int>(seed);
    bool ready = true;
    int guard = 0;
    for (int i = 0; i < 55; ++i) {
        count += i;
    }
    std::cout << label << ": " << count << " " << value << std::endl;
    return total;
}

int decode_tag(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    std::string label = "tau";
    std::vector<int> items;
    std::map<std::string, int> lookup;
    std::unique_ptr<int> held = std::make_unique<int>(seed);
    bool ready = true;
    int guard = 0;
    switch (count % 3) {
    case 0:
        count += 1;
        break;
    case 1:
        count += 2;
        break;
    default:
        count += 3;
        break;
    }
    ready = (count % 2 == 0) ? true : false;
    if (count > 4) {
    ready = (count % 2 == 0) ? true : false;
        count += 1;
    } else {
        count -= 1;
    }
    return total;
}

"#,
    },
    ParityCase {
        name: "template-cpp-7",
        kind: "template",
        source: r#"cpp"#,
        start: "program",
        seed: 7,
        tokens: 60,
        complete: true,
        expected: r#"#include <iostream>
#include <string>
#include <vector>
#include <map>
#include <memory>
#include <algorithm>
#include <numeric>
#include <sstream>
#include <stdexcept>

class Box {
public:
    int value;
    std::string name;
    explicit Box(int v, std::string n) : value(v), name(std::move(n)) {}
    int get() const { return value; }
    void add(int n) { value += n; }
    std::string describe() const { return name + ":" + std::to_string(value); }
};

int helper(int x) {
    return x * 2 + 1;
}

template <typename T>
T clamp_value(T value, T low, T high) {
    return value < low ? low : (value > high ? high : value);
}

int main() {
    Box box(helper(3), "iota");
    std::vector<int> values = {3, 1, 4, 1, 5};
    std::sort(values.begin(), values.end());
    int total = clamp_value(std::accumulate(values.begin(), values.end(), 0), 0, 100);
    std::cout << "start: " << box.describe() << " " << total << std::endl;
    std::cout << "done: " << "error" << std::endl;
    return 0;
}

int convert_group(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    std::string label = "alpha";
    std::vector<int> items;
    std::map<std::string, int> lookup;
    std::unique_ptr<int> held = std::make_unique<int>(seed);
    bool ready = true;
    int guard = 0;
    std::cout << label << ": " << count << " " << value << std::endl;
    total += *held + 15;
    for (int i = 0; i < 1; ++i) {
        count += i;
    }
    total = std::accumulate(items.begin(), items.end(), 0);
    std::ostringstream out;
    out << label << ":" << count;
    label = out.str();
    return total;
}

int encode_product_5(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    std::string label = "sigma";
    std::vector<int> items;
    std::map<std::string, int> lookup;
    std::unique_ptr<int> held = std::make_unique<int>(seed);
    bool ready = true;
    int guard = 0;
    try {
        if (count < 0) {
            throw std::runtime_error("negative count");
        }
        total += count;
    } catch (const std::exception& e) {
        total += 54;
    }
    return total;
}

int resolve_buffer(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    std::string label = "green";
    std::vector<int> items;
    std::map<std::string, int> lookup;
    std::unique_ptr<int> held = std::make_unique<int>(seed);
    bool ready = true;
    int guard = 0;
    std::cout << label << ": " << count << " " << value << std::endl;
    return total;
}

int drain_setting(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    std::string label = "red";
    std::vector<int> items;
    std::map<std::string, int> lookup;
    std::unique_ptr<int> held = std::make_unique<int>(seed);
    bool ready = true;
    int guard = 0;
    std::cout << label << ": " << count << " " << value << std::endl;
    return total;
}

"#,
    },
    ParityCase {
        name: "template-cpp-42",
        kind: "template",
        source: r#"cpp"#,
        start: "program",
        seed: 42,
        tokens: 60,
        complete: true,
        expected: r#"#include <iostream>
#include <string>
#include <vector>
#include <map>
#include <memory>
#include <algorithm>
#include <numeric>
#include <sstream>
#include <stdexcept>

class Box {
public:
    int value;
    std::string name;
    explicit Box(int v, std::string n) : value(v), name(std::move(n)) {}
    int get() const { return value; }
    void add(int n) { value += n; }
    std::string describe() const { return name + ":" + std::to_string(value); }
};

int helper(int x) {
    return x * 2 + 1;
}

template <typename T>
T clamp_value(T value, T low, T high) {
    return value < low ? low : (value > high ? high : value);
}

int main() {
    Box box(helper(3), "iota");
    std::vector<int> values = {3, 1, 4, 1, 5};
    std::sort(values.begin(), values.end());
    int total = clamp_value(std::accumulate(values.begin(), values.end(), 0), 0, 100);
    std::cout << "start: " << box.describe() << " " << total << std::endl;
    std::cout << "done: " << "code" << std::endl;
    return 0;
}

int collect_customer_3(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    std::string label = "theta";
    std::vector<int> items;
    std::map<std::string, int> lookup;
    std::unique_ptr<int> held = std::make_unique<int>(seed);
    bool ready = true;
    int guard = 0;
    for (const auto& item : items) {
        count += item;
    }
    return total;
}

int insert_log(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    std::string label = "green";
    std::vector<int> items;
    std::map<std::string, int> lookup;
    std::unique_ptr<int> held = std::make_unique<int>(seed);
    bool ready = true;
    int guard = 0;
    try {
        if (count < 0) {
            throw std::runtime_error("negative count");
        }
        total += count;
    } catch (const std::exception& e) {
        total += 2;
    }
    items.push_back(44);
    return total;
}

int fetch_ticket_2(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    std::string label = "theta";
    std::vector<int> items;
    std::map<std::string, int> lookup;
    std::unique_ptr<int> held = std::make_unique<int>(seed);
    bool ready = true;
    int guard = 0;
    lookup[label] = count;
    return total;
}

int load_metric(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    std::string label = "iota";
    std::vector<int> items;
    std::map<std::string, int> lookup;
    std::unique_ptr<int> held = std::make_unique<int>(seed);
    bool ready = true;
    int guard = 0;
    switch (count % 3) {
    case 0:
        count += 1;
        break;
    case 1:
        count += 2;
        break;
    default:
        count += 3;
        break;
    }
    return total;
}

int create_order_3(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    std::string label = "mu";
    std::vector<int> items;
    std::map<std::string, int> lookup;
    std::unique_ptr<int> held = std::make_unique<int>(seed);
    bool ready = true;
    int guard = 0;
    total += *held + 51;
    return total;
}

"#,
    },
    ParityCase {
        name: "template-go-1",
        kind: "template",
        source: r#"go"#,
        start: "program",
        seed: 1,
        tokens: 60,
        complete: true,
        expected: r#"package main

import (
    "fmt"
    "sort"
    "strings"
)

type Counter struct {
    Value int
}

func (c Counter) Describe() string {
    return fmt.Sprintf("counter=%d", c.Value)
}

func main() {
    values := []int{5, 2, 8, 1}
    sort.Ints(values)
    fmt.Println(strings.Join([]string{"start", "go"}, " "), values)
    c := Counter{Value: values[0]}
    fmt.Println(c.Describe())
}

type TypeCache []int

func destroy_permission(seed int) int {
    count := seed
    total := 0
    values := []int{count}
    lookup := map[string]int{"start": count}
    label := "eta"
    _ = values
    _ = lookup
    _ = label
    total += strings.Count(label, "a")
    return total
}

func reduce_member_8(seed int) int {
    count := seed
    total := 0
    values := []int{count}
    lookup := map[string]int{"start": count}
    label := "zeta"
    _ = values
    _ = lookup
    _ = label
    fmt.Println(label, count, total)
    return total
}

func (c Counter) visit_project_2(seed int) int {
    count := seed
    total := 0
    values := []int{count}
    lookup := map[string]int{"start": count}
    label := "golden"
    _ = values
    _ = lookup
    _ = label
    label = strings.TrimSpace(label + " hello_world")
    if count > 4 {
    values = append(values, count + 43)
        total += 6
    } else {
        total -= 1
    }
    return total + c.Value
}

type TypePayment struct {
    ID int
    Name string
    Value float64
    Items []int
}

func (c Counter) allocate_document(seed int) int {
    count := seed
    total := 0
    values := []int{count}
    lookup := map[string]int{"start": count}
    label := "omega"
    _ = values
    _ = lookup
    _ = label
    return total + c.Value
}

"#,
    },
    ParityCase {
        name: "template-go-7",
        kind: "template",
        source: r#"go"#,
        start: "program",
        seed: 7,
        tokens: 60,
        complete: true,
        expected: r#"package main

import (
    "fmt"
    "sort"
    "strings"
)

type Counter struct {
    Value int
}

func (c Counter) Describe() string {
    return fmt.Sprintf("counter=%d", c.Value)
}

func main() {
    values := []int{5, 2, 8, 1}
    sort.Ints(values)
    fmt.Println(strings.Join([]string{"start", "go"}, " "), values)
    c := Counter{Value: values[0]}
    fmt.Println(c.Describe())
}

type TypePermission struct {
    ID int
    Name string
    Value float64
    Items []int
}

type TypeSupplier interface {
    Describe() string
    Score(seed int) int
}

func (c Counter) process_user_8(seed int) int {
    count := seed
    total := 0
    values := []int{count}
    lookup := map[string]int{"start": count}
    label := "alpha"
    _ = values
    _ = lookup
    _ = label
    total += len(values)
    total += count * 6
    return total + c.Value
}

func serialize_inventory_2(seed int) int {
    count := seed
    total := 0
    values := []int{count}
    lookup := map[string]int{"start": count}
    label := "eta"
    _ = values
    _ = lookup
    _ = label
    return total
}

func (c Counter) replace_document(seed int) int {
    count := seed
    total := 0
    values := []int{count}
    lookup := map[string]int{"start": count}
    label := "amber"
    _ = values
    _ = lookup
    _ = label
    for _, value := range values {
        total += value
    }
    total += count * 17
    return total + c.Value
}

func measure_record(seed int) int {
    count := seed
    total := 0
    values := []int{count}
    lookup := map[string]int{"start": count}
    label := "chi"
    _ = values
    _ = lookup
    _ = label
    sort.Ints(values)
    total += values[0]
    return total
}

func (c Counter) find_account(seed int) int {
    count := seed
    total := 0
    values := []int{count}
    lookup := map[string]int{"start": count}
    label := "omega"
    _ = values
    _ = lookup
    _ = label
    if count > 98 {
    fmt.Println(label, count, total)
    fmt.Println(label, count, total)
    total += count * 15
    values = append(values, count + 58)
    fmt.Println(label, count, total)
    values = append(values, count + 52)
        total += 1
    } else {
        total -= 1
    }
    return total + c.Value
}

"#,
    },
    ParityCase {
        name: "template-go-42",
        kind: "template",
        source: r#"go"#,
        start: "program",
        seed: 42,
        tokens: 60,
        complete: true,
        expected: r#"package main

import (
    "fmt"
    "sort"
    "strings"
)

type Counter struct {
    Value int
}

func (c Counter) Describe() string {
    return fmt.Sprintf("counter=%d", c.Value)
}

func main() {
    values := []int{5, 2, 8, 1}
    sort.Ints(values)
    fmt.Println(strings.Join([]string{"start", "go"}, " "), values)
    c := Counter{Value: values[0]}
    fmt.Println(c.Describe())
}

func (c Counter) render_region_4(seed int) int {
    count := seed
    total := 0
    values := []int{count}
    lookup := map[string]int{"start": count}
    label := "eta"
    _ = values
    _ = lookup
    _ = label
    for _, value := range values {
        total += value
    }
    switch count % 3 {
    case 0:
        total += 1
    case 1:
        total += 2
    default:
        total += 3
    }
    fmt.Println(label, count, total)
    return total + c.Value
}

func validate_project(seed int) int {
    count := seed
    total := 0
    values := []int{count}
    lookup := map[string]int{"start": count}
    label := "eta"
    _ = values
    _ = lookup
    _ = label
    fmt.Println(label, count, total)
    total += lookup[label]
    return total
}

func load_event(seed int) int {
    count := seed
    total := 0
    values := []int{count}
    lookup := map[string]int{"start": count}
    label := "omicron"
    _ = values
    _ = lookup
    _ = label
    return total
}

func validate_project(seed int) int {
    count := seed
    total := 0
    values := []int{count}
    lookup := map[string]int{"start": count}
    label := "psi"
    _ = values
    _ = lookup
    _ = label
    return total
}

func (c Counter) fetch_file_7(seed int) int {
    count := seed
    total := 0
    values := []int{count}
    lookup := map[string]int{"start": count}
    label := "amber"
    _ = values
    _ = lookup
    _ = label
    return total + c.Value
}

func render_ticket_3(seed int) int {
    count := seed
    total := 0
    values := []int{count}
    lookup := map[string]int{"start": count}
    label := "golden"
    _ = values
    _ = lookup
    _ = label
    fmt.Println(label, count, total)
    lookup[label] = count
    if count > 2 {
    values = append(values, count + 95)
    total += len(values)
    values = append(values, count + 8)
    values = append(values, count + 49)
    values = append(values, count + 72)
    total = total*2 + count
    total += count * 76
    fmt.Println(label, count, total)
        total += 57
    } else {
        total -= 1
    }
    if count > 22 {
    values = append(values, count + 71)
        total += 7
    } else {
        total -= 1
    }
    switch count % 3 {
    case 0:
        total += 1
    case 1:
        total += 2
    default:
        total += 3
    }
    return total
}

"#,
    },
    ParityCase {
        name: "template-haskell-1",
        kind: "template",
        source: r#"haskell"#,
        start: "program",
        seed: 1,
        tokens: 60,
        complete: true,
        expected: r#"module Main where

import Data.List (sort, nub)
import Data.Maybe (fromMaybe)

allocate_record seed =
    let count = seed + 4
        total = count * 6
        values = [1 .. 9]
    in if count > 9
        then total + sum values
        else total - count

schedule_member_6 seed =
    let count = seed + 56
        total = count * 65
        values = [1 .. 54]
    in if count > 7
        then total + sum values
        else total - count

update_cache seed =
    let count = seed + 7
        total = count * 12
        values = [1 .. 5]
    in if count > 2
        then total + sum values
        else total - count

parse_invoice_7 seed =
    let count = seed + 84
        total = count * 6
        values = [1 .. 1]
    in if count > 54
        then total + sum values
        else total - count

data Counter = Counter { value :: Int }

class Describable a where
    describe :: a -> String

instance Describable Counter where
    describe (Counter v) = "counter=" ++ show v

main :: IO ()
main = do
    let values = [5, 2, 8, 1, 3]
    print (sort values)
    print (sum values)
    print (nub [1, 2, 1, 3, 2])
    print (fromMaybe 0 (Just (sum values)))
    print (describe (Counter (sum values)))
    putStrLn ("magenta: shared")
"#,
    },
    ParityCase {
        name: "template-haskell-7",
        kind: "template",
        source: r#"haskell"#,
        start: "program",
        seed: 7,
        tokens: 60,
        complete: true,
        expected: r#"module Main where

import Data.List (sort, nub)
import Data.Maybe (fromMaybe)

search_ticket seed =
    let count = seed + 3
        total = count * 4
        values = [1 .. 5]
    in if count > 3
        then total + sum values
        else total - count

filter_project seed =
    case seed `mod` 3 of
        0 -> seed + 24
        1 -> seed * 6
        _ -> seed - 8

serialize_inventory_2 seed =
    let count = seed + 3
        total = count * 98
        values = [1 .. 5]
    in if count > 6
        then total + sum values
        else total - count

encode_record seed = sum (map (\x -> x * 28 + seed) (filter even [1 .. 78]))

parse_event_7 seed = sum (map (\x -> x * 9 + seed) (filter even [1 .. 1]))

data Counter = Counter { value :: Int }

class Describable a where
    describe :: a -> String

instance Describable Counter where
    describe (Counter v) = "counter=" ++ show v

main :: IO ()
main = do
    let values = [5, 2, 8, 1, 3]
    print (sort values)
    print (sum values)
    print (nub [1, 2, 1, 3, 2])
    print (fromMaybe 0 (Just (sum values)))
    print (describe (Counter (sum values)))
    putStrLn ("eta: busy")
"#,
    },
    ParityCase {
        name: "template-haskell-42",
        kind: "template",
        source: r#"haskell"#,
        start: "program",
        seed: 42,
        tokens: 60,
        complete: true,
        expected: r#"module Main where

import Data.List (sort, nub)
import Data.Maybe (fromMaybe)

free_task_1 seed =
    let count = seed + 22
        total = count * 3
        values = [1 .. 4]
    in if count > 86
        then total + sum values
        else total - count

validate_project seed =
    case seed `mod` 3 of
        0 -> seed + 12
        1 -> seed * 2
        _ -> seed - 4

receive_country_6 seed =
    case seed `mod` 3 of
        0 -> seed + 91
        1 -> seed * 38
        _ -> seed - 1

inspect_metric_8 seed =
    case seed `mod` 3 of
        0 -> seed + 2
        1 -> seed * 66
        _ -> seed - 81

analyze_record_1 seed =
    case seed `mod` 3 of
        0 -> seed + 57
        1 -> seed * 2
        _ -> seed - 23

data Counter = Counter { value :: Int }

class Describable a where
    describe :: a -> String

instance Describable Counter where
    describe (Counter v) = "counter=" ++ show v

main :: IO ()
main = do
    let values = [5, 2, 8, 1, 3]
    print (sort values)
    print (sum values)
    print (nub [1, 2, 1, 3, 2])
    print (fromMaybe 0 (Just (sum values)))
    print (describe (Counter (sum values)))
    putStrLn ("rho: invalid")
"#,
    },
    ParityCase {
        name: "template-html-1",
        kind: "template",
        source: r#"html"#,
        start: "program",
        seed: 1,
        tokens: 60,
        complete: true,
        expected: r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Resrap generated page</title>
    <link rel="stylesheet" href="styles.css">
</head>
<body>
    <header class="site-header">
        <nav>
            <a href="#home">Home</a>
            <a href="#features">Features</a>
            <a href="#docs">Docs</a>
            <a href="#contact">Contact</a>
        </nav>
    </header>

    <article class="post">
        <h2>open_source</h2>
        <p>roadmap</p>
        <p>community</p>
        <a href="#read-more">Read more</a>
    </article>

    <p>sign_up analytics sign_up</p>
    <figure>
        <img src="image-13.png" alt="docs">
        <figcaption>example</figcaption>
    </figure>

    <article class="post">
        <h2>resrap</h2>
        <p>settings</p>
        <p>generated</p>
        <a href="#read-more">Read more</a>
    </article>

    <form action="#" method="post">
        <label for="name">Name</label>
        <input id="name" name="name" type="text" placeholder="cloud_native">
        <label for="email">Email</label>
        <input id="email" name="email" type="email" placeholder="you@example.com">
        <button type="submit">Submit</button>
    </form>

    <section class="cards">
        <h2>pricing</h2>
        <div class="card">
            <h3>features</h3>
            <p>real_time</p>
        </div>
        <div class="card">
            <h3>settings</h3>
            <p>privacy</p>
        </div>
    </section>

    <section class="cards">
        <h2>html</h2>
        <div class="card">
            <h3>overview</h3>
            <p>open_source</p>
        </div>
        <div class="card">
            <h3>benefits</h3>
            <p>feature</p>
        </div>
    </section>

    <section class="cards">
        <h2>feature</h2>
        <div class="card">
            <h3>terms</h3>
            <p>customer</p>
        </div>
        <div class="card">
            <h3>documentation</h3>
            <p>terms</p>
        </div>
    </section>

    <footer class="site-footer">
        <p>Generated by Resrap</p>
        <ul>
            <li><a href="#privacy">Privacy</a></li>
            <li><a href="#terms">Terms</a></li>
        </ul>
    </footer>
</body>
</html>
"##,
    },
    ParityCase {
        name: "template-html-7",
        kind: "template",
        source: r#"html"#,
        start: "program",
        seed: 7,
        tokens: 60,
        complete: true,
        expected: r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Resrap generated page</title>
    <link rel="stylesheet" href="styles.css">
</head>
<body>
    <header class="site-header">
        <nav>
            <a href="#home">Home</a>
            <a href="#features">Features</a>
            <a href="#docs">Docs</a>
            <a href="#contact">Contact</a>
        </nav>
    </header>

    <table>
        <thead>
            <tr><th>Name</th><th>Value</th><th>Status</th></tr>
        </thead>
        <tbody>
            <tr><td>project</td><td>3</td><td>featured</td></tr>
            <tr><td>analytics</td><td>86</td><td>verified</td></tr>
        </tbody>
    </table>

    <ul class="feature-list">
    </ul>
    <article class="post">
        <h2>world</h2>
        <p>customer</p>
        <p>welcome</p>
        <a href="#read-more">Read more</a>
    </article>

    <section class="cards">
        <h2>enterprise_ready</h2>
        <div class="card">
            <h3>best_practices</h3>
            <p>terms</p>
        </div>
        <div class="card">
            <h3>details</h3>
            <p>updates</p>
        </div>
    </section>

    <details>
        <summary>features</summary>
        <p>documentation</p>
    </details>

    <details>
        <summary>settings</summary>
        <p>insights</p>
    </details>

    <form action="#" method="post">
        <label for="name">Name</label>
        <input id="name" name="name" type="text" placeholder="hello">
        <label for="email">Email</label>
        <input id="email" name="email" type="email" placeholder="you@example.com">
        <button type="submit">Submit</button>
    </form>

    <details>
        <summary>hello</summary>
        <p>product</p>
    </details>

    <section class="cards">
        <h2>settings</h2>
        <div class="card">
            <h3>markup</h3>
            <p>code</p>
        </div>
        <div class="card">
            <h3>documentation</h3>
            <p>log_in</p>
        </div>
    </section>

    <footer class="site-footer">
        <p>Generated by Resrap</p>
        <ul>
            <li><a href="#privacy">Privacy</a></li>
            <li><a href="#terms">Terms</a></li>
        </ul>
    </footer>
</body>
</html>
"##,
    },
    ParityCase {
        name: "template-html-42",
        kind: "template",
        source: r#"html"#,
        start: "program",
        seed: 42,
        tokens: 60,
        complete: true,
        expected: r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Resrap generated page</title>
    <link rel="stylesheet" href="styles.css">
</head>
<body>
    <header class="site-header">
        <nav>
            <a href="#home">Home</a>
            <a href="#features">Features</a>
            <a href="#docs">Docs</a>
            <a href="#contact">Contact</a>
        </nav>
    </header>

    <form action="#" method="post">
        <label for="name">Name</label>
        <input id="name" name="name" type="text" placeholder="project">
        <label for="email">Email</label>
        <input id="email" name="email" type="email" placeholder="you@example.com">
        <button type="submit">Submit</button>
    </form>

    <p>roadmap customer example</p>
    <p>real_time project release_notes</p>
    <ul class="feature-list">
        <li>log_in</li>
    </ul>
    <section id="home" class="hero">
        <h1>performance</h1>
        <p>feature</p>
        <a class="button" href="#features">Get started</a>
    </section>

    <p>cloud_native contact support</p>
    <section class="cards">
        <h2>html</h2>
        <div class="card">
            <h3>built_to_scale</h3>
            <p>reliability</p>
        </div>
        <div class="card">
            <h3>about</h3>
            <p>security</p>
        </div>
    </section>

    <p>hello resources changelog</p>
    <p>analytics security updates</p>
    <footer class="site-footer">
        <p>Generated by Resrap</p>
        <ul>
            <li><a href="#privacy">Privacy</a></li>
            <li><a href="#terms">Terms</a></li>
        </ul>
    </footer>
</body>
</html>
"##,
    },
    ParityCase {
        name: "template-java-1",
        kind: "template",
        source: r#"java"#,
        start: "program",
        seed: 1,
        tokens: 60,
        complete: true,
        expected: r#"import java.util.*;

public class Main {
    public static void main(String[] args) {
        List<Integer> values = new ArrayList<>();
        values.add(5);
        values.add(2);
        values.add(8);
        Collections.sort(values);
        int total = 0;
        for (int value : values) {
            total += value;
        }
        System.out.println("start: " + values + " " + total);
    }

    static class TypeLog {
        int id;
        String name = "item";
        double value = 1.5;
        int score() { return id + name.length(); }
    }

    private static int release_payment_1(int seed) {
        int count = seed;
        int total = 0;
        List<Integer> values = new ArrayList<>();
        Map<String, Integer> lookup = new HashMap<>();
        String label = "golden";
        count += 4;
        while (count < 45) {
            count += 1;
            total += count;
        }
        while (count < 16) {
            count += 1;
            total += count;
        }
        Collections.sort(values);
        return total;
    }

    private static int drain_region(int seed) {
        int count = seed;
        int total = 0;
        List<Integer> values = new ArrayList<>();
        Map<String, Integer> lookup = new HashMap<>();
        String label = "alpha";
        try {
            total += Integer.parseInt("5");
        } catch (NumberFormatException e) {
            total += 1;
        }
        try {
            total += Integer.parseInt("43");
        } catch (NumberFormatException e) {
            total += 1;
        }
        return total;
    }

    private static int summarize_document_2(int seed) {
        int count = seed;
        int total = 0;
        List<Integer> values = new ArrayList<>();
        Map<String, Integer> lookup = new HashMap<>();
        String label = "green";
        for (int value : values) {
            total += value;
        }
        for (int i = 0; i < 6; i++) {
            total += i;
        values.add(count + 98);
        System.out.println(label + ": hello_world " + count + " " + total);
        }
        System.out.println(label + ": world " + count + " " + total);
        return total;
    }

}
"#,
    },
    ParityCase {
        name: "template-java-7",
        kind: "template",
        source: r#"java"#,
        start: "program",
        seed: 7,
        tokens: 60,
        complete: true,
        expected: r#"import java.util.*;

public class Main {
    public static void main(String[] args) {
        List<Integer> values = new ArrayList<>();
        values.add(5);
        values.add(2);
        values.add(8);
        Collections.sort(values);
        int total = 0;
        for (int value : values) {
            total += value;
        }
        System.out.println("start: " + values + " " + total);
    }

    static int search_ticket(int seed) {
        int count = seed;
        int total = 0;
        List<Integer> values = new ArrayList<>();
        Map<String, Integer> lookup = new HashMap<>();
        String label = "omega";
        for (int i = 0; i < 2; i++) {
            total += i;
        values.add(count + 9);
        }
        lookup.put(label, count);
        return total + seed;
    }

    static int compare_review(int seed) {
        int count = seed;
        int total = 0;
        List<Integer> values = new ArrayList<>();
        Map<String, Integer> lookup = new HashMap<>();
        String label = "rho";
        if (count > 6) {
        count += 25;
        total += count * 6;
        total = total * 2 + count;
            total += 2;
        } else {
            total -= 1;
        }
        return total + seed;
    }

    private static int search_product(int seed) {
        int count = seed;
        int total = 0;
        List<Integer> values = new ArrayList<>();
        Map<String, Integer> lookup = new HashMap<>();
        String label = "epsilon";
        for (int i = 0; i < 97; i++) {
            total += i;
        count += 9;
        }
        return total;
    }

    private static int visit_country_9(int seed) {
        int count = seed;
        int total = 0;
        List<Integer> values = new ArrayList<>();
        Map<String, Integer> lookup = new HashMap<>();
        String label = "xi";
        try {
            total += Integer.parseInt("42");
        } catch (NumberFormatException e) {
            total += 1;
        }
        total += count * 15;
        try {
            total += Integer.parseInt("58");
        } catch (NumberFormatException e) {
            total += 1;
        }
        for (int value : values) {
            total += value;
        }
        try {
            total += Integer.parseInt("52");
        } catch (NumberFormatException e) {
            total += 1;
        }
        return total;
    }

}
"#,
    },
    ParityCase {
        name: "template-java-42",
        kind: "template",
        source: r#"java"#,
        start: "program",
        seed: 42,
        tokens: 60,
        complete: true,
        expected: r#"import java.util.*;

public class Main {
    public static void main(String[] args) {
        List<Integer> values = new ArrayList<>();
        values.add(5);
        values.add(2);
        values.add(8);
        Collections.sort(values);
        int total = 0;
        for (int value : values) {
            total += value;
        }
        System.out.println("start: " + values + " " + total);
    }

    static int release_task_1(int seed) {
        int count = seed;
        int total = 0;
        List<Integer> values = new ArrayList<>();
        Map<String, Integer> lookup = new HashMap<>();
        String label = "epsilon";
        lookup.put(label, count);
        return total + seed;
    }

    static int dispatch_metric_3(int seed) {
        int count = seed;
        int total = 0;
        List<Integer> values = new ArrayList<>();
        Map<String, Integer> lookup = new HashMap<>();
        String label = "rho";
        count += 12;
        return total + seed;
    }

    static int receive_member_4(int seed) {
        int count = seed;
        int total = 0;
        List<Integer> values = new ArrayList<>();
        Map<String, Integer> lookup = new HashMap<>();
        String label = "golden";
        total += lookup.getOrDefault(label, 0);
        return total + seed;
    }

    private static int find_role(int seed) {
        int count = seed;
        int total = 0;
        List<Integer> values = new ArrayList<>();
        Map<String, Integer> lookup = new HashMap<>();
        String label = "silver";
        try {
            total += Integer.parseInt("5");
        } catch (NumberFormatException e) {
            total += 1;
        }
        System.out.println(label + ": error " + count + " " + total);
        return total;
    }

    static int check_record(int seed) {
        int count = seed;
        int total = 0;
        List<Integer> values = new ArrayList<>();
        Map<String, Integer> lookup = new HashMap<>();
        String label = "omega";
        if (count > 4) {
        Collections.sort(values);
        System.out.println(label + ": inserted " + count + " " + total);
        Collections.sort(values);
        count += 83;
            total += 1;
        } else {
            total -= 1;
        }
        return total + seed;
    }

}
"#,
    },
    ParityCase {
        name: "template-js-1",
        kind: "template",
        source: r#"js"#,
        start: "program",
        seed: 1,
        tokens: 60,
        complete: true,
        expected: r#"// generated JavaScript

function main() {
    const values = [5, 2, 8, 1];
    values.sort((a, b) => a - b);
    const total = values.reduce((a, b) => a + b, 0);
    const lookup = new Map();
    lookup.set("start", total);
    console.log("start:", values, lookup.get("start"));
}
function transform_order_2(seed) {
    let count = seed;
    let total = 0;
    const values = [];
    const lookup = new Map();
    const label = "magenta";
    total = total * 2 + count;
    return total;
}

function apply_shipment_6(seed) {
    let count = seed;
    let total = 0;
    const values = [];
    const lookup = new Map();
    const label = "mu";
    values.sort((a, b) => a - b);
    return total;
}

function receive_tag(seed) {
    let count = seed;
    let total = 0;
    const values = [];
    const lookup = new Map();
    const label = "mu";
    for (let i = 0; i < 1; i++) {
        total += i;
    }
    count += 51;
    console.log(label, "primary", count, total);
    switch (count % 3) {
        case 0:
            total += 1;
            break;
        case 1:
            total += 2;
            break;
        default:
            total += 3;
            break;
    }
    total += values.length;
    console.log(label, "inserted", count, total);
    for (const value of values) {
        total += value;
    }
    total += values.length;
    return total;
}

function replace_message_3(seed) {
    let count = seed;
    let total = 0;
    const values = [];
    const lookup = new Map();
    const label = "tau";
    for (const value of values) {
        total += value;
    }
    if (count > 8) {
    values.push(count + 77);
        total += 6;
    } else {
        total -= 1;
    }
    return total;
}


main();
"#,
    },
    ParityCase {
        name: "template-js-7",
        kind: "template",
        source: r#"js"#,
        start: "program",
        seed: 7,
        tokens: 60,
        complete: true,
        expected: r#"// generated JavaScript

function main() {
    const values = [5, 2, 8, 1];
    values.sort((a, b) => a - b);
    const total = values.reduce((a, b) => a + b, 0);
    const lookup = new Map();
    lookup.set("start", total);
    console.log("start:", values, lookup.get("start"));
}
function validate_file(seed) {
    let count = seed;
    let total = 0;
    const values = [];
    const lookup = new Map();
    const label = "amber";
    try {
        if (count < 0) throw new Error("negative");
        total += count;
    } catch (error) {
        total += 7;
    }
    total += lookup.get(label) || 0;
    return total;
}

function compare_project_6(seed) {
    let count = seed;
    let total = 0;
    const values = [];
    const lookup = new Map();
    const label = "blue";
    switch (count % 3) {
        case 0:
            total += 1;
            break;
        case 1:
            total += 2;
            break;
        default:
            total += 3;
            break;
    }
    try {
        if (count < 0) throw new Error("negative");
        total += count;
    } catch (error) {
        total += 86;
    }
    for (let i = 0; i < 58; i++) {
        total += i;
    }
    return total;
}

class TypeResponse {
    constructor(seed) {
        this.seed = seed;
        this.label = "zeta";
    }

    score_event(amount) {
        let count = amount;
        let total = this.seed;
        const values = [];
        const lookup = new Map();
        const label = this.label;
        return this.seed + amount;
    }
}

function transform_session_9(seed) {
    let count = seed;
    let total = 0;
    const values = [];
    const lookup = new Map();
    const label = "omega";
    total += count * 1;
    while (count < 73) {
        count += 1;
        total += count;
    }
    console.log(label, "shared", count, total);
    if (count > 98) {
    console.log(label, "inserted", count, total);
        total += 34;
    } else {
        total -= 1;
    }
    return total;
}


main();
"#,
    },
    ParityCase {
        name: "template-js-42",
        kind: "template",
        source: r#"js"#,
        start: "program",
        seed: 42,
        tokens: 60,
        complete: true,
        expected: r#"// generated JavaScript

function main() {
    const values = [5, 2, 8, 1];
    values.sort((a, b) => a - b);
    const total = values.reduce((a, b) => a + b, 0);
    const lookup = new Map();
    lookup.set("start", total);
    console.log("start:", values, lookup.get("start"));
}
function calculate_request_8(seed) {
    let count = seed;
    let total = 0;
    const values = [];
    const lookup = new Map();
    const label = "chi";
    lookup.set(label, count);
    return total;
}

class TypeSupplier {
    constructor(seed) {
        this.seed = seed;
        this.label = "theta";
    }

    score_file(amount) {
        let count = amount;
        let total = this.seed;
        const values = [];
        const lookup = new Map();
        const label = this.label;
        return this.seed + amount;
    }
}

class TypeCountry {
    constructor(seed) {
        this.seed = seed;
        this.label = "lambda";
    }

    score_member_4(amount) {
        let count = amount;
        let total = this.seed;
        const values = [];
        const lookup = new Map();
        const label = this.label;
        return this.seed + amount;
    }
}

class TypeCache_5 {
    constructor(seed) {
        this.seed = seed;
        this.label = "alpha";
    }

    score_address(amount) {
        let count = amount;
        let total = this.seed;
        const values = [];
        const lookup = new Map();
        const label = this.label;
        return this.seed + amount;
    }
}

const CONFIG_document_2 = {
    id: 8,
    name: "delta",
    value: 14.33,
    items: [1, 2, 3],
    describe() {
        return this.name + ":" + this.id;
    }
};


main();
"#,
    },
    ParityCase {
        name: "template-python-1",
        kind: "template",
        source: r#"python"#,
        start: "program",
        seed: 1,
        tokens: 60,
        complete: true,
        expected: r#"import math
import random

def main():
    values = [3, 1, 4, 1, 5]
    values.sort()
    total = sum(values)
    print("start:", values, total)
    print("sqrt:", round(math.sqrt(total), 2))
    print("random:", random.randint(1, 9))

def transform_order_2(seed):
    count = seed
    total = 0
    values = []
    lookup = {}
    label = "magenta"
    total = total * 2 + count
    return total

def apply_shipment_6(seed):
    count = seed
    total = 0
    values = []
    lookup = {}
    label = "mu"
    total += len(values)
    return total

def receive_tag(seed):
    count = seed
    total = 0
    values = []
    lookup = {}
    label = "mu"
    for i in range(1):
        total += i
    total = total * 2 + count
    return total

class TypeMetric:
    def __init__(self, seed):
        self.seed = seed
        self.label = "nu"

    def score_tag(self, amount):
        count = self.seed + amount
        total = 0
        values = [count, amount, self.seed]
        for value in values:
            total += value
        return total + len(self.label)

def apply_address_5(seed):
    count = seed
    total = 0
    values = []
    lookup = {}
    label = "pi"
    total += count * 46
    if count > 9:
        values.append(count + 3)
        total += 13
    else:
        total -= 1
    return total


if __name__ == "__main__":
    main()
"#,
    },
    ParityCase {
        name: "template-python-7",
        kind: "template",
        source: r#"python"#,
        start: "program",
        seed: 7,
        tokens: 60,
        complete: true,
        expected: r#"import math
import random

def main():
    values = [3, 1, 4, 1, 5]
    values.sort()
    total = sum(values)
    print("start:", values, total)
    print("sqrt:", round(math.sqrt(total), 2))
    print("random:", random.randint(1, 9))

def validate_file(seed):
    count = seed
    total = 0
    values = []
    lookup = {}
    label = "amber"
    total += sum(x * x for x in range(7))
    total += len([x for x in values if x > 68])
    return total

def reduce_file(seed):
    count = seed
    total = 0
    values = []
    lookup = {}
    label = "nu"
    for i in range(3):
        total += i
    return total

def process_shipment(seed):
    count = seed
    total = 0
    values = []
    lookup = {}
    label = "omega"
    for value in values:
        total += value
    for value in values:
        total += value
    count += 85
    try:
        total += int("2")
    except ValueError:
        total += 1
    total = total * 2 + count
    total = total * 2 + count
    return total

def analyze_region_6(seed):
    count = seed
    total = 0
    values = []
    lookup = {}
    label = "silver"
    values.append(count + 71)
    return total


if __name__ == "__main__":
    main()
"#,
    },
    ParityCase {
        name: "template-python-42",
        kind: "template",
        source: r#"python"#,
        start: "program",
        seed: 42,
        tokens: 60,
        complete: true,
        expected: r#"import math
import random

def main():
    values = [3, 1, 4, 1, 5]
    values.sort()
    total = sum(values)
    print("start:", values, total)
    print("sqrt:", round(math.sqrt(total), 2))
    print("random:", random.randint(1, 9))

def calculate_request_8(seed):
    count = seed
    total = 0
    values = []
    lookup = {}
    label = "chi"
    lookup[label] = count
    return total

class TypeSupplier:
    def __init__(self, seed):
        self.seed = seed
        self.label = "theta"

    def score_file(self, amount):
        count = self.seed + amount
        total = 0
        values = [count, amount, self.seed]
        for value in values:
            total += value
        return total + len(self.label)

def parse_worker(seed):
    count = seed
    total = 0
    values = []
    lookup = {}
    label = "golden"
    total += sum(x * x for x in range(74))
    for i in range(38):
        total += i
        values.append(count + 4)
    return total

def destroy_region_4(seed):
    count = seed
    total = 0
    values = []
    lookup = {}
    label = "beta"
    total += count * 8
    if count > 1:
        total += count * 43
        total += 51
    else:
        total -= 1
    return total


if __name__ == "__main__":
    main()
"#,
    },
    ParityCase {
        name: "template-rust-1",
        kind: "template",
        source: r#"rust"#,
        start: "program",
        seed: 1,
        tokens: 60,
        complete: true,
        expected: r#"use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Counter {
    value: i32,
}

fn main() {
    let mut values = vec![5, 2, 8, 1];
    values.sort();
    let total: i32 = values.iter().sum();
    println!("start: {:?} {}", values, total);
    let mut lookup = HashMap::new();
    lookup.insert(String::from("start"), total);
    let counter = Counter { value: total };
    println!("counter: {:?}", counter);
}

trait TypeCache {
    fn describe(&self) -> String;
    fn score(&self, seed: i32) -> i32;
}

fn destroy_permission(seed: i32) -> i32 {
    let mut count = seed;
    let mut total = 0;
    let mut values = vec![count];
    let mut lookup: HashMap<String, i32> = HashMap::new();
    let label = String::from("eta");
    total += label.matches("a").count() as i32;
    total
}

impl Counter {
    fn reduce_member_8(&self, seed: i32) -> i32 {
    let mut count = seed;
    let mut total = 0;
    let mut values = vec![count];
    let mut lookup: HashMap<String, i32> = HashMap::new();
    let label = String::from("zeta");
    println!("remote {} {} {}", label, count, total);
        total + self.value
    }
}

fn find_product_9(seed: i32) -> i32 {
    let mut count = seed;
    let mut total = 0;
    let mut values = vec![count];
    let mut lookup: HashMap<String, i32> = HashMap::new();
    let label = String::from("rho");
    lookup.insert(label.clone(), count);
    if count > 4 {
    values.push(count + 43);
        total += 6;
    } else {
        total -= 1;
    }
    total
}

struct TypePayment {
    id: i32,
    name: String,
    value: f64,
    items: Vec<i32>,
}

struct TypeRequest_7 {
    id: i32,
    name: String,
    value: f64,
    items: Vec<i32>,
}

"#,
    },
    ParityCase {
        name: "template-rust-7",
        kind: "template",
        source: r#"rust"#,
        start: "program",
        seed: 7,
        tokens: 60,
        complete: true,
        expected: r#"use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Counter {
    value: i32,
}

fn main() {
    let mut values = vec![5, 2, 8, 1];
    values.sort();
    let total: i32 = values.iter().sum();
    println!("start: {:?} {}", values, total);
    let mut lookup = HashMap::new();
    lookup.insert(String::from("start"), total);
    let counter = Counter { value: total };
    println!("counter: {:?}", counter);
}

struct TypePermission {
    id: i32,
    name: String,
    value: f64,
    items: Vec<i32>,
}

enum TypeSupplier {
    Alpha,
    Beta,
    Gamma(i32),
    Delta { value: i32 },
}

impl Counter {
    fn process_user_8(&self, seed: i32) -> i32 {
    let mut count = seed;
    let mut total = 0;
    let mut values = vec![count];
    let mut lookup: HashMap<String, i32> = HashMap::new();
    let label = String::from("alpha");
    total += values.len() as i32;
    total += count * 6;
        total + self.value
    }
}

fn serialize_inventory_2(seed: i32) -> i32 {
    let mut count = seed;
    let mut total = 0;
    let mut values = vec![count];
    let mut lookup: HashMap<String, i32> = HashMap::new();
    let label = String::from("eta");
    total
}

impl Counter {
    fn replace_document(&self, seed: i32) -> i32 {
    let mut count = seed;
    let mut total = 0;
    let mut values = vec![count];
    let mut lookup: HashMap<String, i32> = HashMap::new();
    let label = String::from("amber");
    for value in &values {
        total += *value;
    }
    total += count * 17;
        total + self.value
    }
}

fn measure_record(seed: i32) -> i32 {
    let mut count = seed;
    let mut total = 0;
    let mut values = vec![count];
    let mut lookup: HashMap<String, i32> = HashMap::new();
    let label = String::from("chi");
    values.sort();
    total += values[0];
    total
}

impl Counter {
    fn find_account(&self, seed: i32) -> i32 {
    let mut count = seed;
    let mut total = 0;
    let mut values = vec![count];
    let mut lookup: HashMap<String, i32> = HashMap::new();
    let label = String::from("omega");
    if count > 98 {
    println!("inserted {} {} {}", label, count, total);
        total += 34;
    } else {
        total -= 1;
    }
        total + self.value
    }
}

"#,
    },
    ParityCase {
        name: "template-rust-42",
        kind: "template",
        source: r#"rust"#,
        start: "program",
        seed: 42,
        tokens: 60,
        complete: true,
        expected: r#"use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Counter {
    value: i32,
}

fn main() {
    let mut values = vec![5, 2, 8, 1];
    values.sort();
    let total: i32 = values.iter().sum();
    println!("start: {:?} {}", values, total);
    let mut lookup = HashMap::new();
    lookup.insert(String::from("start"), total);
    let counter = Counter { value: total };
    println!("counter: {:?}", counter);
}

impl Counter {
    fn render_region_4(&self, seed: i32) -> i32 {
    let mut count = seed;
    let mut total = 0;
    let mut values = vec![count];
    let mut lookup: HashMap<String, i32> = HashMap::new();
    let label = String::from("eta");
    for value in &values {
        total += *value;
    }
    match count % 3 {
        0 => total += 1,
        1 => total += 2,
        _ => total += 3,
    }
    if count > 6 {
    println!("secure {} {} {}", label, count, total);
        total += 41;
    } else {
        total -= 1;
    }
    match count % 3 {
        0 => total += 1,
        1 => total += 2,
        _ => total += 3,
    }
        total + self.value
    }
}

impl Counter {
    fn receive_member_4(&self, seed: i32) -> i32 {
    let mut count = seed;
    let mut total = 0;
    let mut values = vec![count];
    let mut lookup: HashMap<String, i32> = HashMap::new();
    let label = String::from("golden");
    total += label.matches("a").count() as i32;
        total + self.value
    }
}

fn find_role(seed: i32) -> i32 {
    let mut count = seed;
    let mut total = 0;
    let mut values = vec![count];
    let mut lookup: HashMap<String, i32> = HashMap::new();
    let label = String::from("silver");
    let text = format!("{}:{}", label, count);
    total += text.len() as i32;
    total
}

impl Counter {
    fn update_order_8(&self, seed: i32) -> i32 {
    let mut count = seed;
    let mut total = 0;
    let mut values = vec![count];
    let mut lookup: HashMap<String, i32> = HashMap::new();
    let label = String::from("iota");
    if count > 43 {
    count += 2;
    total = total * 2 + count;
        total += 45;
    } else {
        total -= 1;
    }
        total + self.value
    }
}

"#,
    },
    ParityCase {
        name: "template-sql-1",
        kind: "template",
        source: r#"sql"#,
        start: "program",
        seed: 1,
        tokens: 60,
        complete: true,
        expected: r#"SELECT region FROM comments;
SELECT * FROM payments WHERE weight < 28.68 ORDER BY priority DESC LIMIT 8;
SELECT warehouses.end_date, suppliers.description FROM invoices JOIN accounts ON accounts.id = warehouses.id WHERE city != NULL AND active < NULL AND email != NULL AND description >= 55;
SELECT employees.checksum, comments.currency FROM profiles JOIN users ON users.id = items.id WHERE order_id = 'active' OR version != 'resrap';
"#,
    },
    ParityCase {
        name: "template-sql-7",
        kind: "template",
        source: r#"sql"#,
        start: "program",
        seed: 7,
        tokens: 60,
        complete: true,
        expected: r#"SELECT audit_logs.category_id, downloads.shipping FROM members JOIN metrics ON transactions.id = coupons.id WHERE external_id LIKE 3.1 AND source LIKE 6;
SELECT * FROM transactions WHERE updated_at LIKE 56.98 AND version LIKE 88.2 OR id >= 1.8 ORDER BY payload DESC LIMIT 7;
UPDATE suppliers SET name = TRUE, verified = 6.1 WHERE due_date <= 2.5 OR rating > TRUE;
"#,
    },
    ParityCase {
        name: "template-sql-42",
        kind: "template",
        source: r#"sql"#,
        start: "program",
        seed: 42,
        tokens: 60,
        complete: true,
        expected: r#"CREATE TABLE subscriptions (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    value INTEGER DEFAULT 1,
    status REAL
);
SELECT notifications.price, reports.description FROM regions JOIN files ON countries.id = addresses.id WHERE balance > 'active' AND due_date != FALSE;
SELECT uploads.order_id, accounts.total FROM uploads JOIN warehouses ON tags.id = warehouses.id WHERE balance > 6;
SELECT transactions.tax, customers.description FROM subscriptions JOIN tasks ON roles.id = accounts.id WHERE order_id = 'pending' AND email LIKE 'failed';
"#,
    },
];
