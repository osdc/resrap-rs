// Generated from the Go reference implementation. Do not edit by hand.
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

static int process_ibap(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    char label[32] = "alpha";
    int items[8] = {0};
    for (int i = 0; i < 8; ++i) {
        items[i] = i + seed;
    }
    if (count > 7) {
    count = (count > 91) ? count - 4 : count + 61;
        total += 1;
    } else {
        total -= 1;
    }
    return total;
}

int main(void) {
    int values[8] = {3, 1, 4, 1, 5, 9, 2, 6};
    int total = 0;
    for (int i = 0; i < 8; ++i) {
        total += values[i];
    }
    printf("start: %d\n", total);
    printf("done: %s\n", "gamma");
    return 0;
}

int visit_fdgd(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    char label[32] = "alpha";
    int items[8] = {0};
    for (int i = 0; i < 8; ++i) {
        items[i] = i + seed;
    }
    items[count % 8] = count + 96;
    return total;
}

int apply_ihcg(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    char label[32] = "alpha";
    int items[8] = {0};
    for (int i = 0; i < 8; ++i) {
        items[i] = i + seed;
    }
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
    printf("%s: %d %.2f\n", label, count, value);
    return total;
}

int update_mhah(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    char label[32] = "alpha";
    int items[8] = {0};
    for (int i = 0; i < 8; ++i) {
        items[i] = i + seed;
    }
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
    printf("%s: %d %.2f\n", label, count, value);
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

static int visit_heii(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    char label[32] = "alpha";
    int items[8] = {0};
    for (int i = 0; i < 8; ++i) {
        items[i] = i + seed;
    }
    return total;
}

int main(void) {
    int values[8] = {3, 1, 4, 1, 5, 9, 2, 6};
    int total = 0;
    for (int i = 0; i < 8; ++i) {
        total += values[i];
    }
    printf("start: %d\n", total);
    printf("done: %s\n", "gamma");
    return 0;
}

int compute_nbaj(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    char label[32] = "alpha";
    int items[8] = {0};
    for (int i = 0; i < 8; ++i) {
        items[i] = i + seed;
    }
    for (int i = 0; i < 16; ++i) {
        total += items[i % 8];
    }
    return total;
}

int parse_ikok(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    char label[32] = "alpha";
    int items[8] = {0};
    for (int i = 0; i < 8; ++i) {
        items[i] = i + seed;
    }
    printf("%s: %d %.2f\n", label, count, value);
    return total;
}

int encode_pjda(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    char label[32] = "alpha";
    int items[8] = {0};
    for (int i = 0; i < 8; ++i) {
        items[i] = i + seed;
    }
    value = value * 2.3;
    while (count < 72) {
        ++count;
        total += count;
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

typedef unsigned long Type_fme;

int main(void) {
    int values[8] = {3, 1, 4, 1, 5, 9, 2, 6};
    int total = 0;
    for (int i = 0; i < 8; ++i) {
        total += values[i];
    }
    printf("start: %d\n", total);
    printf("done: %s\n", "beta");
    return 0;
}

int build_kpmm(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    char label[32] = "alpha";
    int items[8] = {0};
    for (int i = 0; i < 8; ++i) {
        items[i] = i + seed;
    }
    total += items[count % 8];
    return total;
}

int merge_haim(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    char label[32] = "alpha";
    int items[8] = {0};
    for (int i = 0; i < 8; ++i) {
        items[i] = i + seed;
    }
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
    return total;
}

int compute_dbmb(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    char label[32] = "alpha";
    int items[8] = {0};
    for (int i = 0; i < 8; ++i) {
        items[i] = i + seed;
    }
    total += items[count % 8];
    return total;
}

int render_akim(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    char label[32] = "alpha";
    int items[8] = {0};
    for (int i = 0; i < 8; ++i) {
        items[i] = i + seed;
    }
    printf("%s: %d %.2f\n", label, count, value);
    items[count % 8] = count + 66;
    return total;
}

int visit_mcbf(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    char label[32] = "alpha";
    int items[8] = {0};
    for (int i = 0; i < 8; ++i) {
        items[i] = i + seed;
    }
    for (int i = 0; i < 33; ++i) {
        total += items[i % 8];
    printf("%s: %d %.2f\n", label, count, value);
    }
    while (count < 25) {
        ++count;
        total += count;
    }
    if (count > 83) {
    total += items[count % 8];
    items[count % 8] = count + 8;
    items[count % 8] = count + 49;
    items[count % 8] = count + 72;
    count = (count > 13) ? count - 4 : count + 56;
        total += 2;
    } else {
        total -= 1;
    }
    if (count > 22) {
    items[count % 8] = count + 71;
        total += 7;
    } else {
        total -= 1;
    }
    while (count < 58) {
        ++count;
        total += count;
    items[count % 8] = count + 59;
    }
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
    Box box(helper(3), "alpha");
    std::vector<int> values = {3, 1, 4, 1, 5};
    std::sort(values.begin(), values.end());
    int total = clamp_value(std::accumulate(values.begin(), values.end(), 0), 0, 100);
    std::cout << "start: " << box.describe() << " " << total << std::endl;
    std::cout << "done: " << "done" << std::endl;
    return 0;
}

int check_diod(int seed) {
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
    guard = 3;
    while (guard > 0) {
        --guard;
        count += 1;
    ready = (count % 2 == 0) ? true : false;
    }
    return total;
}

int handle_pefd(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    std::string label = "omega";
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

int apply_ihcg(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    std::string label = "omega";
    std::vector<int> items;
    std::map<std::string, int> lookup;
    std::unique_ptr<int> held = std::make_unique<int>(seed);
    bool ready = true;
    int guard = 0;
    total = std::accumulate(items.begin(), items.end(), 0);
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

int render_hahg(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    std::string label = "delta";
    std::vector<int> items;
    std::map<std::string, int> lookup;
    std::unique_ptr<int> held = std::make_unique<int>(seed);
    bool ready = true;
    int guard = 0;
    value = value * 5.31;
    if (count > 5) {
    std::sort(items.begin(), items.end());
    value = std::max(value, 3.43);
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
    Box box(helper(3), "beta");
    std::vector<int> values = {3, 1, 4, 1, 5};
    std::sort(values.begin(), values.end());
    int total = clamp_value(std::accumulate(values.begin(), values.end(), 0), 0, 100);
    std::cout << "start: " << box.describe() << " " << total << std::endl;
    std::cout << "done: " << "cpp" << std::endl;
    return 0;
}

int parse_jchl(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    std::string label = "beta";
    std::vector<int> items;
    std::map<std::string, int> lookup;
    std::unique_ptr<int> held = std::make_unique<int>(seed);
    bool ready = true;
    int guard = 0;
    ready = (count % 2 == 0) ? true : false;
    count = count + 4;
    std::sort(items.begin(), items.end());
    value = std::max(value, 5.4);
    return total;
}

int merge_cddi(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    std::string label = "alpha";
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

int find_capc(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    std::string label = "gamma";
    std::vector<int> items;
    std::map<std::string, int> lookup;
    std::unique_ptr<int> held = std::make_unique<int>(seed);
    bool ready = true;
    int guard = 0;
    total += *held + 53;
    return total;
}

int visit_lkge(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    std::string label = "beta";
    std::vector<int> items;
    std::map<std::string, int> lookup;
    std::unique_ptr<int> held = std::make_unique<int>(seed);
    bool ready = true;
    int guard = 0;
    {
        auto add = [](int a, int b) { return a + b; };
        count = add(count, 2);
    }
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
    Box box(helper(3), "beta");
    std::vector<int> values = {3, 1, 4, 1, 5};
    std::sort(values.begin(), values.end());
    int total = clamp_value(std::accumulate(values.begin(), values.end(), 0), 0, 100);
    std::cout << "start: " << box.describe() << " " << total << std::endl;
    std::cout << "done: " << "hello" << std::endl;
    return 0;
}

int update_pegk(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    std::string label = "delta";
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
    items.push_back(1);
    count = count + 42;
    return total;
}

int check_pafd(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    std::string label = "omega";
    std::vector<int> items;
    std::map<std::string, int> lookup;
    std::unique_ptr<int> held = std::make_unique<int>(seed);
    bool ready = true;
    int guard = 0;
    guard = 25;
    while (guard > 0) {
        --guard;
        count += 1;
    items.push_back(44);
    }
    return total;
}

int parse_ogdf(int seed) {
    int count = seed;
    int total = 0;
    double value = 1.5;
    std::string label = "alpha";
    std::vector<int> items;
    std::map<std::string, int> lookup;
    std::unique_ptr<int> held = std::make_unique<int>(seed);
    bool ready = true;
    int guard = 0;
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

type Type_eiba []int

func apply_iodm(seed int) int {
    count := seed
    total := 0
    values := []int{count}
    lookup := map[string]int{"start": count}
    label := "alpha"
    _ = values
    _ = lookup
    _ = label
    count += 4
    switch count % 3 {
    case 0:
        total += 1
    case 1:
        total += 2
    default:
        total += 3
    }
    for i := 0; i < 78; i++ {
        total += i
    values = append(values, count + 14)
    }
    fmt.Println(label, count, total)
    total = total*2 + count
    return total
}

type Type_cbfh interface {
    Describe() string
    Score(seed int) int
}

func (c Counter) handle_dcmc(seed int) int {
    count := seed
    total := 0
    values := []int{count}
    lookup := map[string]int{"start": count}
    label := "alpha"
    _ = values
    _ = lookup
    _ = label
    fmt.Println(label, count, total)
    return total + c.Value
}

func (c Counter) render_hahg(seed int) int {
    count := seed
    total := 0
    values := []int{count}
    lookup := map[string]int{"start": count}
    label := "alpha"
    _ = values
    _ = lookup
    _ = label
    for _, value := range values {
        total += value
    }
    fmt.Println(label, count, total)
    for _, value := range values {
        total += value
    }
    for _, value := range values {
        total += value
    }
    if count > 8 {
    values = append(values, count + 77)
        total += 6
    } else {
        total -= 1
    }
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

type Type_khei struct {
    ID int
    Name string
    Value float64
    Items []int
}

type Type_jchl struct {
    ID int
    Name string
    Value float64
    Items []int
}

func process_jhdc(seed int) int {
    count := seed
    total := 0
    values := []int{count}
    lookup := map[string]int{"start": count}
    label := "alpha"
    _ = values
    _ = lookup
    _ = label
    total += count * 6
    return total
}

func parse_okbc(seed int) int {
    count := seed
    total := 0
    values := []int{count}
    lookup := map[string]int{"start": count}
    label := "alpha"
    _ = values
    _ = lookup
    _ = label
    for _, value := range values {
        total += value
    }
    return total
}

func (c Counter) visit_dagi(seed int) int {
    count := seed
    total := 0
    values := []int{count}
    lookup := map[string]int{"start": count}
    label := "alpha"
    _ = values
    _ = lookup
    _ = label
    sort.Ints(values)
    total += values[0]
    return total + c.Value
}

func split_oilc(seed int) int {
    count := seed
    total := 0
    values := []int{count}
    lookup := map[string]int{"start": count}
    label := "alpha"
    _ = values
    _ = lookup
    _ = label
    total += len(values)
    sort.Ints(values)
    total += values[0]
    sort.Ints(values)
    total += values[0]
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

func (c Counter) process_megb(seed int) int {
    count := seed
    total := 0
    values := []int{count}
    lookup := map[string]int{"start": count}
    label := "alpha"
    _ = values
    _ = lookup
    _ = label
    total += count * 48
    fmt.Println(label, count, total)
    return total + c.Value
}

type Type_ibdl interface {
    Describe() string
    Score(seed int) int
}

func (c Counter) update_gnln(seed int) int {
    count := seed
    total := 0
    values := []int{count}
    lookup := map[string]int{"start": count}
    label := "alpha"
    _ = values
    _ = lookup
    _ = label
    switch count % 3 {
    case 0:
        total += 1
    case 1:
        total += 2
    default:
        total += 3
    }
    delete(lookup, "missing")
    for i := 0; i < 38; i++ {
        total += i
    fmt.Println(label, count, total)
    }
    for _, value := range values {
        total += value
    }
    return total + c.Value
}

func merge_ghij(seed int) int {
    count := seed
    total := 0
    values := []int{count}
    lookup := map[string]int{"start": count}
    label := "alpha"
    _ = values
    _ = lookup
    _ = label
    for i := 0; i < 7; i++ {
        total += i
    count += 1
    }
    count += 39
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

build_bapc seed =
    let count = seed + 36
        total = count * 1
        values = [1 .. 3]
    in if count > 13
        then total + sum values
        else total - count

update_jfhk seed =
    case seed `mod` 3 of
        0 -> seed + 65
        1 -> seed * 54
        _ -> seed - 7

compute_fbpg seed =
    let count = seed + 1
        total = count * 51
        values = [1 .. 32]
    in if count > 32
        then total + sum values
        else total - count

decode_cdog seed =
    let count = seed + 6
        total = count * 1
        values = [1 .. 54]
    in if count > 6
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

decode_eiip seed =
    let count = seed + 9
        total = count * 2
        values = [1 .. 1]
    in if count > 5
        then total + sum values
        else total - count

apply_idga seed =
    let count = seed + 7
        total = count * 17
        values = [1 .. 1]
    in if count > 2
        then total + sum values
        else total - count

handle_nbia seed =
    let count = seed + 7
        total = count * 88
        values = [1 .. 82]
    in if count > 2
        then total + sum values
        else total - count

decode_doil seed =
    let count = seed + 95
        total = count * 1
        values = [1 .. 43]
    in if count > 8
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

check_egbn seed =
    let count = seed + 35
        total = count * 82
        values = [1 .. 3]
    in if count > 6
        then total + sum values
        else total - count

decode_iiha seed =
    let count = seed + 12
        total = count * 87
        values = [1 .. 45]
    in if count > 4
        then total + sum values
        else total - count

visit_gmen seed =
    let count = seed + 1
        total = count * 16
        values = [1 .. 8]
    in if count > 3
        then total + sum values
        else total - count

parse_mdna seed =
    let count = seed + 7
        total = count * 21
        values = [1 .. 33]
    in if count > 43
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
        <h2>team</h2>
        <p>feature</p>
        <p>html</p>
        <a href="#read-more">Read more</a>
    </article>

    <p>grammar product grammar</p>
    <figure>
        <img src="image-13.png" alt="world">
        <figcaption>html</figcaption>
    </figure>

    <article class="post">
        <h2>hello</h2>
        <p>feature</p>
        <p>hello</p>
        <a href="#read-more">Read more</a>
    </article>

    <form action="#" method="post">
        <label for="name">Name</label>
        <input id="name" name="name" type="text" placeholder="team">
        <label for="email">Email</label>
        <input id="email" name="email" type="email" placeholder="you@example.com">
        <button type="submit">Submit</button>
    </form>

    <section class="cards">
        <h2>resrap</h2>
        <div class="card">
            <h3>resrap</h3>
            <p>team</p>
        </div>
        <div class="card">
            <h3>product</h3>
            <p>product</p>
        </div>
    </section>

    <section class="cards">
        <h2>world</h2>
        <div class="card">
            <h3>html</h3>
            <p>team</p>
        </div>
        <div class="card">
            <h3>markup</h3>
            <p>world</p>
        </div>
    </section>

    <section class="cards">
        <h2>world</h2>
        <div class="card">
            <h3>product</h3>
            <p>generated</p>
        </div>
        <div class="card">
            <h3>generated</h3>
            <p>product</p>
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
            <tr><td>feature</td><td>3</td><td>active</td></tr>
            <tr><td>team</td><td>8</td><td>pending</td></tr>
        </tbody>
    </table>

    <article class="post">
        <h2>docs</h2>
        <p>generated</p>
        <p>team</p>
        <a href="#read-more">Read more</a>
    </article>

    <article class="post">
        <h2>hello</h2>
        <p>generated</p>
        <p>resrap</p>
        <a href="#read-more">Read more</a>
    </article>

    <section class="cards">
        <h2>team</h2>
        <div class="card">
            <h3>docs</h3>
            <p>product</p>
        </div>
        <div class="card">
            <h3>markup</h3>
            <p>markup</p>
        </div>
    </section>

    <details>
        <summary>resrap</summary>
        <p>resrap</p>
    </details>

    <details>
        <summary>feature</summary>
        <p>markup</p>
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
        <p>world</p>
    </details>

    <section class="cards">
        <h2>feature</h2>
        <div class="card">
            <h3>world</h3>
            <p>hello</p>
        </div>
        <div class="card">
            <h3>resrap</h3>
            <p>product</p>
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
        <input id="name" name="name" type="text" placeholder="feature">
        <label for="email">Email</label>
        <input id="email" name="email" type="email" placeholder="you@example.com">
        <button type="submit">Submit</button>
    </form>

    <p>feature generated html</p>
    <p>team feature docs</p>
    <ul class="feature-list">
        <li>product</li>
    </ul>
    <section id="home" class="hero">
        <h1>code</h1>
        <p>world</p>
        <a class="button" href="#features">Get started</a>
    </section>

    <p>team generated html</p>
    <section class="cards">
        <h2>hello</h2>
        <div class="card">
            <h3>team</h3>
            <p>code</p>
        </div>
        <div class="card">
            <h3>generated</h3>
            <p>code</p>
        </div>
    </section>

    <p>hello markup docs</p>
    <p>product code markup</p>
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

    static class Type_ibap {
        int id;
        String name = "item";
        double value = 1.5;
        int score() { return id + name.length(); }
    }

    private static int handle_odme(int seed) {
        int count = seed;
        int total = 0;
        List<Integer> values = new ArrayList<>();
        Map<String, Integer> lookup = new HashMap<>();
        String label = "alpha";
        System.out.println(label + ": " + count + " " + total);
        for (int i = 0; i < 4; i++) {
            total += i;
        total += count * 2;
        System.out.println(label + ": " + count + " " + total);
        }
        return total;
    }

    private static int handle_ppla(int seed) {
        int count = seed;
        int total = 0;
        List<Integer> values = new ArrayList<>();
        Map<String, Integer> lookup = new HashMap<>();
        String label = "alpha";
        count += 51;
        System.out.println(label + ": " + count + " " + total);
        if (count > 2) {
        total += values.size();
        total = total * 2 + count;
            total += 2;
        } else {
            total -= 1;
        }
        total += values.size();
        return total;
    }

    private static int merge_eedg(int seed) {
        int count = seed;
        int total = 0;
        List<Integer> values = new ArrayList<>();
        Map<String, Integer> lookup = new HashMap<>();
        String label = "alpha";
        System.out.println(label + ": " + count + " " + total);
        values.add(count + 1);
        try {
            total += Integer.parseInt("77");
        } catch (NumberFormatException e) {
            total += 1;
        }
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

    static int decode_eiip(int seed) {
        int count = seed;
        int total = 0;
        List<Integer> values = new ArrayList<>();
        Map<String, Integer> lookup = new HashMap<>();
        String label = "alpha";
        lookup.put(label, count);
        return total + seed;
    }

    private static int build_gjng(int seed) {
        int count = seed;
        int total = 0;
        List<Integer> values = new ArrayList<>();
        Map<String, Integer> lookup = new HashMap<>();
        String label = "alpha";
        lookup.put(label, count);
        switch (count % 3) {
            case 0: total += 1; break;
            case 1: total += 2; break;
            default: total += 3; break;
        }
        return total;
    }

    private static int compute_peec(int seed) {
        int count = seed;
        int total = 0;
        List<Integer> values = new ArrayList<>();
        Map<String, Integer> lookup = new HashMap<>();
        String label = "alpha";
        for (int i = 0; i < 58; i++) {
            total += i;
        }
        return total;
    }

    static int compute_iecj(int seed) {
        int count = seed;
        int total = 0;
        List<Integer> values = new ArrayList<>();
        Map<String, Integer> lookup = new HashMap<>();
        String label = "alpha";
        total += count * 17;
        return total + seed;
    }

    private static int visit_blad(int seed) {
        int count = seed;
        int total = 0;
        List<Integer> values = new ArrayList<>();
        Map<String, Integer> lookup = new HashMap<>();
        String label = "alpha";
        try {
            total += Integer.parseInt("46");
        } catch (NumberFormatException e) {
            total += 1;
        }
        try {
            total += Integer.parseInt("3");
        } catch (NumberFormatException e) {
            total += 1;
        }
        for (int i = 0; i < 86; i++) {
            total += i;
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

    static int check_egbn(int seed) {
        int count = seed;
        int total = 0;
        List<Integer> values = new ArrayList<>();
        Map<String, Integer> lookup = new HashMap<>();
        String label = "alpha";
        switch (count % 3) {
            case 0: total += 1; break;
            case 1: total += 2; break;
            default: total += 3; break;
        }
        return total + seed;
    }

    private static int decode_jkod(int seed) {
        int count = seed;
        int total = 0;
        List<Integer> values = new ArrayList<>();
        Map<String, Integer> lookup = new HashMap<>();
        String label = "alpha";
        System.out.println(label + ": " + count + " " + total);
        count += 83;
        return total;
    }

    interface Type_pafd {
        String describe();
        int score(int seed);
    }

    interface Type_ohek {
        String describe();
        int score(int seed);
    }

    private static int process_plip(int seed) {
        int count = seed;
        int total = 0;
        List<Integer> values = new ArrayList<>();
        Map<String, Integer> lookup = new HashMap<>();
        String label = "alpha";
        System.out.println(label + ": " + count + " " + total);
        for (int value : values) {
            total += value;
        }
        for (int value : values) {
            total += value;
        }
        return total;
    }

    static int process_bfee(int seed) {
        int count = seed;
        int total = 0;
        List<Integer> values = new ArrayList<>();
        Map<String, Integer> lookup = new HashMap<>();
        String label = "alpha";
        total = total * 2 + count;
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
function parse_apcm(seed) {
    let count = seed;
    let total = 0;
    const values = [];
    const lookup = new Map();
    const label = "alpha";
    console.log(label, count, total);
    return total;
}

const CONFIG_egg = {
    id: 52,
    name: "omega",
    value: 8.57,
    items: [1, 2, 3],
    describe() {
        return this.name + ":" + this.id;
    }
};

function handle_dfca(seed) {
    let count = seed;
    let total = 0;
    const values = [];
    const lookup = new Map();
    const label = "alpha";
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
    count += 51;
    console.log(label, count, total);
    if (count > 2) {
    total += values.length;
    total = total * 2 + count;
        total += 2;
    } else {
        total -= 1;
    }
    total += values.length;
    return total;
}

function merge_eedg(seed) {
    let count = seed;
    let total = 0;
    const values = [];
    const lookup = new Map();
    const label = "alpha";
    console.log(label, count, total);
    values.push(count + 1);
    try {
        if (count < 0) throw new Error("negative");
        total += count;
    } catch (error) {
        total += 77;
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
function collect_iipj(seed) {
    let count = seed;
    let total = 0;
    const values = [];
    const lookup = new Map();
    const label = "alpha";
    for (const value of values) {
        total += value;
    }
    return total;
}

function parse_jnga(seed) {
    let count = seed;
    let total = 0;
    const values = [];
    const lookup = new Map();
    const label = "alpha";
    console.log(label, count, total);
    return total;
}

const CONFIG_lai = {
    id: 68,
    name: "delta",
    value: 3.98,
    items: [1, 2, 3],
    describe() {
        return this.name + ":" + this.id;
    }
};

class Type_ommn {
    constructor(seed) {
        this.seed = seed;
        this.label = "alpha";
    }

    score_pc(amount) {
        let count = amount;
        let total = this.seed;
        const values = [];
        const lookup = new Map();
        const label = this.label;
        return this.seed + amount;
    }
}

class Type_ddck {
    constructor(seed) {
        this.seed = seed;
        this.label = "alpha";
    }

    score_pl(amount) {
        let count = amount;
        let total = this.seed;
        const values = [];
        const lookup = new Map();
        const label = this.label;
        return this.seed + amount;
    }
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
function split_gbni(seed) {
    let count = seed;
    let total = 0;
    const values = [];
    const lookup = new Map();
    const label = "alpha";
    try {
        if (count < 0) throw new Error("negative");
        total += count;
    } catch (error) {
        total += 2;
    }
    return total;
}

class Type_edib {
    constructor(seed) {
        this.seed = seed;
        this.label = "alpha";
    }

    score_ha(amount) {
        let count = amount;
        let total = this.seed;
        const values = [];
        const lookup = new Map();
        const label = this.label;
        return this.seed + amount;
    }
}

function encode_ipaf(seed) {
    let count = seed;
    let total = 0;
    const values = [];
    const lookup = new Map();
    const label = "alpha";
    total = total * 2 + count;
    if (count > 3) {
    console.log(label, count, total);
        total += 1;
    } else {
        total -= 1;
    }
    return total;
}

function merge_ghij(seed) {
    let count = seed;
    let total = 0;
    const values = [];
    const lookup = new Map();
    const label = "alpha";
    for (let i = 0; i < 7; i++) {
        total += i;
    count += 1;
    }
    count += 39;
    return total;
}


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

def parse_apcm(seed):
    count = seed
    total = 0
    values = []
    lookup = {}
    label = "alpha"
    print(label, count, total)
    return total

class Type_eggc:
    def __init__(self, seed):
        self.seed = seed
        self.label = "alpha"

    def score_cj(self, amount):
        count = self.seed + amount
        total = 0
        values = [count, amount, self.seed]
        for value in values:
            total += value
        return total + len(self.label)

class Type_dakb:
    def __init__(self, seed):
        self.seed = seed
        self.label = "alpha"

    def score_pe(self, amount):
        count = self.seed + amount
        total = 0
        values = [count, amount, self.seed]
        for value in values:
            total += value
        return total + len(self.label)

def check_pgig(seed):
    count = seed
    total = 0
    values = []
    lookup = {}
    label = "alpha"
    for i in range(89):
        total += i
    return total

def apply_kicd(seed):
    count = seed
    total = 0
    values = []
    lookup = {}
    label = "alpha"
    total += count * 2
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

def collect_iipj(seed):
    count = seed
    total = 0
    values = []
    lookup = {}
    label = "alpha"
    for value in values:
        total += value
    return total

def parse_jnga(seed):
    count = seed
    total = 0
    values = []
    lookup = {}
    label = "alpha"
    print(label, count, total)
    return total

class Type_laia:
    def __init__(self, seed):
        self.seed = seed
        self.label = "alpha"

    def score_kb(self, amount):
        count = self.seed + amount
        total = 0
        values = [count, amount, self.seed]
        for value in values:
            total += value
        return total + len(self.label)

class Type_biah:
    def __init__(self, seed):
        self.seed = seed
        self.label = "alpha"

    def score_jd(self, amount):
        count = self.seed + amount
        total = 0
        values = [count, amount, self.seed]
        for value in values:
            total += value
        return total + len(self.label)

def compute_cdbm(seed):
    count = seed
    total = 0
    values = []
    lookup = {}
    label = "alpha"
    count += 57
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

def split_gbni(seed):
    count = seed
    total = 0
    values = []
    lookup = {}
    label = "alpha"
    total += sum(x * x for x in range(2))
    return total

class Type_edib:
    def __init__(self, seed):
        self.seed = seed
        self.label = "alpha"

    def score_ha(self, amount):
        count = self.seed + amount
        total = 0
        values = [count, amount, self.seed]
        for value in values:
            total += value
        return total + len(self.label)

class Type_lnjf:
    def __init__(self, seed):
        self.seed = seed
        self.label = "alpha"

    def score_fd(self, amount):
        count = self.seed + amount
        total = 0
        values = [count, amount, self.seed]
        for value in values:
            total += value
        return total + len(self.label)

class Type_ohek:
    def __init__(self, seed):
        self.seed = seed
        self.label = "alpha"

    def score_gd(self, amount):
        count = self.seed + amount
        total = 0
        values = [count, amount, self.seed]
        for value in values:
            total += value
        return total + len(self.label)

class Type_nefl:
    def __init__(self, seed):
        self.seed = seed
        self.label = "alpha"

    def score_ij(self, amount):
        count = self.seed + amount
        total = 0
        values = [count, amount, self.seed]
        for value in values:
            total += value
        return total + len(self.label)

def apply_bmcb(seed):
    count = seed
    total = 0
    values = []
    lookup = {}
    label = "alpha"
    print(label, count, total)
    if count > 94:
        total += count * 5
        values.append(count + 79)
        total += 1
    else:
        total -= 1
    if count > 21:
        values.append(count + 9)
        values.append(count + 2)
        total += 6
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

trait Type_eiba {
    fn describe(&self) -> String;
    fn score(&self, seed: i32) -> i32;
}

fn apply_iodm(seed: i32) -> i32 {
    let mut count = seed;
    let mut total = 0;
    let mut values = vec![count];
    let mut lookup: HashMap<String, i32> = HashMap::new();
    let label = String::from("alpha");
    count += 4;
    match count % 3 {
        0 => total += 1,
        1 => total += 2,
        _ => total += 3,
    }
    for i in 0..78 {
        total += i;
    values.push(count + 14);
    }
    println!("{} {} {}", label, count, total);
    total = total * 2 + count;
    total
}

trait Type_cbfh {
    fn describe(&self) -> String;
    fn score(&self, seed: i32) -> i32;
}

impl Counter {
    fn handle_dcmc(&self, seed: i32) -> i32 {
    let mut count = seed;
    let mut total = 0;
    let mut values = vec![count];
    let mut lookup: HashMap<String, i32> = HashMap::new();
    let label = String::from("alpha");
    println!("{} {} {}", label, count, total);
        total + self.value
    }
}

impl Counter {
    fn render_hahg(&self, seed: i32) -> i32 {
    let mut count = seed;
    let mut total = 0;
    let mut values = vec![count];
    let mut lookup: HashMap<String, i32> = HashMap::new();
    let label = String::from("alpha");
    for value in &values {
        total += *value;
    }
    println!("{} {} {}", label, count, total);
        total + self.value
    }
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

struct Type_khei {
    id: i32,
    name: String,
    value: f64,
    items: Vec<i32>,
}

struct Type_jchl {
    id: i32,
    name: String,
    value: f64,
    items: Vec<i32>,
}

fn process_jhdc(seed: i32) -> i32 {
    let mut count = seed;
    let mut total = 0;
    let mut values = vec![count];
    let mut lookup: HashMap<String, i32> = HashMap::new();
    let label = String::from("alpha");
    total += count * 6;
    total
}

fn parse_okbc(seed: i32) -> i32 {
    let mut count = seed;
    let mut total = 0;
    let mut values = vec![count];
    let mut lookup: HashMap<String, i32> = HashMap::new();
    let label = String::from("alpha");
    for value in &values {
        total += *value;
    }
    total
}

impl Counter {
    fn visit_dagi(&self, seed: i32) -> i32 {
    let mut count = seed;
    let mut total = 0;
    let mut values = vec![count];
    let mut lookup: HashMap<String, i32> = HashMap::new();
    let label = String::from("alpha");
    values.sort();
    total += values[0];
        total + self.value
    }
}

fn split_oilc(seed: i32) -> i32 {
    let mut count = seed;
    let mut total = 0;
    let mut values = vec![count];
    let mut lookup: HashMap<String, i32> = HashMap::new();
    let label = String::from("alpha");
    total += values.len() as i32;
    values.sort();
    total += values[0];
    values.sort();
    total += values[0];
    match count % 3 {
        0 => total += 1,
        1 => total += 2,
        _ => total += 3,
    }
    total
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
    fn process_megb(&self, seed: i32) -> i32 {
    let mut count = seed;
    let mut total = 0;
    let mut values = vec![count];
    let mut lookup: HashMap<String, i32> = HashMap::new();
    let label = String::from("alpha");
    total += count * 48;
    println!("{} {} {}", label, count, total);
        total + self.value
    }
}

trait Type_ibdl {
    fn describe(&self) -> String;
    fn score(&self, seed: i32) -> i32;
}

impl Counter {
    fn update_gnln(&self, seed: i32) -> i32 {
    let mut count = seed;
    let mut total = 0;
    let mut values = vec![count];
    let mut lookup: HashMap<String, i32> = HashMap::new();
    let label = String::from("alpha");
    match count % 3 {
        0 => total += 1,
        1 => total += 2,
        _ => total += 3,
    }
    total += lookup.get("start").copied().unwrap_or(0);
    for i in 0..38 {
        total += i;
    println!("{} {} {}", label, count, total);
    }
    for value in &values {
        total += *value;
    }
        total + self.value
    }
}

fn merge_ghij(seed: i32) -> i32 {
    let mut count = seed;
    let mut total = 0;
    let mut values = vec![count];
    let mut lookup: HashMap<String, i32> = HashMap::new();
    let label = String::from("alpha");
    for i in 0..7 {
        total += i;
    count += 1;
    }
    count += 39;
    total
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
        expected: r#"SELECT updated_at FROM events;
SELECT * FROM items WHERE amount < 28.68 ORDER BY total DESC LIMIT 8;
SELECT events.created_at, orders.value FROM items JOIN items ON items.id = events.id WHERE updated_at != NULL AND status < NULL AND name != NULL AND value >= 55;
SELECT records.email, orders.value FROM payments JOIN users ON users.id = users.id WHERE status = 'resrap' OR email != 'hello';
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
        expected: r#"SELECT sessions.status, payments.quantity FROM invoices JOIN sessions ON accounts.id = accounts.id WHERE amount LIKE 3.1 AND email LIKE 6;
SELECT * FROM accounts WHERE name LIKE 56.98 AND email LIKE 88.2 OR id >= 1.8 ORDER BY updated_at DESC LIMIT 7;
UPDATE orders SET id = TRUE, total = 6.1 WHERE created_at <= 2.5 OR total > TRUE;
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
        expected: r#"CREATE TABLE accounts (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    value INTEGER DEFAULT 1,
    status REAL
);
SELECT accounts.name, products.value FROM products JOIN sessions ON products.id = products.id WHERE created_at > 'resrap' AND created_at != FALSE;
SELECT payments.status, items.id FROM payments JOIN events ON orders.id = events.id WHERE created_at > 6;
SELECT accounts.quantity, orders.value FROM accounts JOIN records ON invoices.id = items.id WHERE status = 'resrap' AND name LIKE 'query';
"#,
    },
];
