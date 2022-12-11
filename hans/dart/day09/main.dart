import '../_lib/measure_time.dart';
import '../_lib/read_file.dart';

void main() {
  final input = readFile('input');
  print(measureTime(() => solution1(input)));
}

int solution1(String input) {
  Set<String> visited = {};

  return visited.length;
}
