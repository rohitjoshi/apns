///APNS Payload format

//extern crate serialize;
extern crate "rustc-serialize" as rustc_serialize;
use std::collections::BTreeMap;
use self::rustc_serialize::json::{ Json, ToJson};
use self::rustc_serialize::{Decodable, Decoder};


///Alert Dictionary
#[derive(Show)]
pub struct AlertDictionary {
	body: String,
	action_loc_key: Option<String>,
	loc_key: Option<String>,
	loc_args: Option<Vec<String>>,
	launch_image: Option<String>,
}
/// Implement custom ToJson object
impl ToJson for AlertDictionary {
	fn to_json(&self) -> Json {
		
		let mut d = BTreeMap::new();
		d.insert("body".to_string(), self.body.to_json());

		if self.action_loc_key.is_some() {
			d.insert("action-loc-key".to_string(), self.action_loc_key.to_json());
		}
		if self.loc_key.is_some() {
			d.insert("loc-key".to_string(), self.loc_key.to_json());
		}
		if self.loc_args.is_some() {
			d.insert("loc-args".to_string(), self.loc_args.to_json());
		}
		if self.launch_image.is_some() {
			d.insert("launch-image".to_string(), self.launch_image.to_json());
		}
		Json::Object(d)
	}
}
impl Decodable for AlertDictionary {
	fn decode<T: Decoder>(decoder: &mut T) -> Result<AlertDictionary, T::Error> {
		decoder.read_struct("AlertDictionary", 5us,
			|_d| -> _
			Result::Ok(AlertDictionary{body:
				match _d.read_struct_field("body", 0us, Decodable::decode)
				{
					Ok(data) => data,
					Err(data) => return Err(data),
				},
				action_loc_key:
				match _d.read_struct_field("action-loc-key", 1us, Decodable::decode)
				{
					Ok(data) => data,
					Err(data) => return Err(data),
				},
				loc_key:
				match _d.read_struct_field("loc-key", 2us, Decodable::decode)
				{
					Ok(data) => data,
					Err(data) => return Err(data),
				},
				loc_args:
				match _d.read_struct_field("loc-args", 3us, Decodable::decode)
				{
					Ok(data) => data,
					Err(data) => return Err(data),
				},
				launch_image:
				match _d.read_struct_field("launch-image", 4us, Decodable::decode)
				{
					Ok(data) => data,
					Err(data) => return Err(data),
				},}))
	}
}


/// APNS message struct.  JSON format
/// APNS Message

#[derive(Show)]
pub struct ApnsMessage {
//	alert_body : Option<String>,
alert_dict : Option<AlertDictionary>,
badge: Option<u64>,
sound: Option<String>,
content_available: Option<u64>,
}
/// Implement custom ToJson trait
impl ToJson for ApnsMessage {
	fn to_json(&self) -> Json {
		let mut d = BTreeMap::new();
		/*if(self.alert_body.is_some()) {
			d.insert("alert".to_string(), self.alert_body.to_json());

		}else*/
		if(self.alert_dict.is_some()) {
			d.insert("alert".to_string(), self.alert_dict.to_json());
		}

		if(self.badge.is_some()) {
			d.insert("badge".to_string(), self.badge.to_json());
		}
		if(self.sound.is_some()) {
			d.insert("sound".to_string(), self.sound.to_json());
		}
		if(self.content_available.is_some()) {
			d.insert("content-available".to_string(), self.content_available.to_json());
		}

		Json::Object(d)
	}
}
impl Decodable for ApnsMessage {
	fn decode<T: Decoder>(decoder: &mut T)
	-> Result<ApnsMessage, T::Error> {
		decoder.read_struct("ApnsMessage", 4us,
			|_d| -> _
			Result::Ok(ApnsMessage{
			/*	alert_body:
				match _d.read_struct_field("alert",
					0us,
					Decodable::decode)
				{
					Ok(data) => data,
					Err(data) => return Err(data),
				},*/
				alert_dict:
				match _d.read_struct_field("alert",
					0us,
					Decodable::decode)
				{
					Ok(data) => data,
					Err(data) => return Err(data),
				},
				badge:
				match _d.read_struct_field("badge",
					1us,
					Decodable::decode)
				{
					Ok(data) => data,
					Err(data) => return Err(data),
				},
				sound:
				match _d.read_struct_field("sound",
					2us,
					Decodable::decode)
				{
					Ok(data) => data,
					Err(data) => return Err(data),
				},
				content_available:
				match _d.read_struct_field("content-available",
					3us,
					Decodable::decode)
				{
					Ok(data) => data,
					Err(data) => return Err(data),
				},}))
	}
}

