
def solution(text):
    return sum(sum(([0] * 65 + list(range(27, 53)) + [0] * 6 + list(range(1, 27)))[ord(err)] for err in (set(line[:len(line) // 2]) & set(line[len(line) // 2:]))) for line in text.readlines())


if __name__ == '__main__':
    with open('../test_input.txt') as f:
        got = solution(f)
        assert got == 157

    with open('../input.txt') as f:
        print(solution(f))
