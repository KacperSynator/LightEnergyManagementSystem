import 'package:flutter/material.dart';
import 'package:fl_chart/fl_chart.dart';
import 'package:collection/collection.dart';
import 'package:mobile_app/utils.dart';
import 'package:intl/intl.dart';
import 'dart:math';
import 'dart:async';

import '../proto/light_energy_management_system.pb.dart';

class DeviceMeasurementsGraph extends StatefulWidget {
  final StreamController<DataPacket> deviceMeasurementsStreamController;

  const DeviceMeasurementsGraph(
      {super.key, required this.deviceMeasurementsStreamController});

  @override
  State<DeviceMeasurementsGraph> createState() =>
      _DeviceMeasurementsGraphState();
}

class _DeviceMeasurementsGraphState extends State<DeviceMeasurementsGraph> {
  static const style = TextStyle(
    color: Color(0xff67727d),
    fontWeight: FontWeight.bold,
    fontSize: 15,
  );

  List<Color> gradientColors = [
    const Color(0xff23b6e6),
    const Color(0xff02d39a),
  ];

  DataPacket? dataPacket;
  List<MeasurementType> measurementsTypes = [];
  String name = "Device: (wololo)";
  FlSpot minXY = const FlSpot(0, 0);
  FlSpot maxXY = const FlSpot(10, 10);
  double subtractedTimestamp = 0;
  List<FlSpot> points = [const FlSpot(4, 4), const FlSpot(10, 10)];

  @override
  void initState() {
    widget.deviceMeasurementsStreamController.stream.listen((dataPacket) {
      _updateGraph(dataPacket);
    });
    super.initState();
  }

  void _updateGraph(DataPacket dataPacket) {
    this.dataPacket = dataPacket;

    if (dataPacket.deviceMeasurements.isEmpty) {
      logger.warning("DeviceMeasurementsGraph::_updateGraph: No measuremnts");

      setState(() {
        name = "Device: ${dataPacket.device.name} ()";
        points = [];
      });

      return;
    }

    measurementsTypes = dataPacket.deviceMeasurements.first.measurements
        .map(((measurement) => measurement.type))
        .toList();

    dataPacket.deviceMeasurements
        .sort(((a, b) => a.timestamp.compareTo(b.timestamp)));

    logger.info(
        "DeviceGraph:_updateGraph: deviceMeasurements: ${dataPacket.deviceMeasurements}");

    if (measurementsTypes.isEmpty) {
      return;
    }

    final type = measurementsTypes.first;

    final pointsX = _getNormalizedXPoints();

    final minX = pointsX.first;
    final maxX = pointsX.last;

    final pointsY = _getYPointsForType(type);

    final minY = pointsY.reduce(min);
    final maxY = pointsY.reduce(max);

    final pointsXY = IterableZip([pointsX, pointsY])
        .map((pair) => FlSpot(pair[0], pair[1]))
        .toList();

    setState(() {
      name = _getStringForType(measurementsTypes.first);
      minXY = FlSpot(minX, minY);
      maxXY = FlSpot(maxX, maxY);
      points = pointsXY;
    });
  }

  List<double> _getNormalizedXPoints() {
    final pointsX = dataPacket!.deviceMeasurements
        .map((deviceMeasurement) => deviceMeasurement.timestamp.toDouble())
        .toList();

    subtractedTimestamp = pointsX.first;

    return pointsX.map((x) => x - subtractedTimestamp).toList();
  }

  void _changeMeasurementType(MeasurementType type) {
    setState(() {
      name = _getStringForType(type);

      points = IterableZip(
              [points.map((point) => point.x), _getYPointsForType(type)])
          .map((pair) => FlSpot(pair[0], pair[1]))
          .toList();
      minXY = FlSpot(minXY.x, points.map((e) => e.y).reduce(min));
      maxXY = FlSpot(maxXY.x, points.map((e) => e.y).reduce(max));
    });
  }

  List<double> _getYPointsForType(MeasurementType type) {
    return dataPacket!.deviceMeasurements
        .map((deviceMeasurement) => deviceMeasurement.measurements
            .where((measurement) => measurement.type == type)
            .map((measurement) => measurement.value)
            .first)
        .toList();
  }

