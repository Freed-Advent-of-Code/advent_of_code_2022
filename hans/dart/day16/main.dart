import 'dart:math';

import '../_lib/measure_time.dart';
import '../_lib/read_file.dart';
import 'models.dart';
import 'utils.dart';

void main() {
  final input = readFile('input');
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

void dfs(DfsDto dto) {
  if (dto.currentMin > dto.maxMin) {
    dto.resultRef.current = max(dto.resultRef.current, dto.currentRelease);
    return;
  }

  if (dto.targetValves.length == 0) {
    final calculatedRelease = dto.currentRelease +
        dto.currentReleaseIncrease * (dto.maxMin - dto.currentMin + 1);
    dto.resultRef.current = max(dto.resultRef.current, calculatedRelease);
    return;
  }

  // release valve
  if (dto.currentValve.rate != 0) {
    final newTargetValves = Set<String>.from(dto.targetValves);
    newTargetValves.remove(dto.currentValve.id);
    dto = dto.update(
        currentMin: dto.currentMin + 1,
        currentRelease: dto.currentRelease + dto.currentReleaseIncrease,
        currentReleaseIncrease:
            dto.currentReleaseIncrease + dto.currentValve.rate,
        targetValves: newTargetValves);

    if (dto.currentMin > dto.maxMin || dto.targetValves.length == 0) {
      dfs(dto);
      return;
    }
  }

  for (String targetValveId in dto.targetValves) {
    int distance = dto.minDistanceGrid[dto.currentValve.id]![targetValveId]!;

    // move
    var newDto = dto.update();
    while (distance > 0) {
      newDto = newDto.update(
          currentMin: newDto.currentMin + 1,
          currentRelease:
              newDto.currentRelease + newDto.currentReleaseIncrease);
      if (newDto.currentMin > newDto.maxMin) {
        break;
      }

      distance -= 1;
    }

    dfs(newDto.update(currentValve: newDto.valveMap[targetValveId]));
  }
}
