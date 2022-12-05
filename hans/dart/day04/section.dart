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

  bool containsSection(Section target) {
    return this.start <= target.start && this.end >= target.end;
  }
}
