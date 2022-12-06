def part_one():
    win = [(1, 2), (3, 1), (2, 3)]
    strategy = {"A": 1, "B": 2, "C": 3, "X": 1, "Y": 2, "Z": 3}
    score = []
    with open("input.txt", "r") as f:
        for line in f.readlines():
            a, b = line.strip().split()
            if (strategy[a], strategy[b]) in win:
                score.append(strategy[b] + 6)
            elif strategy[a] == strategy[b]:
                score.append(strategy[b] + 3)
            else:
                score.append(strategy[b])
        return sum(score)


def part_two():
    with open("input.txt", "r") as f:
        win = [(1, 2), (3, 1), (2, 3)]
        strategy = {"A": 1, "B": 2, "C": 3}
        score = []
        for line in f.readlines():
            a, b = line.strip().split()
            if b == "X":
                score.append([i for i in win if i[1] == strategy[a]][0][0])
            elif b == 'Y':
                score.append(3 + strategy[a])
            else:
                score.append(6 + [i for i in win if i[0] == strategy[a]][0][1])
        return sum(score)


if __name__ == '__main__':
    print(part_one())
    print(part_two())
