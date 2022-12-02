
def calculate(reader):
    return sum([(ord(line.split()[1]) - 88) * 3 + (ord(line.split()[0]) + ord(line.split()[1]) - 151) % 3 + 1 for line in reader.readlines()])


if __name__ == '__main__':
    with open('test_input.txt') as f:
        assert calculate(f) == 12

    with open('input.txt') as f:
        print(calculate(f))
