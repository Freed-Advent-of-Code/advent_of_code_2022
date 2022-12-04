import re
from functools import reduce


def solution(text, n):
    return sum([([0] * 65 + list(range(27, 53)) + [0] * 6 + list(range(1, 27)))[ord(err.pop())] for err in [reduce(lambda x, y: x & y, map(set, group.split('\n'))) for group in re.findall(r"([^\n]+\n[^\n]+\n[^\n]+)", text.read())]])


if __name__ == '__main__':
    with open('../test_input.txt') as f:
        got = solution(f, 3)
        assert got == 70

    with open('../input.txt') as f:
        print(solution(f, 3))
