use std::sync::Arc;

pub trait DbAdapter {

}

pub struct PgAdapter {

}

impl DbAdapter for PgAdapter {

}

impl PgAdapter {
    pub fn new() -> PgAdapter {
        PgAdapter {}
    }
}

pub struct MockPgAdapter {

}

impl DbAdapter for MockPgAdapter {

}

impl MockPgAdapter {
    pub fn new() -> MockPgAdapter {
        MockPgAdapter {}
    }
}

pub struct DbPort {
    adapter: Arc<dyn DbAdapter + Sync + Send>
}

impl DbPort {
    pub fn new(adapter: Arc<dyn DbAdapter + Send + Sync>) -> DbPort {
        DbPort { adapter }
    }
}
