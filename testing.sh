rm -r failed_tests test_result.txt
cargo test -- --nocapture | tee test_result.txt