#pragma once

#include <blepp/blestatemachine.h>

#include <algorithm>
#include <chrono>
#include <iomanip>
#include <iostream>
#include <ranges>
#include <utility>


using NotifyCallback = std::function<void(const BLEPP::PDUNotificationOrIndication&)>;
using ScanCallback = std::function<void()>;

class BLEConnection {
   public:
    BLEConnection(const std::string& service_uuid, const std::string& device_address);
    void Scan();

   private:
    std::string service_uuid_;
    BLEPP::BLEGATTStateMachine gatt_;
};

inline void DefaultScan(BLEPP::BLEGATTStateMachine& gatt, const NotifyCallback& notify_cb,
                        const std::string& service_uuid);
inline void DefaultNotify(const BLEPP::PDUNotificationOrIndication& notification);
inline void SetupGattScanCallback(BLEPP::BLEGATTStateMachine& gatt, ScanCallback scan_cb);
inline void DefaultDisconnect(BLEPP::BLEGATTStateMachine::Disconnect d);
