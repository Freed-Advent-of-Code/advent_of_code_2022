def is_visible(lst, item):
    return bool(item == max(lst) and lst.count(max(lst)) == 1)


def part_one():
    visible = []
    data = [list(map(int, x.strip())) for x in open('input.txt', 'r').readlines()]
    for i in range(len(data)):
        for j in range(len(data[i])):
            if is_visible(data[i][:j+1], data[i][j]) or is_visible(data[i][j:], data[i][j]):
                visible.append(data[i][j])
                continue
            data_vertical = [x[j] for x in data]
            if is_visible(data_vertical[:i+1], data[i][j]) or is_visible(data_vertical[i:], data[i][j]):
                visible.append(data[i][j])
                continue

    return len(visible)


print(part_one())


def part_two():
    return
print(part_two())

