from typing import Self

class FileSystem:    
    def root(self):
        if self.parent == None:
            return self
        else:
            return self.parent.root()

    def size(self):
        raise NotImplementedError()


class Directory(FileSystem):
    def __init__(self, name: str, items: list[FileSystem], parent: Self = None):
        self.name = name
        self.items = items
        self.parent = parent
        self._size = 0

    def process(self):
        self.size()

    def size(self):
        self._size = sum([x.size() for x in self.items])
        directories.append(self._size)
        # global directories
        return self._size


class File(FileSystem):
    def __init__(self, name: str, size: int, parent: Directory = None):
        self.name = name
        self._size = size
        self.parent = parent

    def size(self):
        return self._size


directories = []

def build_directory_tree():
    pwd: Directory = Directory(name="/", items=[])
    for e in open('input.txt', 'r').readlines():
        e = e.strip()
        if "$ cd /" == e:
            pwd = pwd.root()
        elif "$ cd .." == e:
            pwd = pwd.parent
        elif "$ cd " in e:
            pwd.items.append(Directory(name=e[5:], items=[], parent=pwd))
            pwd = pwd.items[-1]
        elif "$ ls" == e:
            pass
        elif "dir " in e:
            pwd.items.append(Directory(name=e[4:], items=[], parent=pwd))
        else:
            size, name = e.split(" ")
            pwd.items.append(File(name=name, size=int(size), parent=pwd))
    root = pwd.root()
    root.process()
    return root


def part_one():
    root = build_directory_tree()
    return sum(x for x in directories if x <= 100000)

print(part_one())


def part_two():
    if directories:
        directories.clear()
    root = build_directory_tree()
    total = max(x for x in directories)
    return min(x for x in directories if x > total - 40000000)

print(part_two())