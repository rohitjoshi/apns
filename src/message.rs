///APNS Payload format

//extern crate serialize;
extern crate "rustc-serialize" as rustc_serialize;
use std::collections::BTreeMap;
use self::rustc_serialize::json::{ DecodeResult, Json,  ToJson, BuilderError, ParserError};
use self::rustc_serialize::json::Object;
use self::rustc_serialize::json::DecoderError::{ MissingFieldError, ExpectedError};
use self::rustc_serialize::{Decodable, Decoder}; 
use std::io::{ IoErrorKind};
use log;

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

///Alert payoad type
#[derive(  Show)]
pub enum AlertPayloadType {
	StrAlert(String),
	DictAlert(AlertDictionary),
}

///Decodable trait implementation for AlertPayloadType
impl ::rustc_serialize::Decodable for AlertPayloadType {
	fn decode<__D: ::rustc_serialize::Decoder>(__arg_0: &mut __D)
	-> ::std::result::Result<AlertPayloadType, __D::Error> {
		__arg_0.read_enum("AlertPayloadType",
			|_d| -> _
				::std::result::Result::Ok( 				
					match _d.read_enum_variant_arg(0us, ::rustc_serialize::Decodable::decode)
					{
						Ok(__try_var) => {
							ApnsMessage::to_alert_payload_type(&__try_var).unwrap()
						},
						Err(__try_var) => {
							return Err(__try_var);
						},
					}
				)
			)
	}
}


#[derive(  Show)]
pub struct ApnsMessage {
	alert_payload : AlertPayloadType,
	badge: Option<u64>,
	sound: Option<String>,
	content_available: Option<u64>,
}
impl ApnsMessage {
	pub fn new(alert: AlertPayloadType, badge: Option<u64>, sound: Option<String>, content_available: Option<u64>) -> ApnsMessage{
		ApnsMessage {
			alert_payload : alert,
			badge: badge,
			sound: sound,
			content_available: content_available,
		}
	}

	pub fn to_alert_payload_type(json_str: &String) -> Result<AlertPayloadType, BuilderError> {
		let mut alert_payload : Option<AlertPayloadType> = None;
		//if it is string value e.g "test"  than return
        if json_str.as_slice().contains_char('{') == false {
          return Ok(AlertPayloadType::StrAlert(json_str.clone()));
        }
        
		let mut json_result = Json::from_str(json_str.as_slice());
		if json_result.is_err() {
			let e : Result<AlertPayloadType, BuilderError> = Err(json_result.err().unwrap());
			return e;
		}
		let json_result = json_result.unwrap();
		if json_result.is_string() {
 			return Ok(AlertPayloadType::StrAlert(json_result.as_string().unwrap().to_string()));
 		}else {
 			let mut decoder = rustc_serialize::json::Decoder::new(json_result.clone());
 			let alert_dict_object: AlertDictionary = match Decodable::decode(&mut decoder) {
 				Ok(v) => v,
 				Err(e) => {
 					return Err(ParserError::IoError(IoErrorKind::OtherIoError, 
 						"Failed to convert to AlertDictionary object")); }
 				}; 
 				return Ok(AlertPayloadType::DictAlert(alert_dict_object));

 		}

	}
 }


/// Implement custom ToJson trait
impl ToJson for ApnsMessage {
	fn to_json(&self) -> Json {
		let mut d = BTreeMap::new();
		match self.alert_payload {
			AlertPayloadType::StrAlert(ref data) => {
				d.insert("alert".to_string(), data.to_json());
			},
			AlertPayloadType::DictAlert(ref data) => {
				d.insert("alert".to_string(), data.to_json());
			}
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
				Result::Ok(ApnsMessage{alert_payload:
				match _d.read_struct_field( "alert",
					0us,
					Decodable::decode)
				{
					Ok(data) => {
						//panic!("JOSHI:alert_payload type: ");
						data
					},
					Err(data) => {
						//panic!("JOSHI:alert_payload type: ");
						return Err(data);
					},
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


/// Push Notification 
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
				"body": "Hello, world!",
				"action-loc-key": "Open"
				
			},
			"badge": 2
		}"#;

        
		let apns_message: ApnsMessage = match json::decode(aps_str.as_slice()) {
			Ok(v) => v,
			Err(e) => { 
				panic!("Failed to convert {:?}", e);
			}
		};
		println!("apns_encode_decode:{:?}", apns_message);
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
		info!("apns_encode_decode_1:{:?}", apns_message);
		assert_eq!(apns_message.badge.unwrap(), 2u64);

	}


}
