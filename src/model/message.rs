pub struct Message {
    pub id: Uuid,
    pub user: User,
    pub body: String,
    pub timestamp: DateTime<Utc>,
}

impl Message {
    pub fn new(id: Uuid, user: User, body: &str, timestamp: DateTime<Utc>) -> Self {
        Message {
            id,
            user,
            body: String::from(body),
            timestamp,
        }
    }
}
