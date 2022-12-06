def part_one():
    with open("input.txt", "r") as f:
        calories = []
        one_elf = []
        for line in f.readlines():
            if line == "\n":
                calories.append(sum(one_elf))
                one_elf = []
            else:
                one_elf.append(int(line.strip()))
        return max(calories)


def part_two():
    with open("input.txt", "r") as f:
        calories = []
        one_elf = []
        for line in f.readlines():
            if line == "\n":
                calories.append(sum(one_elf))
                one_elf = []
            else:
                one_elf.append(int(line.strip()))
        return sum(sorted(calories, reverse=True)[:3])


if __name__ == '__main__':
    print(part_one())
    print(part_two())
