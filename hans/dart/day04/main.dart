import '../_lib/measure_time.dart';
import '../_lib/read_file.dart';
import 'utils.dart';

void main() {
  final input = readFile('input');
  print(measureTime(() => resolvePuzzle1(input)));
  print(measureTime(() => resolvePuzzle2(input)));
}

int resolvePuzzle1(String input) {
  int duplicatedCount = 0;
  final sectionPairs = input.split('\n').map(parseToSectionPair);
  sectionPairs.forEach((pair) {
    if (pair[0].isContainsSection(pair[1]) ||
        pair[1].isContainsSection(pair[0])) {
      duplicatedCount += 1;
    }
  });

  return duplicatedCount;
}

int resolvePuzzle2(String input) {
  int overlappedCount = 0;
  final sectionPairs = input.split('\n').map(parseToSectionPair);
  sectionPairs.forEach((pair) {
    if (pair[0].isOverlapSection(pair[1]) ||
        pair[1].isOverlapSection(pair[0])) {
      overlappedCount += 1;
    }
  });

  return overlappedCount;
}
