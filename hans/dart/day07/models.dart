abstract class Node {
  late String name;
  int get size;
  late Node? parent;
}

class File implements Node {
  late int _size;

  @override
  late String name;

  @override
  int get size {
    return _size;
  }

  @override
  late Node? parent;

  File(Node parent, String name, int size) {
    this.parent = parent;
    this.name = name;
    this._size = size;
  }
}

class Directory implements Node {
  late List<Node> children;

  @override
  late String name;

  @override
  int get size {
    return this
        .children
        .fold(0, (totalSize, childNode) => totalSize + childNode.size);
  }

  @override
  late Node? parent;

  Directory(this.parent, this.name);
}
