pub mod transaction_data;
pub mod vest_bitcoin;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::time::Instant;

use transaction_data::*;
use vest::bitcoin::varint::{BtcVarint, VarInt};
use vest::properties::*;
use vest::regular::bytes;
use vest::regular::disjoint::DisjointFrom;
use vest::regular::modifier::*;
use vest::regular::repetition::*;
use vest::regular::sequence::*;
use vest::regular::tag::*;
use vest::regular::uints::*;
use vest::regular::variant::*;
use vest::utils::*;
use vest_bitcoin::*;

use bitcoin::consensus::{Decodable, Encodable};

use criterion::black_box;

use base64::prelude::*;

fn parse_vestbtc_tx() -> Result<(), Box<dyn std::error::Error>> {
    println!("==========================");
    println!("Parsing with vest-bitcoin:");
    for bytes in transaction_data() {
        let (consumed, parsed_tx) = tx().parse(&bytes).unwrap_or_else(|e| {
            panic!("Failed to parse transaction: {}", e);
        });
        println!("consumed: {}", consumed);
        // println!("parsed_tx: {:?}", parsed_tx);
    }

    Ok(())
}

fn parse_rustbtc_tx() -> Result<(), Box<dyn std::error::Error>> {
    println!("==========================");
    println!("Parsing with rust-bitcoin:");
    for bytes in transaction_data() {
        let tx = bitcoin::Transaction::consensus_decode(&mut &bytes[..]).unwrap_or_else(|e| {
            panic!("Failed to parse transaction: {}", e);
        });
        // println!("tx: {:?}", tx);
    }

    Ok(())
}

fn serialize_vestbtc_tx() -> Result<(), Box<dyn std::error::Error>> {
    // println!("==========================");
    // println!("Serializing with vest-bitcoin:");
    for (i, tx_msg) in vestbtc_transaction_msgs().iter().enumerate() {
        let len_cal = tx().length(&tx_msg);
        let mut buf = vec![0; len_cal];
        let len = tx()
            .serialize(tx_msg, &mut buf, 0)
            .unwrap_or_else(|e| panic!("Failed to serialize Handshake: {}", e));
        // println!("len: {}", len);
        // assert_eq!(&buf[0..len], &transaction_data()[i][0..len]);
    }

    Ok(())
}

fn serialize_rustbtc_tx() -> Result<(), Box<dyn std::error::Error>> {
    // println!("==========================");
    // println!("Serializing with rust-bitcoin:");
    for (i, tx_msg) in rustbtc_transaction_msgs().iter().enumerate() {
        // let mut buf = Vec::with_capacity(4096);
        let mut buf = Vec::new();
        // let mut buf = vec![0; 4096];
        let len = tx_msg
            .consensus_encode(&mut buf)
            .unwrap_or_else(|e| panic!("Failed to serialize Handshake: {}", e));
        // println!("len: {}", len);
        // assert_eq!(&buf[0..len], &transaction_data()[i][0..len]);
    }

    Ok(())
}

fn transaction_data() -> [&'static [u8]; 8] {
    [TX1, TX2, TX3, TX4, TX5, TX6, TX7, TX8]
}

fn vestbtc_transaction_msgs<'a>() -> [vest_bitcoin::Tx<'a>; 8] {
    transaction_data().map(|bytes| tx().parse(&bytes).unwrap().1)
}

fn rustbtc_transaction_msgs<'a>() -> [bitcoin::Transaction; 8] {
    transaction_data().map(|bytes| bitcoin::Transaction::consensus_decode(&mut &bytes[..]).unwrap())
}

fn hex_to_bytes(hex: &str) -> Vec<u8> {
    hex::decode(hex).unwrap()
}

