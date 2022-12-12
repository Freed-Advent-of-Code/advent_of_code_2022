import '../_lib/measure_time.dart';
import '../_lib/read_file.dart';
import 'models.dart';

void main() {
  final input = readFile('input');
  print(measureTime(() => solution1(input)));
  print(measureTime(() => solution2(input)));
}

int solution1(String input) {
  final rope = Rope.fromKnotsCount(2);
  final motions = input.split('\n').map(Motion.fromString);

  motions.forEach(rope.move);

  return rope.tailVisited.length;
}

int solution2(String input) {
  final rope = Rope.fromKnotsCount(10);
  final motions = input.split('\n').map(Motion.fromString);

  motions.forEach(rope.move);

  return rope.tailVisited.length;
}
