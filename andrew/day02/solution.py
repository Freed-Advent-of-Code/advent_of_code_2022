part_one_mapping = {
    "A X": 3 + 1,  # 4
    "A Y": 6 + 2,  # 8
    "A Z": 0 + 3,  # 3
    "B X": 0 + 1,  # 1
    "B Y": 3 + 2,  # 5
    "B Z": 6 + 3,  # 9
    "C X": 6 + 1,  # 7
    "C Y": 0 + 2,  # 2
    "C Z": 3 + 3,  # 6
}


part_two_mapping = {
    "A X": 0 + 3,
    "A Y": 3 + 1,
    "A Z": 6 + 2,
    "B X": 0 + 1,
    "B Y": 3 + 2,
    "B Z": 6 + 3,
    "C X": 0 + 2,
    "C Y": 3 + 3,
    "C Z": 6 + 1,
}


def calculate_score(mapping: dict):
    file = open("./input.txt", "r")
    total_score = 0
    for line in file:
        round = "".join(line.splitlines())
        score = mapping[round]
        total_score += score
    return total_score


if __name__ == "__main__":
    print("part one:", calculate_score(part_one_mapping))
    print("part two:", calculate_score(part_two_mapping))