/// Load blocks stored in benches/data/sampled_blocks.txt
fn load_bitcoin_blocks(path: &str) -> Vec<Vec<u8>> {
    let mut blocks_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    blocks_path.push(path);
    let blocks_file = File::open(blocks_path).expect("failed to read blocks data");

    BufReader::new(blocks_file)
        .lines()
        .map(|line| {
            BASE64_STANDARD
                .decode(line.unwrap())
                .expect("failed to load test blocks")
        })
        .collect::<Vec<_>>()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test_blocks = load_bitcoin_blocks("benches/data/sampled_blocks.txt");
    let vestbtc_msgs = test_blocks
        .iter()
        .map(|bytes| block().parse(bytes).unwrap().1)
        .collect::<Vec<_>>();
    println!("vestbtc_msgs: {}", vestbtc_msgs.len());
    let rustbtc_msgs = test_blocks
        .iter()
        .map(|bytes| bitcoin::Block::consensus_decode(&mut &bytes[..]).unwrap())
        .collect::<Vec<_>>();
    println!("rustbtc_msgs: {}", rustbtc_msgs.len());
    let max_block_size = test_blocks.iter().map(|block| block.len()).max().unwrap();
    println!("max_block_size: {}", max_block_size);

    let vestbtc_serialized_blocks = vestbtc_msgs
        .iter()
        .map(|blk| {
            let len_cal = block().length(blk);
            let mut buf = vec![0; len_cal];
            block().serialize(blk, &mut buf, 0).unwrap();
            buf
        })
        .collect::<Vec<_>>();
    let rustbtc_serialized_blocks = rustbtc_msgs
        .iter()
        .map(|blk| {
            let mut buf = Vec::new();
            blk.consensus_encode(&mut buf).unwrap();
            buf
        })
        .collect::<Vec<_>>();
    println!(
        "vestbtc_serialized_blocks: {}",
        vestbtc_serialized_blocks.len()
    );
    println!(
        "rustbtc_serialized_blocks: {}",
        rustbtc_serialized_blocks.len()
    );
    assert_eq!(
        vestbtc_serialized_blocks.len(),
        rustbtc_serialized_blocks.len()
    );
    for (i, (vest_blk, rust_blk)) in vestbtc_serialized_blocks
        .iter()
        .zip(rustbtc_serialized_blocks.iter())
        .enumerate()
    {
        assert_eq!(
            vest_blk.len(),
            rust_blk.len(),
            "Block {} has different sizes",
            i
        );
        assert_eq!(vest_blk, rust_blk, "Block {} is different", i);
    }

    // let vestbtc_msgs = vestbtc_transaction_msgs();
    // let rustbtc_msgs = rustbtc_transaction_msgs();

    let bench_vestbtc_serialize = || -> Result<(), Box<dyn std::error::Error>> {
        for blk in vestbtc_msgs.iter() {
            let len_cal = block().length(&blk);
            // let len_cal = tx().length(&blk);
            let mut buf = vec![0; len_cal];
            // let mut buf = vec![0; max_block_size];
            let len = block().serialize(blk, &mut buf, 0)?;
            // let len = tx().serialize(blk, &mut buf, 0)?;
        }
        Ok(())
    };

    let bench_rustbtc_serialize = || -> Result<(), Box<dyn std::error::Error>> {
        for blk in rustbtc_msgs.iter() {
            // let mut buf = Vec::with_capacity(max_block_size);
            let mut buf = Vec::new();
            let len = blk.consensus_encode(&mut buf)?;
        }
        Ok(())
    };

    // parse_vestbtc_tx()?;
    // parse_rustbtc_tx()?;
    // serialize_vestbtc_tx()?;
    // serialize_rustbtc_tx()?;
    // bench_fn(parse_rustbtc_tx)?;
    // bench_fn(parse_vestbtc_tx)?;
    bench_fn(bench_rustbtc_serialize, 50)?;
    bench_fn(bench_vestbtc_serialize, 50)?;
    // bench_fn(bench_vestbtc_serialize, 5000000)?;
    // bench_fn(bench_rustbtc_serialize, 5000000)?;
    // test();

    // for hex in transaction_data() {
    //     println!("{:?}", hex_to_bytes(hex));
    // }

    Ok(())
}

fn bench_fn(
    f: impl Fn() -> Result<(), Box<dyn std::error::Error>>,
    iters: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let start = std::time::Instant::now();
    for _ in 0..iters {
        f()?;
    }
    println!("Time elapsed: {} ms", start.elapsed().as_millis());

    Ok(())
}

