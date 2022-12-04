def part_one():
    return len([e for e in open('input.txt', 'r').readlines() if abs(max(eval(x)-1 for x in e.split(","))) == len(set(range(int(e.split(",")[0].split("-")[0]), int(e.split(",")[0].split("-")[1])+1)) & set(range(int(e.split(",")[1].split("-")[0]), int(e.split(",")[1].split("-")[1])+1)))])
    
print(part_one())


def part_two():
    return len([e for e in open('input.txt', 'r').readlines() if 0 < len(set(range(int(e.split(",")[0].split("-")[0]), int(e.split(",")[0].split("-")[1])+1)) & set(range(int(e.split(",")[1].split("-")[0]), int(e.split(",")[1].split("-")[1])+1)))])
  
print(part_two())