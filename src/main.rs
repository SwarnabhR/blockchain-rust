use std::time::{SystemTime, UNIX_EPOCH};
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use serde::{Serialize, Deserialize};
use sha2::Digest;
use warp::Filter;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Block {
    index: u32,
    timestamp: u128,
    data: String,
    previous_hash: String,
    hash: String,
    nonce: u64,
}

impl Block {

    fn genesis() -> Block {
        Block {
            index: 0,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis(),

            data: String::from("Genesis Block"),
            previous_hash: String::from("0"),
            hash: String::from("0"),
            nonce: 0,
        }
    }

    // Function to create a new block
    fn new(index: u32, data: String, previous_hash: String) -> Block {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        let mut block = Block {
            index,
            timestamp,
            data,
            previous_hash: previous_hash.clone(),
            hash: String::new(),
            nonce: 0,
        };

        // Calculate the hash for this block
        block.hash = block.calculate_hash();

        block
    }

    // Function to calculate the hash of the block
    fn calculate_hash(&self) -> String {
        let block_data = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.data, self.previous_hash, self.nonce
        );
        format!("{:x}", sha2::Sha256::digest(block_data.as_bytes()))
    }

    // Function to mine a new blockchain by finding a valid hash
    fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);

        while &self.hash[..difficulty] != target {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }

        println!("Block mined: {}", self.hash);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Blockchain {
    chain: Vec<Block>,
    difficulty: usize,
}

impl Blockchain {
    // Function to initialize a new blockchain with a genesis block
    fn new() -> Blockchain {
        Blockchain {
            chain: vec![Block::genesis()],
            difficulty: 4,
        }
    }

    // Function to get the last block in the chain
    fn get_latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    // Function to add a new block to the chain
    fn add_block(&mut self, data: String) {
        let previous_block = self.get_latest_block();
        let mut new_block = Block::new(
            previous_block.index + 1,
            data,
            previous_block.hash.clone(),
        );

        new_block.mine_block(self.difficulty);

        self.chain.push(new_block);
    }

    // Function to validate the integrity of the blockchain
    fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i-1];

            // Check if the current block's hash is corrent
            if current_block.hash != current_block.calculate_hash() {
                println!("Invalid hash for block {}", current_block.index);
                return false;
            }

            // Check if the current block correctly references the prev block
            if current_block.previous_hash != previous_block.hash {
                println!("Invalid previous hash for block {}", current_block.index);
                return false;
            }
        }
        true
    }
}


fn handle_connection(mut stream: TcpStream, blockchain: &Blockchain) {
    let mut buffer = [0; 512];

    // Read the incoming data
    stream.read(&mut buffer).unwrap();

    // Simulate sendig the blockchain to the peer
    let response = serde_json::to_string(&blockchain.chain).unwrap();
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

async fn start_server(blockchain: Arc<Blockchain>) {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Listening on port 7878...");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let blockchain_clone = blockchain.clone();

        thread::spawn(move || {
            handle_connection(stream, &blockchain_clone);
        });
    }
    let blockchain_route = warp::path("blockchain")
        .and(warp::get())
        .map(move || warp::reply::json(&*blockchain));

    warp::serve(blockchain_route)
        .run(([127, 0, 0, 1], 3030))
        .await;


}

fn connect_to_peer(address: &str) {
    let mut stream = TcpStream::connect(address).unwrap();
    let mut buffer = Vec::new();

    stream.read_to_end(&mut buffer).unwrap();

    let peer_blockchain: Vec<Block> = serde_json::from_slice(&buffer).unwrap();
    println!("Received blockchain from {}: {:?}", address, peer_blockchain);
}

#[tokio::main]
async fn main() {
    let mut blockchain = Blockchain::new();

    println!("Mining block 1...");
    blockchain.add_block("First Block after Genesis".to_string());

    println!("Mining block 2...");
    blockchain.add_block("Second Block after Genesis".to_string());

    for block in &blockchain.chain {
        println!("{:?}", block);
    }
    // Validate the chain
    println!("Is blockchain valid? {}", blockchain.is_chain_valid());

    let blockchain = Arc::new(Blockchain::new());
    start_server(Arc::clone(&blockchain)).await;

    connect_to_peer("127.0.0.1:7878");

}