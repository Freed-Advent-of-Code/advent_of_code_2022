from collections import defaultdict


def get_crates(reader):
    crates = defaultdict(list)
    while True:
        line = reader.readline()
        if line.startswith(' 1'):
            break
        stripped = line.rstrip('\n')
        i = 1
        while stripped:
            crate = stripped[:4].strip()
            if crate:
                crates[i].append(crate[1])
            stripped = stripped[4:]
            i += 1

    return crates


def move(crates, n, from_, to):
    for _ in range(n):
        crate = crates[from_].pop(0)
        crates[to].insert(0, crate)


def solution(reader):
    crates = get_crates(reader)
    next(reader)
    for command_line in reader:
        split_cmd = command_line.split()
        command = [int(comm) for comm in split_cmd[1::2]]
        move(crates, *command)

    return "".join(crates[i][0] for i in range(1, len(crates) + 1))


if __name__ == '__main__':
    with open('../test_input.txt') as f:
        got = solution(f)
        assert got == 'CMZ'

    with open('../input.txt') as f:
        print(solution(f))
