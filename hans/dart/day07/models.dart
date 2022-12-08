import 'dart:io';

abstract class Node {
  late String name;
  int get size;
  late Directory? parent;

  Node setParent(Directory parent);
  void printFileTree({int depth = 0}) {
    for (var _ = 0; _ < depth; _++) {
      stdout.write('  ');
    }
  }
}

class File extends Node {
  late int _size;

  @override
  int get size {
    return _size;
  }

  File(String name, int size) {
    this.name = name;
    this._size = size;
  }

  @override
  File setParent(Directory parent) {
    this.parent = parent;
    this.parent!.children.add(this);
    return this;
  }

  @override
  void printFileTree({int depth = 0}) {
    super.printFileTree(depth: depth);
    print('- ${this.name} (${this.size})');
  }
}

class Directory extends Node {
  List<Node> children = [];

  @override
  late String name;

  @override
  int get size {
    return this
        .children
        .fold(0, (totalSize, childNode) => totalSize + childNode.size);
  }

  Directory(this.name);

  @override
  Directory setParent(Directory parent) {
    this.parent = parent;
    this.parent!.children.add(this);
    return this;
  }

  @override
  void printFileTree({int depth = 0}) {
    super.printFileTree(depth: depth);
    print('- [${this.name}] (${this.size})');
    this
        .children
        .forEach((childNode) => childNode.printFileTree(depth: depth + 1));
  }
}
