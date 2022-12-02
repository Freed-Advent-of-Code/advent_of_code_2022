import '../_lib/read_file.dart';

const scoreMap1 = {
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

const scoreMap2 = {
  'A': {
    'X': 3 + 0,
    'Y': 1 + 3,
    'Z': 2 + 6,
  },
  'B': {
    'X': 1 + 0,
    'Y': 2 + 3,
    'Z': 3 + 6,
  },
  'C': {
    'X': 2 + 0,
    'Y': 3 + 3,
    'Z': 1 + 6,
  }
};

void main() {
  final input = readFile('input');
  print(resolvePuzzle1(input));
  print(resolvePuzzle2(input));
}

int resolvePuzzle1(String input) {
  return resolve(input, scoreMap1);
}

int resolvePuzzle2(String input) {
    return resolve(input, scoreMap2);
}

int resolve(String input, Map<String, Map<String, int>> scoreMap) {
  return input
      .split('\n')
      .map((line) => line.split(' '))
      .map((choose) => scoreMap[choose[0]]![choose[1]]!)
      .reduce((a, b) => a + b);
}
