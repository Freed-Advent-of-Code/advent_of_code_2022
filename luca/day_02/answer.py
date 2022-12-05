from typing import TextIO


def generate_total_score(text: TextIO, part_number: int, point: dict[str: int], combinations: list[list]) -> int:
    total_score = 0
    for line in text.read().split('\n'):
        for i in range(3):
            if line in combinations[i]:
                total_score += point[line[-1]]
                if part_number == 1:
                    total_score += i*3
                elif part_number == 2:
                    total_score += i+1
            
    return total_score


if __name__ == '__main__':
    part_one_data = {
        'part_number': 1,
        'point': {'X': 1, 'Y': 2, 'Z': 3},
        'combinations': [['A Z', 'B X', 'C Y'], ['A X', 'B Y', 'C Z'], ['C X', 'B Z', 'A Y']]
    }
    part_two_data = {
        'part_number': 2,
        'point': {'X': 0, 'Y': 3, 'Z': 6},
        'combinations': [['B X', 'A Y', 'C Z'], ['C X', 'B Y', 'A Z'], ['A X', 'C Y', 'B Z']]
    }
    with open('luca/day_02/input.txt') as f:
        print(generate_total_score(text=f, **part_one_data))
        f.seek(0)
        print(generate_total_score(text=f, **part_two_data))