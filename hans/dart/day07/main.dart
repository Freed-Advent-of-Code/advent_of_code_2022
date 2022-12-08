import 'dart:math';

import '../_lib/measure_time.dart';
import '../_lib/read_file.dart';
import 'models.dart';

void main() {
  final input = readFile('input');
  print(measureTime(() => solutionPuzzle1(input)));
  print(measureTime(() => solutionPuzzle2(input)));
}

int solutionPuzzle1(String input) {
  final lines = input.split('\n').sublist(1);
  final root = Directory('/');
  createFileTree(root, lines);
  return getTotalDirectorySize(root, (size) => size <= 100000);
}

int solutionPuzzle2(String input) {
  const totalDiskSize = 70000000;
  const neededSize = 30000000;

  final lines = input.split('\n').sublist(1);
  final root = Directory('/');
  createFileTree(root, lines);
  return getMinimumDirectorySize(root,
      (size) => root.size - size <= totalDiskSize - neededSize, totalDiskSize);
}

void createFileTree(Directory root, List<String> lines) {
  Directory currentDir = root;
  int index = 0;
  while (index < lines.length) {
    String line = lines[index];

    if (line.startsWith('\$ cd')) {
      final dirName = line.substring(5);
      if (dirName == '..') {
        final parent = currentDir.parent;
        currentDir = parent == null ? root : parent;
      } else {
        final dir = currentDir.children.firstWhere(
            (childNode) => childNode.name == dirName && childNode is Directory);
        currentDir = dir as Directory;
      }

      index += 1;
    }

    if (line.startsWith('\$ ls')) {
      int outputIndex = index + 1;
      while (
          outputIndex < lines.length && !lines[outputIndex].startsWith('\$')) {
        line = lines[outputIndex];
        if (line.startsWith('dir')) {
          final dirName = line.substring(4);
          Directory(dirName).setParent(currentDir);
        } else {
          final splited = line.split(' ');
          final size = int.parse(splited[0]);
          final fileName = splited[1];
          File(fileName, size).setParent(currentDir);
        }

        outputIndex += 1;
      }

      index = outputIndex;
      continue;
    }
  }
}

int getTotalDirectorySize(Directory directory, bool Function(int) filter) {
  int total = filter(directory.size) ? directory.size : 0;
  directory.children.forEach((childNode) {
    if (!(childNode is Directory)) {
      return;
    }

    total += getTotalDirectorySize(childNode, filter);
  });

  return total;
}

int getMinimumDirectorySize(
    Directory directory, bool Function(int) filter, int maxSize) {
  final childSizeList = directory.children
      .where(
          (childNode) => childNode is Directory && !childNode.children.isEmpty)
      .map((childNode) =>
          getMinimumDirectorySize(childNode as Directory, filter, maxSize));

  final minChildSize =
      childSizeList.length > 0 ? childSizeList.reduce(min) : maxSize;

  return filter(directory.size)
      ? min(minChildSize, directory.size)
      : minChildSize;
}
