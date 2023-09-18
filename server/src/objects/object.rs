use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct Channel {
    pub name: String, // uuid
    pub token: String,
    pub title: String,
    pub owner_name: String, // display name
    pub owner_icon: String, // image url
}

#[derive(SimpleObject)]
pub struct Comment {
    pub id: i32, // auto increment
    pub body: String,
    pub channel: String, // uuid
    pub owner: String, // display name
}
