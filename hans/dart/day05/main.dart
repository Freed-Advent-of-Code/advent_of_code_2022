import '../_lib/measure_time.dart';
import '../_lib/read_file.dart';
import '../_lib/stack.dart';
import 'models.dart';

void main() {
  final input = readFile('input');
  print(measureTime(() => resolvePuzzle1(input)));
  print(measureTime(() => resolvePuzzle2(input)));
}

String resolvePuzzle1(String input) {
  final splited = input.split('\n\n');
  final stackList = parseToStackList(splited[0]);
  final procedures = splited[1].split('\n').map(Procedure.fromString);
  procedures.forEach((procedure) => performProcedure(stackList, procedure));

  return stackList
      .where((stack) => !stack.isEmpty)
      .map((stack) => stack.top)
      .join('');
}

String resolvePuzzle2(String input) {
  final splited = input.split('\n\n');
  final stackList = parseToStackList(splited[0]);
  final procedures = splited[1].split('\n').map(Procedure.fromString);
  procedures.forEach(
      (procedure) => performProcedureWithBulkMove(stackList, procedure));

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

void performProcedure(List<Stack<String>> stackList, Procedure procedure) {
  for (int _ = 0; _ < procedure.count; _++) {
    final crateToMove = stackList[procedure.from].pop();
    if (crateToMove == null) {
      continue;
    }

    stackList[procedure.to].push(crateToMove);
  }
}

void performProcedureWithBulkMove(
    List<Stack<String>> stackList, Procedure procedure) {
  final tmpStack = Stack<String>();
  for (int _ = 0; _ < procedure.count; _++) {
    final crateToMove = stackList[procedure.from].pop();
    if (crateToMove == null) {
      continue;
    }

    tmpStack.push(crateToMove);
  }

  while (!tmpStack.isEmpty) {
    final crateToMove = tmpStack.pop();
    if (crateToMove == null) {
      continue;
    }

    stackList[procedure.to].push(crateToMove);
  }
}
