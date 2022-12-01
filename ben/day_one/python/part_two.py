import heapq
from typing import TextIO


def get_most_nth_calories(reader: TextIO, n: int = 1):
    return sum(heapq.nlargest(n, list(sum(map(int, chunk.split('\n'))) for chunk in reader.read().split('\n\n') if chunk)))


if __name__ == '__main__':
    with open('test_input.txt') as f:
        assert get_most_nth_calories(f, 3) == 45000
    with open('input.txt') as f:
        print(get_most_nth_calories(f, 3))
