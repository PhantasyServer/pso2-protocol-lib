rm -r failed_tests test_result.txt
cargo test --features ppac -- --nocapture | tee test_result.txt
