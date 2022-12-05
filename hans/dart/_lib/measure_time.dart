dynamic measureTime(dynamic Function() func) {
  print(
      '\nStart to measure execution time of function "${getFunctionName(func)}"');
  final startTime = DateTime.now();
  final result = func();
  final endTime = DateTime.now();
  print(
      'It takes ${endTime.millisecondsSinceEpoch - startTime.millisecondsSinceEpoch}ms');
  return result;
}

String getFunctionName(Function func) {
  final regExp = RegExp('\'(.+)\'');
  final match = regExp.firstMatch(func.toString());
  if (match == null) {
    return '';
  }

  return match.group(1)!;
}
