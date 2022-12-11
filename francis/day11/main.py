

class Monkey:
    def __init__(self, number: int, items: list, operation: str, divisible_test: int, when_true: int, when_false: int) -> None:
        self.number = number
        self.items = items
        self.operation = lambda old: eval(operation)
        self.divisible_test = divisible_test
        self.when_true = when_true
        self.when_false = when_false
        self.inspected = 0
    
    def inspect(self, n: int) -> int:
        self.inspected += 1
        return self.when_true if n % self.divisible_test == 0 else self.when_false

def find_monkeys(data: str) -> list:
    monkeys = []
    for number, monkey in enumerate(data.split("\n\n")):
        for line in monkey.split("\n"):
            if line.startswith("  Starting items: "):
                starting_items = list(map(int, line.split("  Starting items: ")[1].split(", ")))
            elif line.startswith("  Operation: new = "):
                operation = line.split("  Operation: new = ")[1]
            elif line.startswith("  Test: "):
                divisible_test = int(line.split("  Test: divisible by ")[1])
            elif line.startswith("    If true: throw to monkey "):
                when_true = int(line.split("    If true: throw to monkey ")[1])
            elif line.startswith("    If false: throw to monkey "):
                when_false = int(line.split("    If false: throw to monkey ")[1])
        

        monkeys.append(
            Monkey(
                number,
                starting_items,
                operation,
                divisible_test,
                when_true,
                when_false
            )
        )
    return monkeys

def part_one():
    with open('input.txt', 'r') as f:
        data = f.read()
    monkeys = find_monkeys(data)
    for round in range(20):
        for i, monkey in enumerate(monkeys):
            while monkey.items:
                item = monkey.items.pop(0)
                op_v = monkey.operation(item) // 3
                monkeys[monkey.inspect(op_v)].items.append(op_v)
    x, y = sorted([m.inspected for m in monkeys])[-2:]
    return x * y

print(part_one())


def part_two():
    with open('input.txt', 'r') as f:
        data = f.read()
    monkeys = find_monkeys(data)
    lcm = 1
    for monkey in monkeys:
        lcm = lcm * monkey.divisible_test
    for _ in range(10000):
        for monkey in monkeys:
            while monkey.items:
                item = monkey.items.pop(0)
                op_v = monkey.operation(item) % lcm
                monkeys[monkey.inspect(op_v)].items.append(op_v)
    x, y = sorted([m.inspected for m in monkeys])[-2:]
    return x * y

# 함수 실행 시간 측정하기
import time
start_time = time.time()
print(part_two())
print("--- %s seconds ---" % (time.time() - start_time))
