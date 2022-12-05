import 'dart:io';

String readFile(String fileName) {
  final currentContextFilePath = Directory.current.path;
  return File('${currentContextFilePath}/${fileName}').readAsStringSync();
}
