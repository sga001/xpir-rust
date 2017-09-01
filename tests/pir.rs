extern crate rand;
extern crate xpir;

use std::mem;
use xpir::client::PirClient;
use xpir::server::PirServer;
use rand::RngCore;
use std::thread;

macro_rules! get_size {
    ($d_type:ty) => (mem::size_of::<$d_type>() as u64);
}

#[test]
fn pir_decode_d3() {
    let num = 8192;
    let alpha = 8;
    let d = 3;
    let mut collection: Vec<[u8; 32]> = Vec::new();

    let mut rng = rand::thread_rng();

    for _ in 0..num {
        let mut x: [u8; 32] = [0; 32];
        rng.fill_bytes(&mut x);

        collection.push(x);
    }

    let truth = collection.clone();

    // Create the client
    let mut client = PirClient::new(1, 1);

    let first = 0;
    let last = 1;
    let test_num = last - first;

    let server = PirServer::with_params(&collection[first..last], alpha, d);
    client.update_params(get_size!([u8; 32]), test_num as u64, alpha, d);

    for i in 0..test_num {
        let query = client.gen_query(i as u64);
        let reply = server.gen_reply(&query);
        let result = client.decode_reply::<[u8; 32]>(&reply);
        assert_eq!(result, truth[first + i as usize]);
    }

    let first = 1;
    let last = 3;
    let test_num = last - first;

    let server_2 = PirServer::with_params(&collection[first..last], alpha, d);
    client.update_params(get_size!([u8; 32]), test_num as u64, alpha, d);

    for i in 0..test_num {
        let query = client.gen_query(i as u64);
        let reply = server_2.gen_reply(&query);
        let result = client.decode_reply::<[u8; 32]>(&reply);
        assert_eq!(result, truth[first + i as usize]);
    }

    let first = 3;
    let last = 8;
    let test_num = last - first;

    let server_3 = PirServer::with_params(&collection[first..last], alpha, d);
    client.update_params(get_size!([u8; 32]), test_num as u64, alpha, d);

    for i in 0..test_num {
        let query = client.gen_query(i as u64);
        let reply = server_3.gen_reply(&query);
        let result = client.decode_reply::<[u8; 32]>(&reply);
        assert_eq!(result, truth[first + i as usize]);
    }
}

#[test]
fn pir_decode() {
    let num = 8;
    let alpha = 8;
    let d = 2;
    let mut collection: Vec<[u8; 32]> = Vec::new();

    let mut rng = rand::thread_rng();

    for _ in 0..num {
        let mut x: [u8; 32] = [0; 32];
        rng.fill_bytes(&mut x);

        collection.push(x);
    }

    let truth = collection.clone();

    // Create the client
    let mut client = PirClient::new(1, 1);

    let first = 0;
    let last = 1;
    let test_num = last - first;

    let server = PirServer::with_params(&collection[first..last], alpha, d);
    client.update_params(get_size!([u8; 32]), test_num as u64, alpha, d);

    for i in 0..test_num {
        let query = client.gen_query(i as u64);
        let reply = server.gen_reply(&query);
        let result = client.decode_reply::<[u8; 32]>(&reply);
        assert_eq!(result, truth[first + i as usize]);
    }

    let first = 1;
    let last = 3;
    let test_num = last - first;

    let server_2 = PirServer::with_params(&collection[first..last], alpha, d);
    client.update_params(get_size!([u8; 32]), test_num as u64, alpha, d);

    for i in 0..test_num {
        let query = client.gen_query(i as u64);
        let reply = server_2.gen_reply(&query);
        let result = client.decode_reply::<[u8; 32]>(&reply);
        assert_eq!(result, truth[first + i as usize]);
    }

    let first = 3;
    let last = 8;
    let test_num = last - first;

    let server_3 = PirServer::with_params(&collection[first..last], alpha, d);
    client.update_params(get_size!([u8; 32]), test_num as u64, alpha, d);

    for i in 0..test_num {
        let query = client.gen_query(i as u64);
        let reply = server_3.gen_reply(&query);
        let result = client.decode_reply::<[u8; 32]>(&reply);
        assert_eq!(result, truth[first + i as usize]);
    }
}

#[test]
fn pir_threads() {
    let num = 6;
    let alpha = 1;
    let d = 2;
    let mut collection: Vec<[u8; 32]> = Vec::new();

    let mut rng = rand::thread_rng();

    for _ in 0..num {
        let mut x: [u8; 32] = [0; 32];
        rng.fill_bytes(&mut x);

        collection.push(x);
    }

    let truth = collection.clone();

    // Create the client

    let first = 0;
    let last = 5;
    let test_num = last - first;

    let mut ids = Vec::new();

    for i in 0..test_num {
        let server = PirServer::with_params(&collection[first..last], alpha, d);
        let mut client = PirClient::new(1, 1);
        client.update_params(get_size!([u8; 32]), test_num as u64, alpha, d);

        let truth_clone = truth.clone();

        ids.push(thread::spawn(move || {
            let query = client.gen_query(i as u64);
            let reply = server.gen_reply(&query);
            let result = client.decode_reply::<[u8; 32]>(&reply);
            assert_eq!(result, truth_clone[first + i as usize]);
        }));
    }

    for thread in ids.drain(..) {
        thread.join().unwrap();
    }
}


#[test]
fn pir_sizes() {
    let size = 288;
    let index = 70;

    let mut collection: Vec<[u8; 288]> = Vec::new();

    let mut rng = rand::thread_rng();

    let alphas = vec![14, 16];
    let ds = vec![2, 3];
    let ns = vec![1<<16, 1<<18, 1<<20];

    let mut num_prev = 0;

    for num in ns {

        for _ in num_prev..num {
            let mut x: [u8; 288] = [0; 288];
            rng.fill_bytes(&mut x);

            collection.push(x);
        }

        num_prev = num;

        for d in &ds { 
            for alpha in &alphas {
                let server = PirServer::with_params(&collection, *alpha, *d);
                let client = PirClient::with_params(size, num, *alpha, *d);

                let query = client.gen_query(index);
                let reply = server.gen_reply(&query);
                println!("query: num {}, a {}, d {}, size {}", num, alpha, *d, 
                         query.query.len() / 1024);
                println!("reply num {}, a {}, d {}, size {}", num, alpha, *d, 
                         reply.reply.len() / 1024);
            }
        }
    }
}


