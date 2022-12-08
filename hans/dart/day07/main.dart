import 'dart:collection';
import 'dart:math';

import '../_lib/measure_time.dart';
import '../_lib/read_file.dart';
import 'models.dart';

void main() {
  final input = readFile('input');
  print(measureTime(() => solutionPuzzle1(input)));
}

int solutionPuzzle1(String input) {
  final lines = input.split('\n').sublist(1);
  final root = Directory('/');
  createFileTree(root, lines);
  return getTotalDirectorySize(root, (size) => size <= 100000);
}

void createFileTree(Directory root, List<String> lines) {}

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
