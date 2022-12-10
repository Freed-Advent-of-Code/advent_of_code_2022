def move(command, h):
    if command == "R":
        h = (h[0], h[1] + 1)
    elif command == "L":
        h = (h[0], h[1] - 1)
    elif command == "U":
        h = (h[0] + 1, h[1])
    elif command == "D":
        h = (h[0] - 1, h[1])
    return h

def follow(h, t):
    if abs(h[0] - t[0]) > 1:
        if h[0] > t[0]:
            t = (t[0] + 1, t[1])
        elif h[0] < t[0]:
            t = (t[0] - 1, t[1])

        if abs(h[1] - t[1]) > 0:
            if h[1] > t[1]:
                t = (t[0], t[1] + 1)
            elif h[1] < t[1]:
                t = (t[0], t[1] - 1)
    elif abs(h[1] - t[1]) > 1:
        if h[1] > t[1]:
            t = (t[0], t[1] + 1)
        elif h[1] < t[1]:
            t = (t[0], t[1] - 1)

        if abs(h[0] - t[0]) > 0:
            if h[0] > t[0]:
                t = (t[0] + 1, t[1])
            elif h[0] < t[0]:
                t = (t[0] - 1, t[1])
    else:
        return t, False
    return t, True


def part_one():
    data = [x.strip().split(" ") for x in open('input.txt', 'r').readlines()]
    h = (0, 0)
    t = (0, 0)
    # previous_t = (0, 0)
    visited = [t]
    for e in data:
        for i in range(int(e[1])):
            h = move(e[0], h)
            t, is_moved = follow(h, t)
            if is_moved:
                visited.append(t)
    return len(set(visited))

print(part_one())

def part_two():
    data = [x.strip().split(" ") for x in open('input.txt', 'r').readlines()]
    knots = [(0, 0)] * 10
    visited = [knots[-1]]
    for e in data:
        for i in range(int(e[1])):
            knots[0] = move(e[0], knots[0])
            for j in range(9):
                knots[j+1], is_moved = follow(knots[j], knots[j+1])
                if is_moved:
                    visited.append(knots[-1])
                    visited = list(set(visited))
    return len(set(visited))

print(part_two())

