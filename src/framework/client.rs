use std::cell::RefCell;
use std::rc::Rc;

use crate::clients::common::ClientTrait;
use crate::Result;
use crate::schema::entity::Entity;
use crate::schema::field::Field;
use crate::schema::notification::{Notification, Config, Token};

type ClientRef = Rc<RefCell<dyn ClientTrait>>;
pub struct Client(ClientRef);

impl Client {
    pub fn new(client: impl ClientTrait + 'static) -> Self {
        Client(Rc::new(RefCell::new(client)))
    }

    pub fn clone(&self) -> Self {
        Client(self.0.clone())
    }

    pub fn connect(&self) -> Result<()> {
        self.0.borrow_mut().connect()
    }

    pub fn connected(&self) -> bool {
        self.0.borrow().connected()
    }

    pub fn disconnect(&self) -> bool {
        self.0.borrow_mut().disconnect()
    }

    pub fn get_entities(&self, entity_type: &str) -> Result<Vec<Entity>> {
        self.0.borrow_mut().get_entities(entity_type)
    }

    pub fn get_entity(&self, entity_id: &str) -> Result<Entity> {
        self.0.borrow_mut().get_entity(entity_id)
    }

    pub fn get_notifications(&self) -> Result<Vec<Notification>> {
        self.0.borrow_mut().get_notifications()
    }

    pub fn read(&self, requests: &Vec<Field>) -> Result<()> {
        self.0.borrow_mut().read(requests)
    }

    pub fn register_notification(&self, config: &Config) -> Result<Token> {
        self.0.borrow_mut().register_notification(config)
    }

    pub fn unregister_notification(&self, token: &Token) -> Result<()> {
        self.0.borrow_mut().unregister_notification(token)
    }

    pub fn write(&self, requests: &Vec<Field>) -> Result<()> {
        self.0.borrow_mut().write(requests)
    }
}