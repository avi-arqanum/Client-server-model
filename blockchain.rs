use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f32,
}

#[derive(Debug)]
struct Block {
    prev_block_hash: String,
    curr_block_hash: String,
    transactions: Vec<Transaction>,
    nonce: u32,
}

impl Block {
    fn new(prev_block_hash: String, transactions: Vec<Transaction>) -> Self {
        let mut block = Block {
            prev_block_hash,
            curr_block_hash: String::new(),
            transactions,
            nonce: 0,
        };

        block.curr_block_hash = block.compute_hash();
        block
    }

    fn compute_hash(&self) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();

        hasher.update(format!("{:?}{:?}{:?}", &self.prev_block_hash, &self.transactions, self.nonce));
        format!("{:x}", hasher.finalize())
    }

    fn mine(&mut self, difficulty: usize) {
        while &self.curr_block_hash[..difficulty] != "0".repeat(difficulty).as_str() {
            self.nonce += 1;
            self.curr_block_hash = self.compute_hash();
        }
    }
}

struct Blockchain {
    blocks: Vec<Block>,
    unconfirmed_transactions: Vec<Transaction>,
    difficulty: usize,
}

impl Blockchain {
    fn new() -> Self {
        let minting_transaction = Transaction {
            sender: String::from("Creator"),
            receiver: creator_address.to_string(),
            amount: 1_000_000.0,
        };

        let genesis_block = Block::new(String::from("0"), vec![minting_transaction]);

        Blockchain {
            blocks: vec![genesis_block],
            unconfirmed_transactions: Vec::new(),
            difficulty: 4,
        }
    }

    fn calculate_balance(&self, user: &str) -> f32 {
        let mut balance = 0.0;
        
        for block in &self.blocks {
            for transaction in &block.transactions {
                if transaction.sender == user {
                    balance -= transaction.amount;
                } else if transaction.receiver == user {
                    balance += transaction.amount;
                }
            }
        }

        balance
    }

    fn add_transaction(&mut self, sender: String, receiver: String, amount: f32) -> bool {
        let sender_balance = self.calculate_balance(&sender);
        
        if sender_balance >= amount {
            self.unconfirmed_transactions.push(Transaction { sender, receiver, amount });

            if self.unconfirmed_transactions.len() == 6 {
                self.mine_unconfirmed_transactions();
            }

            true
        } else {
            false
        }
    }

    fn mine_unconfirmed_transactions(&mut self) {
        if self.unconfirmed_transactions.is_empty() {
            return;
        }

        let mut new_block = Block::new(self.last_block_hash(), self.unconfirmed_transactions.clone());
        new_block.mine(self.difficulty);
        self.blocks.push(new_block);
        self.unconfirmed_transactions.clear();
    }

    fn last_block_hash(&self) -> String {
        self.blocks.last().unwrap().curr_block_hash.clone()
    }
}

fn main() {
    let mut blockchain = Blockchain::new();

    blockchain.add_transaction(String::from("Creator"), String::from("Alice"), 50.0);
    blockchain.add_transaction(String::from("Alice"), String::from("Bob"), 25.0);
    
    for block in &blockchain.blocks {
        println!("{:?}", block);
    }
}
