mod GameSpyBackendServer;
// mod GameSpyQRServer;
// mod GameSpyProfileServer;
// mod GameSpyPlayerSearchServer;
// mod GameSpyGamestatsServer;
// mod GameSpyNatNegServer;
// mod NasServer;
// mod Dls1Server;
// mod InternalStatsServer;
// mod AdminPageServer;
// mod RegPageServer;
// mod StorageServer;
// mod GameStatsServer;

use std::thread;

fn main() {
    // init the database
    GameSpyBackendServer::main();
    
    
    // this is the general format we will use to run the threads for each server
    thread::spawn(|| {
        println!("Hello from thread!")
    });
    println!("Hello, main thread!");
}
