def compare(x, y):
    if type(x) == type(y) == int:
        if x < y:
            return True
        elif x > y:
            return False
        else:
            return None
    elif type(x) == type(y) == list:
        for i in range(max(len(x), len(y))):
            if i >= len(x):
                return True
            elif i >= len(y):
                return False

            ret = compare(x[i], y[i])
            if ret is not None:
                return ret

    elif type(x) != type(y):
        if type(x) == list:
            return compare(x, [y])
        else:
            return compare([x], y)


def part_one():
    with open('input.txt', 'r') as f:
        data = f.read()
    right = []
    for i, x in enumerate(data.split('\n\n')):
        ret = compare(*[eval(y)for y in x.split("\n")])
        if ret:
            right.append(i+1)
    print(right)
    return sum(right)


print(part_one())

def part_two():
    with open('input.txt', 'r') as f:
        data = f.read()
    # heightmap = [list(x.strip()) for x in data.split('\n')]
    # return bfs(heightmap, 2)

print(part_two())
