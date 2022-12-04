import '../_lib/measure_time.dart';
import '../_lib/read_file.dart';

void main() {
  final input = readFile('input');
  print(measureTime(() => resolve(input)));
}

int resolve(String input) {
  return input.split('\n').fold( 0, (count, pair) => count + ((List<String> pair) => (int.parse(pair[0].split('-')[0]) <= int.parse(pair[1].split('-')[0]) && int.parse(pair[0].split('-')[1]) >= int.parse(pair[1].split('-')[1])) || (int.parse(pair[1].split('-')[0]) <= int.parse(pair[0].split('-')[0]) && int.parse(pair[1].split('-')[1]) >= int.parse(pair[0].split('-')[1])) ? 1 : 0)(pair.split(',')));
}
