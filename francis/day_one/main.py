def part_one():
    with open('input.txt', 'r') as f:
        data = f.read()
    return max(sum(int(x) for x in e.split("\n")) for e in data.split("\n\n"))

print(part_one())

def part_two():
    with open('input.txt', 'r') as f:
        data = f.read()
    return sum(sorted(sum(int(x) for x in e.split("\n")) for e in data.split("\n\n"))[::-1][:3])
print(part_two())