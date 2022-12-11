class Position {
  int x = 0;
  int y = 0;

  Position(this.x, this.y);

  bool isTouchingWith(Position otherPos) {
    int distanceX = (this.x - otherPos.x).abs();
    int distanceY = (this.y - otherPos.y).abs();

    return distanceX <= 1 && distanceY <= 1;
  }

  @override
  String toString() {
    return '${this.x},${this.y}';
  }
}

class Motion {
  final Direction direction;
  final int moveAmount;

  const Motion(this.direction, this.moveAmount);

  factory Motion.fromString(String raw) {
    final splited = raw.split(' ');
    final direction = Direction.getByCode(splited[0]);
    final moveAmount = int.parse(splited[1]);

    return Motion(direction, moveAmount);
  }
}

enum Direction {
  Right('R'),
  Left('L'),
  Up('U'),
  Down('D'),
  None('_');

  const Direction(this.code);
  final String code;

  factory Direction.getByCode(String code) {
    return Direction.values.firstWhere((value) => value.code == code,
        orElse: () => Direction.None);
  }
}
