import '../_lib/read_file.dart';

void main() {
  final input = readFile('input');
  print(resolve(input));
}

int resolve(String input) {
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
