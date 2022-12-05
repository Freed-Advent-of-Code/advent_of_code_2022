from typing import TextIO


def generate_elves_total_calories(text: TextIO) -> list[int]:
    elf_total_calories_list = []
    one_elf_total_calories = 0
    for line in text.read().split('\n'):
        if line == '':
            elf_total_calories_list.append(one_elf_total_calories)
            one_elf_total_calories = 0
        else:
            one_elf_total_calories += int(line)

    elf_total_calories_list.sort(reverse=True)        
    return elf_total_calories_list


if __name__ == '__main__':
    with open('luca/day_01/input.txt') as f:
        print(f'part1: {generate_elves_total_calories(f)[0]}')
        f.seek(0)
        print(f'part2: {sum(generate_elves_total_calories(f)[:3])}')