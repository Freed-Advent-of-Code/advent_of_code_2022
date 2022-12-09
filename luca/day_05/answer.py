from typing import TextIO
from copy import deepcopy


def move_crates_part1(text: TextIO, data: list[list]) -> str:
    for line in text.read().split('\n'):
        command = line.split()
        how_many, from_crate, to_crate = [int(command[2*x+1]) for x in range(3)]
        for _ in range(how_many):
            data[to_crate].append(data[from_crate].pop())

    return ''.join([data[x][-1] for x in range(1, 10)])

def move_crates_part2(text: TextIO, data: list[list]) -> str:
    for line in text.read().split('\n'):
        command = line.split()
        how_many, from_crate, to_crate = [int(command[2*x+1]) for x in range(3)]
        temporary_crate = []
        for _ in range(how_many):
            temporary_crate.append(data[from_crate].pop())
        while temporary_crate:
            data[to_crate].append(temporary_crate.pop())

    return ''.join([data[x][-1] for x in range(1, 10)])



if __name__ == '__main__':
    data1 = [
    [],
    ['F', 'C', 'J', 'P', 'H', 'T', 'W'], 
    ['G', 'R', 'V', 'F', 'Z', 'J', 'B', 'H'], 
    ['H', 'P', 'T', 'R'], ['Z', 'S', 'N', 'P', 'H', 'T'], 
    ['N', 'V', 'F', 'Z', 'H', 'J', 'C', 'D'], 
    ['P', 'M', 'G', 'F', 'W', 'D', 'Z'],
    ['M', 'V', 'Z', 'W', 'S', 'J', 'D', 'P'],
    ['N', 'D', 'S'],
    ['D', 'Z', 'S', 'F', 'M']
    ]
    data2 = deepcopy(data1)
    with open('luca/day_05/input.txt') as f:
        print(move_crates_part1(f, data1))
        f.seek(0)
        print(move_crates_part2(f, data2))

