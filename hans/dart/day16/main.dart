import '../_lib/measure_time.dart';
import '../_lib/read_file.dart';
import 'models.dart';
import 'utils.dart';

void main() {
  final input = readFile('test');
  print(measureTime(() => solution1(input)));
}

int solution1(String input) {
  final valves = input.split('\n').map(Valve.fromString);
  final valveMap = {for (var valve in valves) valve.id: valve};
  final minDistanceGrid = floydWarshall(valves);

  final targetValves =
      valves.where((valve) => valve.rate > 0).map((valve) => valve.id).toSet();
  final resultRef = Ref(0);

  dfs(DfsDto(
      valveMap, minDistanceGrid, targetValves, valveMap['AA']!, resultRef));

  return resultRef.current;
}

void dfs(DfsDto dto) {}
