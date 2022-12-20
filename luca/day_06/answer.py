from typing import TextIO


def day_06(text: TextIO, character_length: int) -> int | None:
    text_line = text.read()
    for i in range(len(text_line)):
        if len(set(text_line[i:i+character_length])) == character_length:
            return i+character_length


if __name__ == '__main__':
    with open('input.txt') as f:
        print(day_06(f, 4))
        f.seek(0)
        print(day_06(f, 14))
