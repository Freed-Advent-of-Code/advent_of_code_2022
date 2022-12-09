import re
import copy
CRATES = {
    1: ["C", "Z", "N", "B", "M", "W", "Q", "V"],
    2: ["H", "Z", "R", "W", "C", "B"],
    3: ["F", "Q", "R", "J"],
    4: ["Z", "S", "W", "H", "F", "N", "M", "T"],
    5: ["G", "F", "W", "L", "N", "Q", "P"],
    6: ["L", "P", "W"],
    7: ["V", "B", "D", "R", "G", "C", "Q", "J"],
    8: ["Z", "Q", "N", "B", "W"],
    9: ["H", "L", "F", "C", "G", "T", "J"]
}

def part_one():
    crates = copy.deepcopy(CRATES)

    for e in open('input.txt', 'r').readlines():
        e = eval(re.sub(r"move (\d+) from (\d+) to (\d+)", r"[\1,\2,\3]", e.strip()))
        for f in range(e[0]):
            crates[e[2]].append(crates[e[1]].pop())

    return "".join([y.pop() for x,y in crates.items()])
    
print(part_one())


def part_two():
    crates = copy.deepcopy(CRATES)

    for e in open('input.txt', 'r').readlines():
        e = eval(re.sub(r"move (\d+) from (\d+) to (\d+)", r"[\1,\2,\3]", e.strip()))
        crates[e[2]] += [crates[e[1]].pop() for x in range(e[0])][::-1]

    return "".join([y.pop() for x,y in crates.items()])
  
print(part_two())