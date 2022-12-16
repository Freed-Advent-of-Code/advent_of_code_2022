def compare(x, y):
    if type(x) == type(y) == int:
        if x == y:
            return None
        return x < y
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
    right = []
    right.append([[2]])
    right.append([[6]])
    
    for e in data.split('\n\n'):
        for f in e.split('\n'):
            right.append(eval(f))

    def merge_sort(arr):
        if len(arr) <= 1:
            return arr
        mid = len(arr) // 2
        left = merge_sort(arr[:mid])
        right = merge_sort(arr[mid:])
        return merge(left, right)

    def merge(left, right):
        ret = []
        while left and right:
            if compare(left[0], right[0]):
                ret.append(left.pop(0))
            else:
                ret.append(right.pop(0))
        return ret + left + right

    right = merge_sort(right)
    return (right.index([[6]])+1) * (right.index([[2]])+1)
        

print(part_two())
