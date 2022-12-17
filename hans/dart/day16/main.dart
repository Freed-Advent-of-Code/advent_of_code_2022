import 'dart:math';

import '../_lib/measure_time.dart';
import '../_lib/read_file.dart';
import 'model.dart';

void main() {
  final input = readFile('test');
  print(measureTime(() => solution1(input)));
}

int solution1(String input) {
  final valves = input.split('\n').map(Valve.fromString);
  final valveMap = {for (var valve in valves) valve.id: valve};
  final resultRef = Ref(0);

  Set<String> released =
      valves.where((valve) => valve.rate > 0).map((valve) => valve.id).toSet();
  print(released);
  dfs(valveMap, valveMap['AA']!, resultRef, 30, released);

  return resultRef.current;
}

void dfs(
  Map<String, Valve> valveMap,
  Valve currentValve,
  Ref<int> resultRef,
  int maxMin,
  Set<String> released, {
  int currentMin: 1,
  int currentRelease: 0,
  int currentReleaseIncrease: 0,
}) {
  if (currentMin > maxMin) {
    resultRef.current = max(resultRef.current, currentRelease);
    return;
  }

  if (valveMap.length == released.length) {
    final result =
        currentRelease + currentReleaseIncrease * (maxMin - currentMin + 1);
    resultRef.current = max(resultRef.current, result);
    return;
  }

  currentRelease += currentReleaseIncrease;
  currentMin += 1;
  final newReleased = Set<String>.from(released)..add(currentValve.id);

  // open valve
  if (!released.contains(currentValve.id)) {
    dfs(
      valveMap,
      currentValve,
      resultRef,
      maxMin,
      newReleased,
      currentMin: currentMin,
      currentRelease: currentRelease,
      currentReleaseIncrease: currentReleaseIncrease += currentValve.rate,
    );
  }

  // move to tunnel
  for (var valveId in currentValve.tunnels) {
    dfs(
      valveMap,
      valveMap[valveId]!,
      resultRef,
      maxMin,
      newReleased,
      currentMin: currentMin,
      currentRelease: currentRelease,
      currentReleaseIncrease: currentReleaseIncrease,
    );
  }
}
