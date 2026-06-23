// NOTE: We have import the trait while using them
use my_traits_lib::{SocialPost, Summary};
fn main() {
    let post = SocialPost {
        username: "horse_ebooks".to_string(),
        content: "Horses are very good!".to_string(),
        reply: false,
        repost: false,
    };

    println!("{}", post.summarize());
}
