import '../_lib/measure_time.dart';
import '../_lib/read_file.dart';
import 'section.dart';

void main() {
  final input = readFile('input');
  print(measureTime(() => resolvePuzzle1(input)));
}

int resolvePuzzle1(String input) {
  int duplicatedCount = 0;
  final sectionPairs = input.split('\n').map(parseToSectionPair);
  sectionPairs.forEach((pair) {
    if (pair[0].containsSection(pair[1]) || pair[1].containsSection(pair[0])) {
      duplicatedCount += 1;
    }
  });

  return duplicatedCount;
}

List<Section> parseToSectionPair(String raw) {
  return raw.split(',').map(Section.fromString).toList();
}
