use bdk_wallet::wallet::AddressInfo;
use bdk_wallet::KeychainKind;
use bdk_wallet::bitcoin::Network;
use bdk_wallet::Wallet;
use bdk_electrum::{BdkElectrumClient, electrum_client};
use bdk_electrum::electrum_client::Client;

const STOP_GAP: usize = 50;
const BATCH_SIZE: usize = 5;
const EXTERNAL_DESCRIPTOR: &str = "tr(tprv8ZgxMBicQKsPdrjwWCyXqqJ4YqcyG4DmKtjjsRt29v1PtD3r3PuFJAjWytzcvSTKnZAGAkPSmnrdnuHWxCAwy3i1iPhrtKAfXRH7dVCNGp6/86'/1'/0'/0/*)#g9xn7wf9";
const INTERNAL_DESCRIPTOR: &str = "tr(tprv8ZgxMBicQKsPdrjwWCyXqqJ4YqcyG4DmKtjjsRt29v1PtD3r3PuFJAjWytzcvSTKnZAGAkPSmnrdnuHWxCAwy3i1iPhrtKAfXRH7dVCNGp6/86'/1'/0'/1/*)#e3rjrmea";

fn main() -> () {
    print_page_link("electrum/");

    let mut wallet: Wallet = Wallet::new(
        EXTERNAL_DESCRIPTOR,
        INTERNAL_DESCRIPTOR,
        Network::Signet,
    ).unwrap();

    let address: AddressInfo = wallet.reveal_next_address(KeychainKind::External);
    println!("Generated address {} at index {}", address.address, address.index);

    // Syncing the wallet
    let client: BdkElectrumClient<Client> = BdkElectrumClient::new(
        electrum_client::Client::new("ssl://mempool.space:60602").unwrap()
    );

    let full_scan_request = wallet.start_full_scan();
    let mut update = client
        .full_scan(full_scan_request, STOP_GAP, BATCH_SIZE, true).unwrap()
        .with_confirmation_time_height_anchor(&client).unwrap();

    let now = std::time::UNIX_EPOCH.elapsed().unwrap().as_secs();
    let _ = update.graph_update.update_last_seen_unconfirmed(now);

    wallet.apply_update(update).unwrap();
    let balance = wallet.balance();
    println!("Wallet balance: {} sat", balance.total().to_sat());
}

fn print_page_link(link: &str) -> () {
    println!();
    println!("+------------------------------------------------------------------------------------------+");
    println!("|                                                                                          |");
    println!("| Companion code for https://bitcoindevkit.github.io/book-of-bdk/cookbook/wallet/{} |", link);
    println!("|                                                                                          |");
    println!("+------------------------------------------------------------------------------------------+");
    println!();
}
