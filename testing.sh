rm -r failed_tests test_result.txt
cargo test --features ppac,ngs_packets -- --nocapture | tee test_result.txt
