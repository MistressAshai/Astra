use astra_derive::{Astra, AstraBook};
use astra_formats::indexmap::IndexMap;
use astra_formats::Sheet;

#[derive(AstraBook)]
pub struct FriendListBook {
    pub friend_list_data: Sheet<IndexMap<String, FriendListData>>,
}

#[derive(Debug, Default, Clone, Astra)]
pub struct FriendListData {
    #[astra(key = "@Out")]
    pub out: String,
    #[astra(key = "@FLID", id)]
    pub flid: String,
    #[astra(key = "@PID")]
    pub pid: String,
    #[astra(key = "@Level")]
    pub level: i8,
    #[astra(key = "@ContentText")]
    pub content_text: String,
    #[astra(key = "@StampName")]
    pub stamp_name: String,
    #[astra(key = "@ImageName")]
    pub image_name: String,
    #[astra(key = "@ImageNameS")]
    pub image_name_s: String,
    #[astra(key = "@Country")]
    pub country: i8,
}
