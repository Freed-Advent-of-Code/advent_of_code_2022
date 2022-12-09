def calculate_value(letter: str):
    assert letter.isalpha()
    try:
        if letter.islower():
            return ord(letter) - 96
        else:
            return ord(letter) - 38
    except Exception:
        print(letter)


def part_one():
    file = open("./input.txt", "r")
    total = 0
    for line in file:
        extracted = "".join(line.splitlines())
        half_len = int(len(extracted) / 2)

        a = extracted[:half_len]
        b = extracted[half_len:]

        common = "".join(set(a).intersection(b))
        total += calculate_value(common)

    return total


def part_two():
    file = open("./input.txt", "r")
    data = file.read()
    data_into_list = data.splitlines()
    total = 0
    for i in range(2, len(data_into_list), 3):
        a = data_into_list[i - 2]
        b = data_into_list[i - 1]
        c = data_into_list[i]
        _common = "".join(set(a).intersection(b))
        common = "".join(set(_common).intersection(c))

        total += calculate_value(common)

    return total


if __name__ == "__main__":
    print("part one:", part_one())
    print("part two:", part_two())