  String _getStringForType(MeasurementType type) {
    return "Device: ${dataPacket!.device.name} ($type)";
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.center,
      children: [
        Stack(
          children: <Widget>[
            AspectRatio(
              aspectRatio: 1.4,
              child: DecoratedBox(
                decoration: const BoxDecoration(
                  borderRadius: BorderRadius.all(
                    Radius.circular(18),
                  ),
                  color: Color(0xff232d37),
                ),
                child: Padding(
                  padding: const EdgeInsets.only(
                    right: 35,
                    top: 10,
                    bottom: 10,
                  ),
                  child: LineChart(
                    mainData(),
                  ),
                ),
              ),
            ),
          ],
        ),
        SizedBox(
          width: double.infinity,
          height: 50,
          child: ListView.builder(
            scrollDirection: Axis.horizontal,
            itemBuilder: (ctx, index) => Padding(
              padding: const EdgeInsets.all(10),
              child: ElevatedButton(
                onPressed: () =>
                    _changeMeasurementType(measurementsTypes[index]),
                style: ElevatedButton.styleFrom(
                    foregroundColor: const Color(0xff02d39a),
                    backgroundColor: const Color(0xff232d37)),
                child: Text("${measurementsTypes[index]}"),
              ),
            ),
            itemCount: measurementsTypes.length,
            shrinkWrap: true,
          ),
        ),
      ],
    );
  }

  Widget bottomTitleWidgets(double value, TitleMeta meta) {
    final dateTime = DateTime.fromMillisecondsSinceEpoch((value + subtractedTimestamp).toInt() * 1000);
    return SideTitleWidget(
      axisSide: meta.axisSide,
      space: 10,
      child: RotatedBox(
        quarterTurns: 0,
        child: Text(
          DateFormat('dd/MM/yy\nHH:mm:ss').format(dateTime),
          style: style,
        ),
      ),
    );
  }

  Widget leftTitleWidgets(double value, TitleMeta meta) {
    return SideTitleWidget(
      axisSide: meta.axisSide,
      space: 3,
      child: Text(
        meta.formattedValue,
        style: style,
      ),
    );
  }

  LineChartData mainData() {
    return LineChartData(
      gridData: FlGridData(
        show: true,
        drawVerticalLine: true,
        getDrawingHorizontalLine: (value) {
          return FlLine(
            color: const Color(0xff37434d),
            strokeWidth: 1,
          );
        },
        getDrawingVerticalLine: (value) {
          return FlLine(
            color: const Color(0xff37434d),
            strokeWidth: 1,
          );
        },
      ),
      titlesData: FlTitlesData(
        show: true,
        rightTitles: AxisTitles(
          sideTitles: SideTitles(showTitles: false),
        ),
        topTitles: AxisTitles(
          axisNameSize: 30,
          axisNameWidget: Padding(
            padding: const EdgeInsets.only(bottom: 10),
            child: Text(
              name,
              style: const TextStyle(
                color: Color(0xff02d39a),
                fontWeight: FontWeight.bold,
                fontSize: 16,
              ),
            ),
          ),
          sideTitles: SideTitles(showTitles: false),
        ),
        bottomTitles: AxisTitles(
          // axisNameWidget: const Text("Time", style: style,),
          sideTitles: SideTitles(
            showTitles: true,
            reservedSize: 50,
            getTitlesWidget: bottomTitleWidgets,
            interval: max(maxXY.x / 3, 0.01),
          ),
        ),
        leftTitles: AxisTitles(
          // axisNameWidget: Text("value", style: style,),
          sideTitles: SideTitles(
            showTitles: true,
            getTitlesWidget: leftTitleWidgets,
            reservedSize: 55,
            interval: max(maxXY.y / 3, 0.01),
          ),
        ),
      ),
      borderData: FlBorderData(
        show: true,
        border: Border.all(color: const Color(0xff37434d)),
      ),
      minX: minXY.x,
      maxX: maxXY.x,
      minY: min(minXY.y, 0),
      maxY: maxXY.y,
      lineBarsData: [
        LineChartBarData(
          spots: points,
          isCurved: false,
          gradient: LinearGradient(
            colors: gradientColors,
          ),
          barWidth: 5,
          isStrokeCapRound: true,
          dotData: FlDotData(
            show: false,
          ),
          belowBarData: BarAreaData(
            show: true,
            gradient: LinearGradient(
              colors: gradientColors
                  .map((color) => color.withOpacity(0.3))
                  .toList(),
            ),
          ),
        ),
      ],
    );
  }
}
