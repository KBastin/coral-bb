pub struct SubForum {
    id: u32,
    name: String,
    description: String,
    parent_sub_forum_id: Option<u32>,
}

pub struct NewSubForum {
    name: String,
    description: String,
    parent_sub_forum_id: Option<u32>,
}