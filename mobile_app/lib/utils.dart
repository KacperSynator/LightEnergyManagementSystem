import 'package:logging/logging.dart';
import 'package:uuid/uuid.dart';
import 'dart:developer';

final logger = Logger("Main logger");

void initLogger() {
  Logger.root.level = Level.ALL;
  Logger.root.onRecord.listen((record) {
    log('${record.level.name}: ${record.time}: ${record.message} ');
  });
}

String getUniqueId() {
  return const Uuid().v4();
}
