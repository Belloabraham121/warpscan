use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::sync::Arc;
use tokio::runtime::Runtime;
use warpscan::{blockchain::BlockchainService, cache::CacheManager, config::Config, ui::app::App};

/// Benchmark the main application initialization
fn bench_app_initialization(c: &mut Criterion) {
    c.bench_function("app_initialization", |b| {
        let rt = Runtime::new().unwrap();
        b.iter(|| {
            rt.block_on(async {
                let config = Config::default();
                let cache_manager = CacheManager::new(config.clone()).unwrap();
                let blockchain_client =
                    BlockchainService::new(config.clone(), Arc::new(cache_manager.clone()))
                        .await
                        .unwrap();
                let app = App::new(config, blockchain_client, cache_manager);
                black_box(app)
            })
        });
    });
}

/// Benchmark address lookup
fn bench_address_lookup(c: &mut Criterion) {
    c.bench_function("address_lookup", |b| {
        let rt = Runtime::new().unwrap();
        b.iter(|| {
            rt.block_on(async {
                let config = Config::default();
                let cache_manager = CacheManager::new(config.clone()).unwrap();
                let blockchain_client =
                    BlockchainService::new(config.clone(), Arc::new(cache_manager.clone()))
                        .await
                        .unwrap();
                let mut app = App::new(config, blockchain_client, cache_manager);

                // Benchmark lookup (this will fail if no API key, but we're just measuring time)
                let _ = app
                    .lookup_address(black_box("0x0000000000000000000000000000000000000000"))
                    .await;
                black_box(app.state.clone())
            })
        });
    });
}

/// Benchmark transaction details fetching
fn bench_transaction_details(c: &mut Criterion) {
    c.bench_function("transaction_details", |b| {
        let rt = Runtime::new().unwrap();
        b.iter(|| {
            rt.block_on(async {
                let config = Config::default();
                let cache_manager = CacheManager::new(config.clone()).unwrap();
                let blockchain_client =
                    BlockchainService::new(config.clone(), Arc::new(cache_manager.clone()))
                        .await
                        .unwrap();

                // Benchmark transaction details fetch
                let _ = blockchain_client
                    .get_transaction_details(black_box(
                        "0x0000000000000000000000000000000000000000000000000000000000000000",
                    ))
                    .await;
                black_box(())
            })
        });
    });
}

criterion_group!(
    benches,
    bench_app_initialization,
    bench_address_lookup,
    bench_transaction_details
);
criterion_main!(benches);
