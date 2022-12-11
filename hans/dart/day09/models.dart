class Position {
  int x = 0;
  int y = 0;

  Position(this.x, this.y);

  bool isTouchingWith(Position otherPos) {
    int distanceX = (this.x - otherPos.x).abs();
    int distanceY = (this.y - otherPos.y).abs();

    return distanceX <= 1 && distanceY <= 1;
  }
}
