#include "ble_connection.h"

BLEConnection::BLEConnection(const std::string& service_uuid) : service_uuid_{service_uuid} {
        SetupGattScanCallback(gatt_,  std::bind(DefaultScan, gatt_, DefaultNotify, service_uuid_));
        BLEPP::log_level = BLEPP::Error;
        gatt_.cb_disconnected = DefaultDisconnect;
        gatt_.connect_blocking("95b17eef-0276-4e5d-a97b-afc0eff7b4dd");
}

void BLEConnection::Scan() { gatt_.read_and_process_next(); }


void SetupGattScanCallback(BLEPP::BLEGATTStateMachine& gatt, ScanCallback scan_cb) {
    gatt.setup_standard_scan(scan_cb);
}

void DefaultNotify(const BLEPP::PDUNotificationOrIndication& notification) {
    std::string msg(notification.value().first, notification.value().second);
    std::cout << msg << std::endl;
}

void DefaultScan(BLEPP::BLEGATTStateMachine& gatt, const NotifyCallback& notify_cb,
                        const std::string& service_uuid) {
    using std::ranges::views::filter;

    auto ByUUID = [&service_uuid](auto& characteristic) { return characteristic.uuid == BLEPP::UUID(service_uuid); };

    for (auto& service : gatt.primary_services) {
        for (auto& characteristic : service.characteristics | filter(ByUUID)) {
            std::cout << "Characteristic found\n";
            characteristic.cb_notify_or_indicate = notify_cb;
            characteristic.set_notify_and_indicate(true, false);
        }
    }
}

void DefaultDisconnect(BLEPP::BLEGATTStateMachine::Disconnect d) {
    if(d.reason != BLEPP::BLEGATTStateMachine::Disconnect::ConnectionClosed) {
        std::cerr << "Disconnect for reason " << BLEPP::BLEGATTStateMachine::get_disconnect_string(d) << std::endl;
        exit(1);
    } else {
        exit(0);
    }
}
