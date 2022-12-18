import re


def check_within_range(sensor, distance, beacon):
    return abs(sensor[0] - beacon[0]) + abs(sensor[1] - beacon[1]) <= distance

def part_one():
    with open('input.txt', 'r') as f:
        data = f.read()
    sensors = []
    beacons = []
    for line in data.split("\n"):
        e = list(map(int, re.sub(r"Sensor at x=(\d+), y=(\d+): closest beacon is at x=(-?\d+), y=(-?\d+)", r"\1,\2,\3,\4", line.strip()).split(",")))
        sensors.append(tuple(e[:2]))
        beacons.append(tuple(e[2:]))
        # print(e)
    distance = []
    for sensor, beacon in zip(sensors, beacons):
        distance.append(abs(sensor[0] - beacon[0]) + abs(sensor[1] - beacon[1]))
    
    max_x = max(sensors, key=lambda x: x[0])[0]
    min_x = min(sensors, key=lambda x: x[0])[0]
    y = 2000000
    count = -1
    for x in range(min_x - max(distance), max_x + max(distance)):
        for s, d in zip(sensors, distance):
            if check_within_range(s, d, (x, y)):
                count += 1
                break

    print(count)

# print(part_one())


def get_sensor_edge_position(sensor, distance):
    position = set()
    for i in range(distance):
        for j in range(4):
            if j == 0:
                p = (sensor[0] + (distance-i) + 1, sensor[1] + i)
            elif j == 1:
                p = (sensor[0] - (distance-i) - 1, sensor[1] + i)
            elif j == 2:
                p = (sensor[0] + i, sensor[1] + (distance-i) + 1)
            elif j == 3:
                p = (sensor[0] + i, sensor[1] - (distance-i) - 1)
            if 0 <= p[0] <= 4000000 and 0 <= p[1] <= 4000000:
                position.add(p)
            
    return position    

def part_two():
    with open('input.txt', 'r') as f:
        data = f.read()
    sensors = []
    beacons = []
    for line in data.split("\n"):
        e = list(map(int, re.sub(r"Sensor at x=(\d+), y=(\d+): closest beacon is at x=(-?\d+), y=(-?\d+)", r"\1,\2,\3,\4", line.strip()).split(",")))
        sensors.append(tuple(e[:2]))
        beacons.append(tuple(e[2:]))
        # print(e)
    distance = []
    for sensor, beacon in zip(sensors, beacons):
        distance.append(abs(sensor[0] - beacon[0]) + abs(sensor[1] - beacon[1]))
    
    positions = set()
    for i, (s, d) in enumerate(zip(sensors, distance)):
        positions |= get_sensor_edge_position(s, d)
        print(i)
    
    for i, p in enumerate(positions):
        for s, d in zip(sensors, distance):
            if check_within_range(s, d, p):
                break
        else:
            return p[0] * 4000000 + p[1]
        if i % 100000 == 0:
            print(i)

print(part_two())
