class Position {
  int x = 0;
  int y = 0;

  Position(this.x, this.y);

  Position move(Motion motion) {
    Position newPosition = Position(this.x, this.y);

    if (motion.direction == Direction.Right) {
      newPosition.x += motion.moveAmount;
    }

    if (motion.direction == Direction.Left) {
      newPosition.x -= motion.moveAmount;
    }

    if (motion.direction == Direction.Up) {
      newPosition.y += motion.moveAmount;
    }

    if (motion.direction == Direction.Down) {
      newPosition.y -= motion.moveAmount;
    }

    return newPosition;
  }

  Position followTo(Position target) {
    if (this.isTouchingWith(target)) {
      return this;
    }

    final differ = this.differ(target);
    if (this._getManhattanDistance(target) == 2) {
      return this.move(Motion.fromVector(differ).unitVector);
    }

    return this
        .move(Motion.fromVector(Position(0, differ.y)).unitVector)
        .move(Motion.fromVector(Position(differ.x, 0)).unitVector);
  }

  Position get reverse {
    return Position(this.x * -1, this.y * -1);
  }

  bool isTouchingWith(Position target) {
    return _getDistanceX(target) <= 1 && _getDistanceY(target) <= 1;
  }

  Position differ(Position target) {
    return Position(target.x - this.x, target.y - this.y);
  }

  int _getDistanceX(Position target) {
    return (this.x - target.x).abs();
  }

  int _getDistanceY(Position target) {
    return (this.y - target.y).abs();
  }

  int _getManhattanDistance(Position target) {
    return _getDistanceX(target) + _getDistanceY(target);
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

  factory Motion.fromDirection(Direction direction) {
    return Motion(direction, 1);
  }

  factory Motion.fromVector(Position vector, {int moveAmount = 0}) {
    if (vector.x > 0) {
      return Motion(Direction.Right, moveAmount == 0 ? vector.x : moveAmount);
    }

    if (vector.x < 0) {
      return Motion(Direction.Left, moveAmount == 0 ? vector.x : moveAmount);
    }

    if (vector.y > 0) {
      return Motion(Direction.Up, moveAmount == 0 ? vector.y : moveAmount);
    }

    if (vector.y < 0) {
      return Motion(Direction.Down, moveAmount == 0 ? vector.y : moveAmount);
    }

    return Motion(Direction.None, 0);
  }

  Motion get unitVector {
    return Motion(this.direction, 1);
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

class Rope {
  final Set<String> tailVisited = {};
  final List<Position> knots;

  Rope(this.knots);

  factory Rope.fromKnotsCount(int knotsCount) {
    return Rope(List<Position>.generate(knotsCount, (_) => Position(0, 0)));
  }

  Position get head {
    return knots[0];
  }

  void set head(Position head) {
    knots[0] = head;
  }

  Position get tail {
    return knots[knots.length - 1];
  }

  void move(Motion motion) {
    for (int _ = 0; _ < motion.moveAmount; _++) {
      head = head.move(motion.unitVector);
      for (int i = 1; i < knots.length; i++) {
        if (!knots[i].isTouchingWith(knots[i - 1])) {
          knots[i] = knots[i].followTo(knots[i - 1]);
        }
      }

      tailVisited.add(tail.toString());
    }
  }
}
