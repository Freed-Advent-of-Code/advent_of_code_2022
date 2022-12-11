class Position {
  int x = 0;
  int y = 0;

  Position(this.x, this.y);

  Position move(Position vector) {
    return Position(this.x + vector.x, this.y + vector.y);
  }

  Position get reverse {
    return Position(this.x * -1, this.y * -1);
  }

  int _getDistanceX(Position otherPos) {
    return (this.x - otherPos.x).abs();
  }

  int _getDistanceY(Position otherPos) {
    return (this.y - otherPos.y).abs();
  }

  int manhattanDistanceWith(Position otherPos) {
    return _getDistanceX(otherPos) + _getDistanceY(otherPos);
  }

  bool isTouchingWith(Position otherPos) {
    return _getDistanceX(otherPos) <= 1 && _getDistanceY(otherPos) <= 1;
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

  Position get unitVector {
    final unitVector = Position(0, 0);

    if (this.direction == Direction.Up) {
      unitVector.y += 1;
    }

    if (this.direction == Direction.Down) {
      unitVector.y -= 1;
    }

    if (this.direction == Direction.Right) {
      unitVector.x += 1;
    }

    if (this.direction == Direction.Left) {
      unitVector.x -= 1;
    }

    return unitVector;
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
