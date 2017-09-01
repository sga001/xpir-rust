#![feature(custom_attribute, custom_derive, plugin)]

#[macro_use]
extern crate criterion;
extern crate xpir;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use criterion::Criterion;
use std::time::Duration;
use xpir::client::PirClient;
use xpir::server::PirServer;
use rand::ChaChaRng;
use rand::{RngCore, FromEntropy};

const SIZE: usize = 288;
const DIM: u64 = 2;
const ALPHA: u64 = 14;
const NUMS: [u64; 3] = [1 << 16, 1 << 18, 1 << 20];

#[derive(Serialize, Clone)]
struct Element {
    #[serde(serialize_with = "<[_]>::serialize")]
    e: [u8; SIZE],
}

fn setup(c: &mut Criterion) {
    c.bench_function_over_inputs(
        &format!("setup_d{}", DIM),
        |b, &&num| {
            // setup
            let mut rng = ChaChaRng::from_entropy();
            let mut collection = vec![];
            for _ in 0..num {
                let mut x = [0u8; SIZE];
                rng.fill_bytes(&mut x);
                collection.push(x);
            }
            // measurement
            b.iter(|| {
               PirServer::with_params(&collection, ALPHA, DIM)
            })
        },
        &NUMS,
    );
}

fn query(c: &mut Criterion) {
    c.bench_function_over_inputs(
        &format!("query_d{}", DIM),
        |b, &&num| {
            // setup
            let client = PirClient::with_params(SIZE as u64, num, ALPHA, DIM);
            // measurement
            b.iter_with_setup(|| rand::random::<u64>() % num, |idx| client.gen_query(idx));
        },
        &NUMS,
    );
}

fn reply(c: &mut Criterion) {
    c.bench_function_over_inputs(
        &format!("reply_d{}", DIM),
        |b, &&num| {
            // setup
            let mut rng = ChaChaRng::from_entropy();
            let mut collection = vec![];
            for _ in 0..num {
                let mut x = [0u8; SIZE];
                rng.fill_bytes(&mut x);
                collection.push(x);
            }

            let server = PirServer::with_params(&collection, ALPHA, DIM);
            let client = PirClient::with_params(SIZE as u64, num, ALPHA, DIM);

            // measurement
            b.iter_with_setup(
                || client.gen_query(rand::random::<u64>() % num),
                |query| server.gen_reply(&query),
            );
        },
        &NUMS,
    );
}

fn decode(c: &mut Criterion) {
    c.bench_function_over_inputs(
        &format!("decode_d{}", DIM),
        |b, &&num| {
            // setup
            let mut rng = ChaChaRng::from_entropy();
            let mut collection = vec![];
            for _ in 0..num {
                let mut x = [0u8; SIZE];
                rng.fill_bytes(&mut x);
                collection.push(x);
            }

            let server = PirServer::with_params(&collection, ALPHA, DIM);
            let client = PirClient::with_params(SIZE as u64, num, ALPHA, DIM);
            let idx = rand::random::<u64>() % num;
            let query = client.gen_query(idx);
            let reply = server.gen_reply(&query);

            // measurement
            b.iter(|| client.decode_reply::<Element>(&reply));
        },
        &NUMS,
    );
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .sample_size(10)
        .measurement_time(Duration::new(5, 0))
        .warm_up_time(Duration::new(1, 0))
        .without_plots();
    targets = setup, query, reply, decode
}

criterion_main!(benches);
