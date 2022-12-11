
def get_cycle(data):
    cycle = [1]
    for e in data:
        if e[0] == "noop":
            cycle.append(cycle[-1])
        elif e[0] == "addx":
            cycle.append(cycle[-1])
            cycle.append(cycle[-1]+int(e[1]))
    return cycle[:-1]


def part_one():
    data = [x.strip().split(" ") for x in open('input.txt', 'r').readlines()]
    cycle = get_cycle(data)
    values = [20, 60, 100, 140, 180, 220]
    return sum(cycle[x-1] * x for x in values)

print(part_one())


def get_sprite(n):
    return [min(n+x, 40) for x in range(3)]


def part_two():
    data = [x.strip().split(" ") for x in open('input.txt', 'r').readlines()]
    cycle = get_cycle(data)
    crt = []
    for e in cycle:
        if len(crt) % 40 + 1 in get_sprite(e):
            crt.append("#")
        else:
            crt.append(".")
    for i in range(0, len(cycle), 40):
        print(crt[i:i+40])
    return

part_two()