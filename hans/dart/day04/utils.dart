import 'model.dart';

List<Section> parseToSectionPair(String raw) {
  return raw.split(',').map(Section.fromString).toList();
}
