def part_one():
    file = open("./input.txt", "r")
    data = file.read()
    data_into_list = data.splitlines()

    count = 0
    for line in data_into_list:
        a, b = [x.split("-") for x in line.split(",")]

        if (
            int(a[0]) <= int(b[0])
            and int(a[1]) >= int(b[1])
            or int(a[0]) >= int(b[0])
            and int(a[1]) <= int(b[1])
        ):
            count += 1

    return count


def part_two():
    file = open("./input.txt", "r")
    data = file.read()
    data_into_list = data.splitlines()
    count = 0
    for line in data_into_list:
        a, b = [x.split("-") for x in line.split(",")]
        a_range = [x for x in range(int(a[0]), int(a[1]) + 1)]
        b_range = [x for x in range(int(b[0]), int(b[1]) + 1)]

        if set(a_range).intersection(b_range):
            count += 1

    return count


if __name__ == "__main__":
    print("part one:", part_one())
    print("part two:", part_two())
