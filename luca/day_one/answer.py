from typing import TextIO


def generate_total_calories(text: TextIO) -> list[int]:
    elf_total_calories_list = []
    one_elf_total_calories = 0
    for line in text.readlines():
        if line[:-1] == '':
            elf_total_calories_list.append(one_elf_total_calories)
            one_elf_total_calories = 0
        else:
            one_elf_total_calories += int(line[:-1])

    elf_total_calories_list.sort(reverse=True)        
    return elf_total_calories_list


if __name__ == '__main__':
    with open('luca/day_one/input.txt') as f:
        print(generate_total_calories(f)[0])
        f.seek(0)
        print(sum(generate_total_calories(f)[:3]))