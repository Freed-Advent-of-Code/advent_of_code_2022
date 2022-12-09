def calculate_index(total_length: int):
    file = open("./input.txt", "r")
    data = file.read()

    for i in range((total_length - 1), len(data)):
        if (
            len(set([data[x] for x in range(i - (total_length - 1), i + 1)]))
            == total_length
        ):
            return i + 1


if __name__ == "__main__":
    print("part one:", calculate_index(4))
    print("part two:", calculate_index(14))
