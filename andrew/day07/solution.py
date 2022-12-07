def part_one():
    pass


def part_two():
    pass


class Node:
    def __init__(self, name, size=None) -> None:
        self.name = name
        self.size = size if size else 0
        self.children: list[Node] = []

    def __str__(self):
        return self.name + " " + str(self.size)


class Tree:
    def __init__(self, name) -> None:
        self.root = Node(name)
        self.current_directory = self.root
        self.parent_directory = None

    def add_folder(self, name, size=None):
        self.current_directory.children.append(Node(name, size))
        # self.current_directory.size +

    def add_file(self, name, size):
        self.add_folder(name, size)
        self.current_directory.size += size

    def go_out(self):
        self.current_directory = self.parent_directory

    def __str__(self) -> str:
        return str(self.root)

    def go_into(self, name):
        self.parent_directory = self.current_directory
        # self.current_directory = self.parent_directory.children.


def create_directory_tree():
    file = open("./test.txt", "r")
    data = file.read()
    data_into_list = data.splitlines()

    for line in data_into_list:
        arr = line.split(" ")

        if "cd" in arr:
            current_directory = arr[2]

            if current_directory == "/":
                filesystem = Tree(current_directory)
            elif current_directory == "..":
                filesystem.go_out()

        elif "ls" in arr:
            ...
        elif "dir" in arr:
            directory_name = arr[1]
            filesystem.add_folder(directory_name)
        else:
            file_name = arr[1]
            file_size = int(arr[0])
            filesystem.add_file(file_name, file_size)

            # if current_directory not in directory.keys():
            #     directory[arr[2]] = {"size": 0, "children": {}}
    return filesystem


if __name__ == "__main__":
    xx = create_directory_tree()
    print(xx)

    for child in xx.root.children:
        print(child)
    # print(xx.root.children)
    # print("part one:", part_one())
    # print("part two:", part_two())
