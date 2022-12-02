from typing import TextIO


def generate_total_score(text: TextIO) -> int:
    total_score = 0
    muk_jji_ppa_point = {
        'X': 1,
        'Y': 2,
        'Z': 3 
    }
    win_draw_lose_combinations = [['A Z', 'B X', 'C Y'], ['A X', 'B Y', 'C Z'], ['C X', 'B Z', 'A Y']]
    for line in text.read().split('\n'):
        for i in range(3):
            if line in win_draw_lose_combinations[i]:
                total_score += i*3 + muk_jji_ppa_point[line[-1]]
            
    return total_score


if __name__ == '__main__':
    with open('luca/day_two/input.txt') as f:
        print(generate_total_score(f))