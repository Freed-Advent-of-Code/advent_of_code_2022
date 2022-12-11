import '../_lib/measure_time.dart';
import '../_lib/read_file.dart';
import 'models.dart';

void main() {
  final input = readFile('input');
  print(measureTime(() => solution1(input)));
}

int solution1(String input) {
  var head = Position(0, 0);
  var tail = Position(0, 0);
  Set<String> tailVisited = {tail.toString()};
  final motions = input.split('\n').map(Motion.fromString);

  motions.forEach((motion) {
    for (int _ = 0; _ < motion.moveAmount; _++) {
      head = head.move(motion.unitVector);
      if (!head.isTouchingWith(tail)) {
        if (head.manhattanDistanceWith(tail) > 2) {
          tail = head.move(motion.unitVector.reverse);
        } else {
          tail = tail.move(motion.unitVector);
        }

        tailVisited.add(tail.toString());
      }
    }

    print('head: ${head}, tail: ${tail}');
  });

  return tailVisited.length;
}
