// #include <iostream>
// #include <iomanip>
// #include <blepp/blestatemachine.h>
// #include <blepp/float.h>  //BLE uses the obscure IEEE11073 decimal exponent floating point values
// #include <unistd.h>
// #include <chrono>
// #include <algorithm>
// using namespace std;
// using namespace chrono;
// using namespace BLEPP;

// int main(int argc, char **argv)
// {
// 	if(argc != 2)
// 	{	
// 		cerr << "Please supply address.\n";
// 		cerr << "Usage:\n";
// 		cerr << "prog <addres>";
// 		exit(1);
// 	}

// 	log_level = Error;

// 	//This class does all of the GATT interactions for you.
// 	//It's a callback based interface, so you need to provide 
// 	//callbacks to make it do anything useful. Don't worry: this is
// 	//a library, not a "framework", so it won't steal your main loop.
// 	//In other examples, I show you how to get and use the file descriptor, so 
// 	//you will only make calls into BLEGATTStateMachine when there's data
// 	//to process.
// 	BLEGATTStateMachine gatt;

// 	//This function will be called when a push notification arrives from the device.
// 	//Not much sanity/error checking here, just for clarity.
// 	//Basically, extract the float and log it along with the time.
// 	std::function<void(const PDUNotificationOrIndication&)> notify_cb = [&](const PDUNotificationOrIndication& n)
// 	{
//         std::string msg {n.value().first, n.value().second};
//         std::cout << "Len: " << msg.size() << "\nMessage: ";
// 		std::ranges::for_each(msg, [](const auto& c) {std::cout << c;});
//         std::cout << "\n--------------\n";
// 	};
	
// 	//This is called when a complete scan of the device is done, giving
// 	//all services and characteristics. This one simply searches for the 
// 	//standardised "temperature" characteristic (aggressively cheating and not
// 	//bothering to check if the service is correct) and sets up the device to 
// 	//send us notifications.
// 	//
// 	//This will simply sit there happily connected in blissful ignorance if there's
// 	//no temperature characteristic.
// 	std::function<void()> found_services_and_characteristics_cb = [&gatt, &notify_cb](){
// 		for(auto& service: gatt.primary_services)
// 			for(auto& characteristic: service.characteristics)
// 				if(characteristic.uuid == UUID("95b17eef-0276-4e5d-a97b-afc0eff7b4dd"))
// 				{
// 					characteristic.cb_notify_or_indicate = notify_cb;
// 					characteristic.set_notify_and_indicate(true, false);
// 				}
// 	};
	
// 	//This is the simplest way of using a bluetooth device. If you call this 
// 	//helper function, it will put everything in place to do a complete scan for
// 	//services and characteristics when you connect. If you want to save a small amount
// 	//of time on a connect and avoid the complete scan (you are allowed to cache this 
// 	//information in certain cases), then you can provide your own callbacks.
// 	gatt.setup_standard_scan(found_services_and_characteristics_cb);

// 	//I think this one is reasonably clear?
// 	gatt.cb_disconnected = [](BLEGATTStateMachine::Disconnect d)
// 	{
// 		cerr << "Disconnect for reason " << BLEGATTStateMachine::get_disconnect_string(d) << endl;
// 		exit(1);
// 	};
	
// 	//This is how to use the blocking interface. It is very simple. You provide the main 
// 	//loop and just hammer on the state machine struct. 
// 	gatt.connect_blocking(argv[1]);
// 	for(;;)
// 		gatt.read_and_process_next();

// }



#include <chrono>
#include <thread>

#include "ble_connection.h"
// #include "lamp_controller.pb.h"

constexpr auto LampControlerUUID {"95b17eef-0276-4e5d-a97b-afc0eff7b4dd"};
constexpr auto LampControlerMacAddress {"EC:62:60:93:A4:B2"};

int main() {
    using namespace std::chrono_literals;

    BLEConnection ble_connection{LampControlerUUID, LampControlerMacAddress};

    while(true) {
        ble_connection.Scan();
        std::this_thread::sleep_for(1s);
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