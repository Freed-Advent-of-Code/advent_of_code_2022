from collections import deque

def bfs(heightmap, part=1):
    Q = deque()
    for x in range(len(heightmap)):
        for y in range(len(heightmap[x])):
            if part == 1 and heightmap[x][y]=='S':
                Q.append(((x, y), 0))
                heightmap[x][y] = "a"
            if heightmap[x][y]=='E':
                heightmap[x][y] = "z"
            if part == 2 and heightmap[x][y] == 'a':
                Q.append(((x, y), 0))

    way = set()
    while Q:
        (x, y), d = Q.popleft()
        if (x, y) in way:
            continue
        way.add((x,y))
        if heightmap[x][y]=='z':
            return d
        for move_x, move_y in [(-1,0),(0,1),(1,0),(0,-1)]:
            xx = x+move_x
            yy = y+move_y
            if 0 <= xx < len(heightmap) \
                and 0 <= yy < len(heightmap[0]) \
                and ord(heightmap[xx][yy]) <= ord(heightmap[x][y]) + 1:
                    Q.append(((xx, yy), d + 1))

def part_one():
    with open('input.txt', 'r') as f:
        data = f.read()
    heightmap = [list(x.strip()) for x in data.split('\n')]
    return bfs(heightmap)

print(part_one())

def part_two():
    with open('input.txt', 'r') as f:
        data = f.read()
    heightmap = [list(x.strip()) for x in data.split('\n')]
    return bfs(heightmap, 2)

print(part_two())
