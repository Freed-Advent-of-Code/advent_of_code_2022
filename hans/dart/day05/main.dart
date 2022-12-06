import '../_lib/measure_time.dart';
import '../_lib/read_file.dart';
import '../_lib/stack.dart';

void main() {
  final input = readFile('input');
  measureTime(() => resolve(input));
}

String resolve(String input) {
  final splited = input.split('\n\n');
  final stackList = parseToStackList(splited[0]);
  final procedures = splited[1].split('\n');

  return "";
}

List<List<String>> parseToStackList(String raw) {
  final splited = raw.split('\n');
  final stackIds = splited.removeLast();
  final List<List<String>> stackList = createStackListFromIds(stackIds);

  RegExp crateRegExp = RegExp(r' ( ) |\[([A-Z])\]');
  for (final line in splited.reversed) {
    final matches = crateRegExp.allMatches(line).toList();
    for (var i = 0; i < matches.length; i++) {
      final match = matches[i];
      final crate = match.group(2);
      if (crate == null) {
        continue;
      }

      stackList[i + 1].add(crate);
    }
  }

  return stackList;
}

List<List<String>> createStackListFromIds(String ids) {
  final List<List<String>> stackList = [[]];
  final matches = RegExp(r'(\d)').allMatches(ids);
  for (int i = 0; i < matches.length; i++) {
    stackList.add([]);
  }

  return stackList;
}
