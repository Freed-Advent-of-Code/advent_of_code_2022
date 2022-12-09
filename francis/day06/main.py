def find_marker(data, n):
    buffer = []
    for s in data:
        buffer.append(s)
        if len(set(buffer[-n:])) == n:
            return len(buffer)

def part_one():
    with open('input.txt', 'r') as f:
        return find_marker(f.read(), 4)

print(part_one())


def part_two():
    with open('input.txt', 'r') as f:
        return find_marker(f.read(), 14)
  
print(part_two())