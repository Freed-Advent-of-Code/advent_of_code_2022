from typing import TextIO


def get_most_calories(reader: TextIO):
    return max(sum(map(int, chunk.split('\n'))) for chunk in reader.read().split('\n\n'))


if __name__ == '__main__':
    with open('input.txt') as f:
        print(get_most_calories(f))
