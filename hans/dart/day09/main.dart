import '../_lib/measure_time.dart';
import '../_lib/read_file.dart';
import 'models.dart';

void main() {
  final input = readFile('input');
  print(measureTime(() => solution1(input)));
}

int solution1(String input) {
  final head = Position(0, 0);
  final tail = Position(0, 0);
  Set<String> tailVisited = {tail.toString()};
  final motions = input.split('\n').map(Motion.fromString);

  motions.forEach((motion) {});

  return tailVisited.length;
}
