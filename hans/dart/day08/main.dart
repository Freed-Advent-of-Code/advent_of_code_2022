import '../_lib/measure_time.dart';
import '../_lib/read_file.dart';

void main() {
  final input = readFile('input');
  print(measureTime(() => solutionPuzzle1(input)));
}

int solutionPuzzle1(String input) {
  final grid = parseToGrid(input);
  final gridRows = grid.length;
  final gridCols = grid[0].length;

  Set<String> visibleTrees = {};

  for (int row = 0; row < gridRows; row++) {
    int maxHeight = -1;
    for (int col = 0; col < gridCols; col++) {
      int treeHeight = grid[row][col];
      if (treeHeight > maxHeight) {
        visibleTrees.add('${row},${col}');
        maxHeight = treeHeight;
      }
    }

    maxHeight = -1;
    for (int col = gridCols - 1; col >= 0; col--) {
      int treeHeight = grid[row][col];
      if (treeHeight > maxHeight) {
        visibleTrees.add('${row},${col}');
        maxHeight = treeHeight;
      }
    }
  }

  for (int col = 0; col < gridCols; col++) {
    int maxHeight = -1;
    for (int row = 0; row < gridRows; row++) {
      int treeHeight = grid[row][col];
      if (treeHeight > maxHeight) {
        visibleTrees.add('${row},${col}');
        maxHeight = treeHeight;
      }
    }

    maxHeight = -1;
    for (int row = gridRows - 1; row >= 0; row--) {
      int treeHeight = grid[row][col];
      if (treeHeight > maxHeight) {
        visibleTrees.add('${row},${col}');
        maxHeight = treeHeight;
      }
    }
  }

  return visibleTrees.length;
}

List<List<int>> parseToGrid(String input) {
  return input
      .split('\n')
      .map((line) => line.split('').map(int.parse).toList())
      .toList();
}
