import '../_lib/read_file.dart';

void main() {
  final input = readFile('input');
  print(resolvePuzzle1(input));
  print(resolvePuzzle2(input));
}

int resolvePuzzle1(String input) {
  return input
      .split("\n")
      .map((rucksack) => Set<String>.from(
              rucksack.substring(0, rucksack.length ~/ 2).split(''))
          .intersection(Set<String>.from(
              rucksack.substring(rucksack.length ~/ 2).split('')))
          .map((char) => char.codeUnitAt(0) <= 'Z'.codeUnitAt(0)
              ? char.codeUnitAt(0) - 'A'.codeUnitAt(0) + 27
              : char.codeUnitAt(0) - 'a'.codeUnitAt(0) + 1))
      .fold<int>(0, (total, curr) => total + curr.toList()[0]);
}

int resolvePuzzle2(String input) {
  return List<Iterable<String>>.generate(input.split('\n').length ~/ 3,
          (i) => input.split('\n').getRange(i * 3, i * 3 + 3))
      .map((group) => group.fold(
          Set<int>.from(List<int>.generate(52, (i) => i + 1)),
          (set, rucksack) => set.intersection(Set<int>.from(rucksack
              .split('')
              .map((char) => char.codeUnitAt(0) <= 'Z'.codeUnitAt(0)
                  ? char.codeUnitAt(0) - 'A'.codeUnitAt(0) + 27
                  : char.codeUnitAt(0) - 'a'.codeUnitAt(0) + 1)))))
      .fold(0, (total, curr) => total + curr.toList()[0]);
}
