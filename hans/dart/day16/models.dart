class Ref<T> {
  T current;

  Ref(this.current);
}

class Valve {
  static final RegExp _regExp =
      RegExp(r'Valve (?<id>.{2}).+rate=(?<rate>\d+).+valves? (?<tunnels>.+)');

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

  @override
  String toString() {
    return '\n- id: ${id}\n- rate: ${rate}\n- tunnels: ${tunnels}\n';
  }
}

class DfsDto {
  final Map<String, Valve> valveMap;
  final Map<String, Map<String, int>> minDistanceGrid;
  final Set<String> targetValves;
  final Ref<int> resultRef;
  final Valve currentValve;
  final int currentMin;
  final int maxMin;
  final int currentRelease;
  final int currentReleaseIncrease;

  DfsDto(this.valveMap, this.minDistanceGrid, this.targetValves,
      this.currentValve, this.resultRef,
      {this.currentMin: 1,
      this.maxMin: 30,
      this.currentRelease: 0,
      this.currentReleaseIncrease: 0});

  DfsDto update(
      {Set<String>? targetValves,
      Valve? currentValve: null,
      int? currentMin: null,
      int? currentRelease: null,
      int? currentReleaseIncrease: null}) {
    final newTargetValves = targetValves ?? this.targetValves;
    final newCurrentValve = currentValve ?? this.currentValve;
    final newCurrentMin = currentMin ?? this.currentMin;
    final newCurrentRelease = currentRelease ?? this.currentRelease;
    final newCurrentReleaseIncrease =
        currentReleaseIncrease ?? this.currentReleaseIncrease;

    return DfsDto(this.valveMap, this.minDistanceGrid, newTargetValves,
        newCurrentValve, this.resultRef,
        currentMin: newCurrentMin,
        maxMin: this.maxMin,
        currentRelease: newCurrentRelease,
        currentReleaseIncrease: newCurrentReleaseIncrease);
  }
}
