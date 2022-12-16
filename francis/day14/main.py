def part_one():
    with open('input.txt', 'r') as f:
        data = f.read()
    lines = [x for x in data.split("\n")]
    floor = set()
    for line in lines:
        prev = None
        for point in line.split('->'):
            x, y = list(map(int, point.split(',')))
            if prev is not None:
                dx = x-prev[0]
                dy = y-prev[1]
                for i in range(max(abs(dx), abs(dy)) +1):
                    xx = prev[0] + i * (1 if dx > 0 else (-1 if dx < 0 else 0))
                    yy = prev[1] + i * (1 if dy > 0 else (-1 if dy < 0 else 0))
                    floor.add((xx,yy))
            prev = (x,y)

    max_y = max(r[1] for r in floor) + 2
    #print(max_y)
    min_x = min(r[0] for r in floor) - 200
    max_x = max(r[0] for r in floor) + 200
    for x in range(min_x, max_x):
        floor.add((x, max_y))

    part_1 = False
    for i in range(200000):
        rock = (500,0)
        while True:
            if rock[1] + 1 >= max_y and not part_1:
                part_1 = True
                print(i)
                continue
            if (rock[0], rock[1] + 1) not in floor:
                rock = (rock[0], rock[1] + 1)
            elif (rock[0] - 1,rock[1]+1) not in floor:
                rock = (rock[0] - 1, rock[1] + 1)
            elif (rock[0] + 1, rock[1] + 1) not in floor:
                rock = (rock[0] + 1, rock[1] + 1)
            else:
                break
        if rock == (500,0):
            print(i + 1)
            break
        floor.add(rock)
print(part_one())