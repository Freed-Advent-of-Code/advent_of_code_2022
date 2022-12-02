import '../_lib/read_file.dart';

const scoreMap = {
  'A': {
    'X': 1 + 3,
    'Y': 2 + 6,
    'Z': 3 + 0,
  },
  'B': {
    'X': 1 + 0,
    'Y': 2 + 3,
    'Z': 3 + 6,
  },
  'C': {
    'X': 1 + 6,
    'Y': 2 + 0,
    'Z': 3 + 3,
  }
};

final Map<String, int>? b = scoreMap['A'];
final c = b?['X'];

void main() {
  final input = readFile('input');
  print(resolvePuzzle1(input));
}

int resolvePuzzle1(String input) {
  return input
      .split('\n')
      .map((line) => line.split(' '))
      .map((choose) => scoreMap[choose[0]]![choose[1]]!)
      .reduce((a, b) => a + b);
}
