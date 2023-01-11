use crate::*;
use near_sdk::serde_json;

#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
struct AccountInfo {
    display_name: Option<String>,
    avatar: Option<String>,
    thumbnail: Option<String>,

    about: Option<String>,
    occupation: Option<String>,

    profile_image: Option<String>,
    profile_video: Option<String>,

    email: Option<String>,
    location: Option<String>,

    twitter: Option<String>,
    github: Option<String>,
    telegram: Option<String>,
    linkedin: Option<String>,
    behance: Option<String>,
    website: Option<String>,
}

pub fn validate_account_data(base64: String) -> String {
    let bytes = base64::decode(base64).expect("Can not decode base 64");
    if let Ok(about_me) = serde_json::from_slice::<AccountInfo>(&bytes) {
        let json = serde_json::to_string(&about_me).expect("Can't encode data");
        return base64::encode(json.clone());
    } else {
        panic!("Can not extract data from bytes");
    }
}
