pub struct Message {
    id: String,
    thread_id: String,
    author_id: String,
    content: String,
    created_at: String,
}

pub struct NewMessage {
    thread_id: String,
    author_id: String,
    content: String,
}
