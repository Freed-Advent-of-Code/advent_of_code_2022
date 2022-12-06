class Procedure {
  int count = 0;
  int from = 0;
  int to = 0;

  Procedure(this.count, this.from, this.to);

  Procedure.fromString(String raw) {
    final procedureRegExp =
        RegExp(r'(?<count>\d+)[^\d]+(?<from>\d+)[^\d]+(?<to>\d+)');
    final match = procedureRegExp.firstMatch(raw);
    if (match == null) {
      return;
    }

    final count = int.parse(match.namedGroup('count')!);
    final from = int.parse(match.namedGroup('from')!);
    final to = int.parse(match.namedGroup('to')!);

    this.count = count;
    this.from = from;
    this.to = to;
  }
}
