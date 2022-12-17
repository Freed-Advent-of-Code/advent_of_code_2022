def part_one():
    with open('input.txt', 'r') as f:
        data = f.read()
    lines = [x for x in data.split("\n")]
    floor = set()
    for line in lines:
        prev = None
        for point in line.split('->'):
            x, y = list(map(int, point.split(',')))
            if prev:
                distance_x = abs(x - prev[0])
                distance_y = abs(y - prev[1])
                for i in range(max(distance_x, distance_y) + 1):
                    direction_x = 1 if x > prev[0] else -1 if x < prev[0] else 0
                    direction_y = 1 if y > prev[1] else -1 if y < prev[1] else 0

                    floor.add((prev[0] + i * direction_x, prev[1] + i * direction_y))
            prev = (x,y)

    # for infinity floor
    max_x = max(x[0] for x in floor) + 200
    max_y = max(x[1] for x in floor) + 2
    min_x = min(x[0] for x in floor) - 200
    for x in range(min_x, max_x):
        floor.add((x, max_y))

    start_point = (500,0)
    i = 0
    flag = False
    while i := i + 1:
        rock = start_point
        while True:
            if rock[1] + 1 >= max_y and not flag:
                flag = True
                print(i-1)
            if (rock[0], rock[1] + 1) not in floor:
                rock = (rock[0], rock[1] + 1)
            elif (rock[0] - 1,rock[1]+1) not in floor:
                rock = (rock[0] - 1, rock[1] + 1)
            elif (rock[0] + 1, rock[1] + 1) not in floor:
                rock = (rock[0] + 1, rock[1] + 1)
            else:
                break
        if rock == start_point:
            print(i)
            break
        floor.add(rock)
print(part_one())
