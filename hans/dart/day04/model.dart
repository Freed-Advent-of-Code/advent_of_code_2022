class Section {
  static String separator = '-';

  int start = 0;
  int end = 0;

  static Section fromString(String raw) {
    final splited = raw.split(separator);
    return new Section(int.parse(splited[0]), int.parse(splited[1]));
  }

  Section(int start, int end) {
    this.start = start;
    this.end = end;
  }

  String toString() {
    return '${this.start}${Section.separator}${this.end}';
  }

  bool isContainsSection(Section target) {
    return this.start <= target.start && this.end >= target.end;
  }

  bool isOverlapSection(Section target) {
    return this._isContainsNumber(target.start) ||
        this._isContainsNumber(target.end) ||
        this.isContainsSection(target) ||
        target.isContainsSection(this);
  }

  bool _isContainsNumber(int number) {
    return this.start <= number && this.end >= number;
  }
}
