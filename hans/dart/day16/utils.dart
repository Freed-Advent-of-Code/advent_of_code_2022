import 'models.dart';

Map<String, Map<String, int>> floydWarshall(Iterable<Valve> valves) {
  final infinite = 4294967296;
  final valveIds = valves.map((valve) => valve.id);
  final grid = {
    for (var valveId in valveIds)
      valveId: {for (var valveId in valveIds) valveId: infinite}
  };

  valves.forEach((valve) {
    final currentValveId = valve.id;
    valve.tunnels.forEach((nextValveId) {
      if (currentValveId == nextValveId) {
        grid[currentValveId]![nextValveId] = 0;
        grid[nextValveId]![currentValveId] = 0;
        return;
      }

      grid[currentValveId]![nextValveId] = 1;
      grid[nextValveId]![currentValveId] = 1;
    });
  });

  for (var k in valveIds) {
    for (var i in valveIds) {
      for (var j in valveIds) {
        if (grid[i]![j]! > grid[i]![k]! + grid[k]![j]!) {
          grid[i]![j] = grid[i]![k]! + grid[k]![j]!;
        }
      }
    }
  }

  return grid;
}
