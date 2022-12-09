
def part_one():
    data = [x.strip().split(" ") for x in open('input.txt', 'r').readlines()]
    h = (0, 0)
    t = (0, 0)
    # previous_t = (0, 0)
    visited = [t]
    for e in data:
        for i in range(int(e[1])):
            if e[0] == "R":
                h = (h[0], h[1] + 1)
            elif e[0] == "L":
                h = (h[0], h[1] - 1)
            elif e[0] == "U":
                h = (h[0] + 1, h[1])
            elif e[0] == "D":
                h = (h[0] - 1, h[1])
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
                continue
            visited.append(t)
    return len(set(visited))

print(part_one())


def part_two():
    for x in open('input.txt', 'r').readlines():
        x.strip()
    return

print(part_two())

