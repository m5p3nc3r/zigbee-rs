use zigbee::InMemoryStorage;
use zigbee_base_device_behavior::BaseDeviceBehavior;
use crate::INPUT_CHANNEL;
use esp_println::println;
use embedded_storage::Storage;
pub const STORAGE_SIZE: usize = 16;

// // Ideal position??

// let app = ZigbeeDevice::new(zigbee::Config::default())
//     .with_device_type(zigbee::LogicalType::EndDevice(EndDeviceConfig::default()))
//     .with_network_layer(EspNetworkLayer::new());

// // Input events such as button press, temperature change, etc.
// let input = INPUT_CHANNEL.receiver();

// loop {
//     match select(app, input) {
//         // The primary app event loop, bubbling events to the application if needed
//         embassy_executor::Select::First(event) => {
//             match event {
//                 // Process application events
//                 _ => {}
//             }
//         }
//         // The input event loop, processing input events and sending info to the network
//         embassy_executor::Select::Second(event) => {
//             match event {
//                 // Process input events
//                 _ => {}
//             }
//         }
//     }
// }

#[embassy_executor::task]
pub async fn zigbee_task() {
    // Create a temporary storage to maintain state
    let mut storage = InMemoryStorage::<STORAGE_SIZE>::default();

    let offset = 0;
    // bdbIsNodeOnANetwork = true
    let data: &[u8] = &[1];
    storage.write(offset, data).expect("Failed to write to storage");

    let nlme = zigbee::nwk::nlme::Nlme {};
    let config = zigbee::Config {
        device_type: zigbee::LogicalType::EndDevice,
        ..zigbee::Config::default()
    };

    let bdb_commissioning_capability = 0u8;

    let mut bdb = BaseDeviceBehavior::new(storage, &nlme, config, bdb_commissioning_capability);
    let _ = bdb.start_initialization_procedure();

    let input = INPUT_CHANNEL.receiver();
    loop {
        let state = input.receive().await;
        println!("Button state changed: {}", state);
    }
}
