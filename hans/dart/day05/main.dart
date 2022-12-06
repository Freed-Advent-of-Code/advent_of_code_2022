import '../_lib/measure_time.dart';
import '../_lib/read_file.dart';
import '../_lib/stack.dart';

void main() {
  final input = readFile('input');
  print(measureTime(() => resolve(input)));
}

String resolve(String input) {
  final splited = input.split('\n\n');
  final stackList = parseToStackList(splited[0]);
  final procedures = splited[1].split('\n');
  procedures.forEach((procedure) => performProcedure(stackList, procedure));
  print(stackList);

  return stackList
      .where((stack) => !stack.isEmpty)
      .map((stack) => stack.top)
      .join('');
}

List<Stack<String>> parseToStackList(String raw) {
  final splited = raw.split('\n');
  final stackIds = splited.removeLast();
  final List<Stack<String>> stackList = createStackListFromIds(stackIds);

  RegExp crateRegExp = RegExp(r'\s{1,2}(\s)\s{1,2}|\[(?<crate>[A-Z])\]');
  for (final line in splited.reversed) {
    final matches = crateRegExp.allMatches(line).toList();
    for (var i = 0; i < matches.length; i++) {
      final match = matches[i];
      final crate = match.namedGroup('crate');
      if (crate == null) {
        continue;
      }

      stackList[i + 1].push(crate);
    }
  }

  return stackList;
}

List<Stack<String>> createStackListFromIds(String ids) {
  final List<Stack<String>> stackList = [Stack()];
  final matches = RegExp(r'(\d)').allMatches(ids);
  for (int i = 0; i < matches.length; i++) {
    stackList.add(Stack());
  }

  return stackList;
}

void performProcedure(List<Stack<String>> stackList, String rawProcedure) {
  final procedureRegExp = RegExp(r'(?<count>\d+)[^\d]+(?<from>\d+)[^\d]+(?<to>\d+)');
  final match = procedureRegExp.firstMatch(rawProcedure);
  if (match == null) {
    return;
  }

  final count = int.parse(match.namedGroup('count')!);
  final from = int.parse(match.namedGroup('from')!);
  final to = int.parse(match.namedGroup('to')!);

  for (int _ = 0; _ < count; _++) {
    final crateToMove = stackList[from].pop();
    if (crateToMove == null) {
      continue;
    }

    stackList[to].push(crateToMove);
  }
}