pub struct PushNotification {
	id: u32,
	expiry: u32,
	devicetoken: String,
	payload: Vec<ApnsMessage>,
	priority: u8,
}
/// Implement custom ToJson trait
impl ToJson for PushNotification {
	fn to_json(&self) -> Json {
		let mut d = BTreeMap::new();
		d.insert("id".to_string(), self.id.to_json());
		d.insert("expiry".to_string(), self.expiry.to_json());
		d.insert("devicetoken".to_string(), self.devicetoken.to_json());
		d.insert("payload".to_string(), self.payload.to_json());
		d.insert("priority".to_string(), self.priority.to_json());
		Json::Object(d)
	}
}
///
impl Decodable for PushNotification {
	fn decode<T: Decoder>(decoder: &mut T)
	-> Result<PushNotification, T::Error> {
		decoder.read_struct("PushNotification", 5us,
			|_d| -> _
			Result::Ok(PushNotification{id:
				match _d.read_struct_field("id",
					0us,
					Decodable::decode)
				{
					Ok(data) => data,
					Err(data) => return Err(data),
				},
				expiry:
				match _d.read_struct_field("expiry",
					1us,
					Decodable::decode)
				{
					Ok(data) => data,
					Err(data) => return Err(data),
				},
				devicetoken:
				match _d.read_struct_field("devicetoken",
					2us,
					Decodable::decode)
				{
					Ok(data) => data,
					Err(data) => return Err(data),
				},
				payload:
				match _d.read_struct_field("payload",
					3us,
					Decodable::decode)
				{
					Ok(data) => data,
					Err(data) => return Err(data),
				},
				priority:
				match _d.read_struct_field("priority",
					4us,
					Decodable::decode)
				{
					Ok(data) => data,
					Err(data) => return Err(data),
				},}))
	}
}

#[cfg(test)]
#[allow(experimental)]
pub mod test {
 //	extern crate serialize;
 extern crate "rustc-serialize" as rustc_serialize;
	//extern crate log;
	use super::*;
	use self::rustc_serialize::json;
	use self::rustc_serialize::json::{Json, ToJson};
	use self::rustc_serialize::{Decodable, Decoder};
	use log;
	 //use self::rustc_serialize::{RustcDecodable};
	 #[test]
	 fn alert_dictionary_encode_decode() {
	 	let input_data = AlertDictionary {
	 		body: "test".to_string(),
	 		loc_key: Some("loc_key".to_string()),
	 		action_loc_key: Some("action_loc_key".to_string()),
	 		loc_args: Some(vec!["a".to_string(), "b".to_string()]),
			launch_image: None, //Some("launch_image".to_string()),
		};
		let json_obj: Json = input_data.to_json();
		let json_str: String = json_obj.to_string();
		info!("{}", json_str);
		let alert_dict: AlertDictionary = json::decode(json_str.as_slice()).unwrap();
		info!("{:?}", alert_dict);
		assert_eq!(alert_dict.body, "test");
		assert_eq!(alert_dict.loc_key.unwrap().as_slice(), "loc_key");
	//	let r = vec!["a".to_string(), "b".to_string()];
		//assert_eq!(alert_dict.loc_args.unwrap(),  r);
	}

	#[test]
	fn apns_encode_decode() {
		let mut aps_str= r#"
		{
			"alert":
			{
				"action-loc-key": "Open",
				"body": "Hello, world!"
			},
			"badge": 2
		}"#;

		let apns_message: ApnsMessage = json::decode(aps_str.as_slice()).unwrap();
		info!("APNS:{:?}", apns_message);
		assert_eq!(apns_message.badge.unwrap(), 2u64);

	}

	#[test]
	fn apns_encode_decode_1() {
		let mut aps_str= r#"
		{
			"alert": "test",
			"badge": 2
		}"#;

		let apns_message: ApnsMessage = json::decode(aps_str.as_slice()).unwrap();
		info!("APNS:{:?}", apns_message);
		assert_eq!(apns_message.badge.unwrap(), 2u64);

	}


}
