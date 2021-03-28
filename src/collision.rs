pub struct Contact(ContactID, ContactID);

impl Contact {
    pub fn get_ids(&self) -> (ContactID, ContactID) {
        (self.0, self.1)
    }
}

#[derive(Copy, Clone)]
pub enum ContactID {
    Barrier,
    Player,
}

// pub fn gather_contacts(player:, Barrier:) -> Vec<Contact> {}
    

