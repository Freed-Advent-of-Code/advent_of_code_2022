class Position {
  int x = 0;
  int y = 0;

  Position(this.x, this.y);

  Position move(Position vector) {
    return Position(this.x + vector.x, this.y + vector.y);
  }

  Position followTo(Position target, Motion motion) {
    if (_getManhattanDistanceWith(target) > 2) {
      return target.move(motion.unitVector.reverse);
    } else {
      return this.move(motion.unitVector);
    }
  }

  Position get reverse {
    return Position(this.x * -1, this.y * -1);
  }

  int _getDistanceX(Position target) {
    return (this.x - target.x).abs();
  }

  int _getDistanceY(Position target) {
    return (this.y - target.y).abs();
  }

  int _getManhattanDistanceWith(Position target) {
    return _getDistanceX(target) + _getDistanceY(target);
  }

  bool isTouchingWith(Position target) {
    return _getDistanceX(target) <= 1 && _getDistanceY(target) <= 1;
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
