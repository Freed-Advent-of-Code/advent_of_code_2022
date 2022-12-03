
def calculate(reader):
    return sum([[3, 6, 0][[0, -1, 1][sum(map(lambda x: ord(x) - 65 if ord(x) < 68 else -(ord(x) - 88), line.split()))]] + ord(line.split()[1]) - 87 for line in reader.readlines()])


if __name__ == '__main__':
    with open('test_input.txt') as f:
        assert calculate(f) == 15

    with open('input.txt') as f:
        print(calculate(f))

