import itertools
def part_one():
    return sum(''.join([chr(x+97) for x in range(26)] + [chr(x+65) for x in range(26)]).find(next(x for x in set(e.strip()[:int(len(e.strip())/2)]) & set(e.strip()[int(len(e.strip())/2):])))+1 for e in open('input.txt', 'r').readlines())

print(part_one())


# def part_two():
#     with open('input.txt', 'r') as f:
#         data = f.read()
    
        
# print(part_two())