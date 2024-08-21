pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub enum Error {
    ClientError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ClientError(msg) => write!(f, "Client error: {}", msg),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::ClientError(_) => None,
        }
    }
}

pub type IClient = Box<dyn ClientTrait>;
pub type IValue = Box<dyn ValueTrait>;

pub struct DatabaseEntity {
    entity_id: String,
    entity_type: String,
    entity_name: String,
}

pub struct DatabaseField {
    entity_id: String,
    field: String,
    value: String,
    write_time: String,
    writer_id: String,
}

pub struct DatabaseNotification {
    token: String,
    current: DatabaseField,
    previous: DatabaseField,
    context: Vec<DatabaseField>
}

pub struct NotificationConfig {
    entity_id: String,
    field: String,
    notify_on_change: bool,
    context: Vec<String>
}

pub type NotificationToken = String;

pub trait ValueTrait {
    fn get<T>(&self) -> &T;
    fn set<T>(&mut self, value: T);
}

pub trait ClientTrait {
    fn get_entity(&self, entity_id: &str) -> Result<DatabaseEntity>;
    fn get_entities(&self, entity_type: &str) -> Result<Vec<DatabaseEntity>>;
    fn read(&self, requests: &mut Vec<DatabaseField>) -> Result<()>;
    fn write(&self, requests: &mut Vec<DatabaseField>) -> Result<()>;
    fn register_notification(&self, config: NotificationConfig) -> Result<NotificationToken>;
    fn unregister_notification(&self, token: NotificationToken) -> Result<()>;
    fn process_notifications(&self) -> Result<Vec<DatabaseNotification>>;
}

pub mod rest;
