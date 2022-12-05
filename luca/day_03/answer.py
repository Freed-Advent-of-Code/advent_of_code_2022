from typing import TextIO


def generate_common_alphas_part1(text: TextIO) -> list[str]:
    return [(set(list(line[:len(line)//2])) & set(list(line[len(line)//2:]))).pop() for line in text.read().split('\n') if line]


def generate_common_alphas_part2(text: TextIO) -> list[str]:
    text_lines = text.read().split('\n')
    return [(set(text_lines[i*3:i*3+3][0]) & set(text_lines[i*3:i*3+3][1]) & set(text_lines[i*3:i*3+3][2])).pop() for i in range(len(text_lines)//3)]


def calculate_common_alpahs(generate_common_alphas: list[str]) -> int:
    result = 0 
    for alpha in generate_common_alphas:
        if alpha.islower():
            result += (ord(alpha) - 96)
        else:
            result += (ord(alpha) - 38)

    return result

if __name__ == '__main__':
    with open('luca/day_03/input.txt') as f:
        p1 = generate_common_alphas_part1(text=f)
        f.seek(0)
        p2 = generate_common_alphas_part2(text=f)

        print(calculate_common_alpahs(p1))
        print(calculate_common_alpahs(p2))
