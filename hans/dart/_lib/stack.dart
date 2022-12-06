class Stack<T> {
  List<T> _storage = [];

  Stack() {}

  Stack.fromList(List<T> list) {
    this._storage = list;
  }

  void push(T value) {
    this._storage.add(value);
  }

  T? pop() {
    return this.isEmpty ? null : this._storage.removeLast();
  }

  int get size => this._storage.length;

  bool get isEmpty => this.size == 0;

  T? get top => this.isEmpty ? null : this._storage[this.size - 1];

  @override
  String toString() {
    return this._storage.toString();
  }
}
