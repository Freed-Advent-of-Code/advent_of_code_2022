import re


def part_one(crates):
    crates = crates
    datas = [re.findall(r'\d+', x) for x in open("input.txt", "r").read().split("\n")[10:]]
    for data in datas:
        for _ in range(int(data[0])):
            crates[int(data[2])].append(crates[int(data[1])].pop())
    return "".join([x[-1] for x in crates.values()])


def part_two(crates):
    crates = crates
    datas = [re.findall(r'\d+', x) for x in open("input.txt", "r").read().split("\n")[10:]]
    for data in datas:
        a = crates[int(data[1])][-int(data[0]):]
        crates[int(data[2])].append(a)
    return "".join([x[-1] for x in crates.values()])


if __name__ == '__main__':
    crates = {
        1: ["R", "N", "P", "G"],
        2: ["T", "J", "B", "L", "C", "S", "V", "H"],
        3: ["T", "D", "B", "M", "N", "L"],
        4: ["R", "V", "P", "S", "B"],
        5: ["G", "C", "Q", "S", "W", "M", "V", "H"],
        6: ["W", "Q", "S", "C", "D", "B", "J"],
        7: ["F", "Q", "L"],
        8: ["W", "M", "H", "T", "D", "L", "F", "V"],
        9: ["L", "P", "B", "V", "M", "J", "F"]
    }
    print(part_one(crates))
    print(part_two(crates))
