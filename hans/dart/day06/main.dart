import 'dart:collection';

import '../_lib/measure_time.dart';
import '../_lib/read_file.dart';

class Memo {
  int index = 0;
  bool inMarker = false;
  Memo(this.index, this.inMarker);
}

void main() {
  final input = readFile('input');
  print(measureTime(() => resolvePuzzle1(input)));
}

int resolvePuzzle1(String input) {
  final queue = Queue<int>();
  int count = 0;

  for (int i = 0; i < input.length; i++) {
    final codeUnit = input.codeUnitAt(i);

    while (queue.contains(codeUnit)) {
      queue.removeFirst();
    }

    queue.add(codeUnit);

    if (queue.length == 4) {
      count = i + 1;
      break;
    }
  }

  return count;
}
