#include "ble_connection.h"
#include "lamp_controller.pb.h"

constexpr auto LampControlerUUID {"75b17eef-0276-4e5d-a97b-afc0eff7b4dd"};

int main() {
    BLEConnection ble_connection{LampControlerUUID};

    while(true) {
        ble_connection.Scan();
    }
}

// #include <blepp/blestatemachine.h>  //for UUID. FIXME mofo
// #include <blepp/lescan.h>
// #include <blepp/logging.h>
// #include <blepp/pretty_printers.h>
// #include <bluetooth/bluetooth.h>
// #include <bluetooth/hci.h>
// #include <bluetooth/hci_lib.h>
// #include <signal.h>
// #include <stdio.h>
// #include <unistd.h>

// #include <array>
// #include <boost/optional.hpp>
// #include <cerrno>
// #include <iomanip>
// #include <stdexcept>
// #include <string>
// #include <vector>

// using namespace BLEPP;

// constexpr auto help{R"X(-[sHbdhp]:
//         -s  software filtering of duplicates (default)
//         -H  hardware filtering of duplicates 
//         -b  both hardware and software filtering
//         -d  show duplicates (no filtering)
//         -h  show this message
//         -p  passive scan
//         )X"};

// constexpr auto throbber{"/|\\-"};

// void catch_function(int) { std::cerr << "\nInterrupted!\n"; }

// int main(int argc, char** argv) {
//     HCIScanner::ScanType type = HCIScanner::ScanType::Active;
//     HCIScanner::FilterDuplicates filter = HCIScanner::FilterDuplicates::Software;

//     // std::cin.get();
//     int c{};
//     while ((c = getopt(argc, argv, "sHbdhp")) != -1) {
//         if (c == 'p')
//             type = HCIScanner::ScanType::Passive;
//         else if (c == 's')
//             filter = HCIScanner::FilterDuplicates::Software;
//         else if (c == 'H')
//             filter = HCIScanner::FilterDuplicates::Hardware;
//         else if (c == 'b')
//             filter = HCIScanner::FilterDuplicates::Both;
//         else if (c == 'd')
//             filter = HCIScanner::FilterDuplicates::Off;
//         else if (c == 'h') {
//             std::cout << "Usage: " << argv[0] << " " << help;
//             return 0;
//         } else {
//             std::cerr << argv[0] << ":  unknown option " << c << std::endl;
//             return 1;
//         }
//     }

//     log_level = LogLevels::Warning;
//     HCIScanner scanner(true, filter, type);

//     // //Catch the interrupt signal. If the scanner is not
//     // //cleaned up properly, then it doesn't reset the HCI state.
//     signal(SIGINT, catch_function);

//     // //Something to print to demonstrate the timeout.

//     // //hide cursor, to make the throbber look nicer.
//     std::cout << "[?25l" << std::flush;

//     int i{};
//     while (1) {
//         // Check to see if there's anything to read from the HCI
//         // and wait if there's not.
//         struct timeval timeout;
//         timeout.tv_sec = 0;
//         timeout.tv_usec = 300000;

//         fd_set fds;
//         FD_ZERO(&fds);
//         FD_SET(scanner.get_fd(), &fds);
//         int err = select(scanner.get_fd() + 1, &fds, NULL, NULL, &timeout);

//         // Interrupted, so quit and clean up properly.
//         if (err < 0 && errno == EINTR) break;

//         if (FD_ISSET(scanner.get_fd(), &fds)) {
//             // Only read id there's something to read
//             std::vector<AdvertisingResponse> ads = scanner.get_advertisements();

//             for (const auto& ad : ads) {
//                 std::cout << "Found device: " << ad.address << " ";

//                 if (ad.type == LeAdvertisingEventType::ADV_IND)
//                     std::cout << "Connectable undirected" << std::endl;
//                 else if (ad.type == LeAdvertisingEventType::ADV_DIRECT_IND)
//                     std::cout << "Connectable directed" << std::endl;
//                 else if (ad.type == LeAdvertisingEventType::ADV_SCAN_IND)
//                     std::cout << "Scannable " << std::endl;
//                 else if (ad.type == LeAdvertisingEventType::ADV_NONCONN_IND)
//                     std::cout << "Non connectable" << std::endl;
//                 else
//                     std::cout << "Scan response" << std::endl;
//                 for (const auto& uuid : ad.UUIDs) std::cout << "  Service: " << to_str(uuid) << std::endl;
//                 if (ad.local_name) std::cout << "  Name: " << ad.local_name->name << std::endl;
//                 if (ad.rssi == 127)
//                     std::cout << "  RSSI: unavailable" << std::endl;
//                 else if (ad.rssi <= 20)
//                     std::cout << "  RSSI = " << (int)ad.rssi << " dBm" << std::endl;
//                 else
//                     std::cout << "  RSSI = " << to_hex((uint8_t)ad.rssi) << " unknown" << std::endl;
//             }
//         } else
//             std::cout << throbber[i % 4] << "\b" << std::flush;
//         i++;
//     }

//     // show cursor
//     std::cout << "[?25h" << std::flush;
// }