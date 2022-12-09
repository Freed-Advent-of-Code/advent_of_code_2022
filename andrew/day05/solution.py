def parse_file():
    crates = {
        1: ["D", "B", "J", "V"],
        2: ["P", "V", "B", "W", "R", "D", "F"],
        3: ["R", "G", "F", "L", "D", "C", "W", "Q"],
        4: ["W", "J", "P", "M", "L", "N", "D", "B"],
        5: ["H", "N", "B", "P", "C", "S", "Q"],
        6: ["R", "D", "B", "S", "N", "G"],
        7: ["Z", "B", "P", "M", "Q", "F", "S", "H"],
        8: ["W", "L", "F"],
        9: ["S", "V", "F", "M", "R"],
    }
    file = open("./input.txt", "r")
    data = file.read()
    data_into_list = data.splitlines()

    return crates, data_into_list


def part_one():
    crates, instructions = parse_file()
    for instruction in instructions:
        arr = instruction.split(" ")
        for _ in range(int(arr[1])):
            moved_item = crates[int(arr[3])].pop()
            crates[int(arr[5])].append(moved_item)

    return "".join([crates[key].pop() for key in crates.keys()])


def part_two():
    crates, instructions = parse_file()
    for instruction in instructions:
        arr = instruction.split(" ")
        moved_items = [crates[int(arr[3])].pop() for _ in range(int(arr[1]))]
        moved_items.reverse()

        for item in moved_items:
            crates[int(arr[5])].append(item)

    return "".join([crates[key].pop() for key in crates.keys()])


if __name__ == "__main__":
    print("part one:", part_one())
    print("part two:", part_two())
