def solution(text):
    return [a[0] >= b[0] and a[1] <= b[1] or a[0] <= b[0] and a[1] >= b[1] for a, b in [list(map(lambda x: list(map(int, x.split('-'))), line.split(','))) for line in text.read().split('\n')]].count(True)


if __name__ == '__main__':
    with open('../test_input.txt') as f:
        got = solution(f)
        assert got == 2

    with open('../input.txt') as f:
        print(solution(f))
