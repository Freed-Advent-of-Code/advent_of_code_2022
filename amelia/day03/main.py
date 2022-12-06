def part_one():
    return sum([ord(x)-38 if x.isupper() else ord(x)-96 for x in [list(set(line[len(line) // 2:]).intersection(set(line[:len(line) // 2])))[0] for line in open('input.txt', 'r').readlines()]])


def part_two():
    with open('input.txt', 'r') as f:
        read_data = [line.strip() for line in f.readlines()]
        return sum([ord(y[0])-38 if y[0].isupper() else ord(y[0])-96for y in [list(set(x[0]) & set(x[1]) & set(x[2])) for x in [read_data[i:i+3]for i in range(0, len(read_data), 3)]]])


if __name__ == '__main__':
    print(part_one())
    print(part_two())
