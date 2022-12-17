class Valve {
  static final RegExp _regExp =
      RegExp(r'Valve (?<id>.{2}).+rate=(?<rate>\d).+valves (?<tunnels>.+)');

  final String id;
  final int rate;
  final List<String> tunnels;

  Valve(this.id, this.rate, this.tunnels);

  factory Valve.fromString(String raw) {
    final match = _regExp.firstMatch(raw);

    if (match == null) {
      throw Error();
    }

    final id = match.namedGroup('id')!;
    final rate = int.parse(match.namedGroup('rate')!);
    final tunnels = match.namedGroup('tunnels')!.split(', ');

    return Valve(id, rate, tunnels);
  }
}
