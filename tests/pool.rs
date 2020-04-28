use porus::prelude::*;

use std::mem::size_of;

trait PoolTest {
    type Handle: pool::Handle;

    fn test_handle_non_null() {
        assert!(size_of::<Self::Handle>() == size_of::<Option<Self::Handle>>());
    }
}

struct TestChunk {}

impl PoolTest for TestChunk {
    type Handle = porus::chunk::Handle;
}

#[test]
fn test_chunk_handle_non_nul() {
    TestChunk::test_handle_non_null();
}
