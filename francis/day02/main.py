def part_one():
    with open('input.txt', 'r') as f:
        data = f.read()
    return sum(6 if ord(x) - 64 +1 == ord(y) - 87 or ord(x) - 64 -2 == ord(y) - 87 else 3 if ord(x) - 64 == ord(y) - 87 else 0 for x, y in [line.split(" ") for line in data.split("\n")]) + sum(ord(line.split(" ")[1])-87 for line in data.split("\n"))

print(part_one())

def part_two():
    with open('input.txt', 'r') as f:
        data = f.read()
    return sum(6 if ord(y) - 87 == 3 else 3 if ord(y) - 87 == 2 else 0 for x, y in [line.split(" ") for line in data.split("\n")]) + sum([1,2,3][(ord(x) - (64- (ord(y) - 87))) % 3] for x, y in [line.split(" ") for line in data.split("\n")])
    
        
print(part_two())