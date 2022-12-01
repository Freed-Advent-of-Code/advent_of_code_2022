class TotalCaloriesIsThatElfCarrying:
    def __init__(self, file_path: str) -> None:
        self.one_elf_total_calories = 0
        self.max_elf_total_calories = 0
        self.max_elf_number = 1
        self.file_path = file_path
    
    def read_file(self) -> list[str]:    
        with open(self.file_path, 'r') as f:
            return f.read().split('\n')

    def generate_total_calories_and_elf_number(self) -> str:
        for i in self.read_file():
            if i == '':
                self.max_elf_total_calories = max(self.one_elf_total_calories, self.max_elf_total_calories)
                self.one_elf_total_calories = 0
                self.max_elf_number += 1
            else:
                self.one_elf_total_calories += int(i)

        return f'Max Caloires is {self.max_elf_total_calories}, Elf Number is {self.max_elf_number}'


if __name__ == '__main__':
    answer = TotalCaloriesIsThatElfCarrying(file_path='luca/day_one/input.txt')
    print(answer.generate_total_calories_and_elf_number())