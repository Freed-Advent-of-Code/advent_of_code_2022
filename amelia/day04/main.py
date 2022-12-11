import re


def part_one():
    return len([[int(i[0]), int(i[1]), int(i[2]), int(i[3])] for i in [re.split('[,-]', x) for x in open('input.txt', 'r').read().split('\n')] if int(i[0]) <= int(i[2]) and int(i[1]) >= int(i[3]) or int(i[0]) >= int(i[2]) and int(i[1]) <= int(i[3])])


def part_two():
    return [i if int(i[0]) < int(i[2]) and int(i[1]) < int(i[2]) or int(i[0]) > int(i[2]) and int(i[0]) > int(i[3]) else 'duplicated' for i in [re.split('[,-]', x) for x in open('input.txt', 'r').read().split('\n')]].count('duplicated')


if __name__ == '__main__':
    print(part_one())
    print(part_two())
