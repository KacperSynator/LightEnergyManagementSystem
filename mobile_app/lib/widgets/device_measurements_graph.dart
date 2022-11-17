import 'package:flutter/material.dart';
import 'package:fl_chart/fl_chart.dart';
import 'package:fixnum/fixnum.dart';
import 'package:collection/collection.dart';
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
  FlSpot minXY = FlSpot(0, 0);
  FlSpot maxXY = FlSpot(10, 10);
  List<FlSpot> points = [FlSpot(4, 4), FlSpot(10, 10)];

  @override
  void initState() {
    widget.deviceMeasurementsStreamController.stream.listen((dataPacket) {
      _updateGraph(dataPacket);
    });
    super.initState();
  }

  void _updateGraph(DataPacket dataPacket) {
    this.dataPacket = dataPacket;

    measurementsTypes = dataPacket.deviceMeasurements.first.measurements
        .map(((measurement) => measurement.type))
        .toList();

    final type = measurementsTypes.first;

    final pointsX = _getNormalizedXPoints();

    final maxX = pointsX.reduce(max);

    final pointsY = _getYPointsForType(type);

    final minY = pointsY.reduce(min);
    final maxY = pointsY.reduce(max);

    final pointsXY = IterableZip([pointsX, pointsY])
        .map((pair) => FlSpot(pair[0], pair[1]))
        .toList();

    setState(() {
      name = _getStringForType(measurementsTypes.first);
      minXY = FlSpot(0, minY);
      maxXY = FlSpot(maxX, maxY);
      points = pointsXY;
    });
  }

  List<double> _getNormalizedXPoints() {
    final pointsX = dataPacket!.deviceMeasurements
        .map((deviceMeasurement) => deviceMeasurement.timestamp.toDouble())
        .toList();

    final minX = pointsX.reduce(min);

    return pointsX.map((x) => x - minX).toList();
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
    return Stack(
      children: <Widget>[
        AspectRatio(
          aspectRatio: 1.70,
          child: DecoratedBox(
            decoration: const BoxDecoration(
              borderRadius: BorderRadius.all(
                Radius.circular(18),
              ),
              color: Color(0xff232d37),
            ),
            child: Padding(
              padding: const EdgeInsets.only(
                right: 18,
                left: 12,
                top: 12,
                bottom: 12,
              ),
              child: LineChart(
                mainData(),
              ),
            ),
          ),
        ),
      ],
    );
  }

  Widget bottomTitleWidgets(double value, TitleMeta meta) {
    return SideTitleWidget(
      axisSide: meta.axisSide,
      space: 8,
      child: Text(
        meta.formattedValue,
        style: style,
      ),
    );
  }

  Widget leftTitleWidgets(double value, TitleMeta meta) {
    return SideTitleWidget(
      axisSide: meta.axisSide,
      space: 8,
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
          axisNameWidget: Text(
            name,
            style: style,
          ),
          sideTitles: SideTitles(showTitles: false),
        ),
        bottomTitles: AxisTitles(
          // axisNameWidget: const Text("Time", style: style,),
          sideTitles: SideTitles(
            showTitles: true,
            reservedSize: 36,
            getTitlesWidget: bottomTitleWidgets,
          ),
        ),
        leftTitles: AxisTitles(
          // axisNameWidget: Text("value", style: style,),
          sideTitles: SideTitles(
            showTitles: true,
            getTitlesWidget: leftTitleWidgets,
            reservedSize: 42,
          ),
        ),
      ),
      borderData: FlBorderData(
        show: true,
        border: Border.all(color: const Color(0xff37434d)),
      ),
      minX: minXY.x,
      maxX: maxXY.x,
      minY: minXY.y,
      maxY: maxXY.y,
      lineBarsData: [
        LineChartBarData(
          spots: points,
          isCurved: true,
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
