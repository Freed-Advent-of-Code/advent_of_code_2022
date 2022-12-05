from typing import TextIO


def generate_number_of_fully_contain_pair_part1(text: TextIO) -> int:
    answer = 0
    for line in text.read().split('\n'):
        if line:
            a, another = [x.split('-') for x in line.split(',')]
            if (int(a[0]) <= int(another[0]) and int(a[1]) >= int(another[1])) or (int(a[0]) >= int(another[0]) and int(a[1]) <= int(another[1])): # 완전 겹침
                answer += 1

    return answer

def generate_number_of_fully_contain_pair_part2(text: TextIO) -> int:
    answer = 0
    for line in text.read().split('\n'):
        if line:
            answer += 1
            a, another = [x.split('-') for x in line.split(',')]
            if int(a[1]) < int(another[0]) or int(a[0]) > int(another[1]): # 아예 안 겹침
                answer -= 1

    return answer


if __name__ == '__main__':
    with open('luca/day_04/input.txt') as f:
        print(generate_number_of_fully_contain_pair_part1(text=f))
        f.seek(0)
        print(generate_number_of_fully_contain_pair_part2(text=f))


