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
//#[derive( RustcDecodable, Show)]
pub enum AlertPayloadType {
	StrAlert(String),
	DictAlert(AlertDictionary),
}
impl ::rustc_serialize::Decodable for AlertPayloadType {
	fn decode<__D: ::rustc_serialize::Decoder>(__arg_0: &mut __D)
	-> ::std::result::Result<AlertPayloadType, __D::Error> {
       println!("::rustc_serialize::Decodable for AlertPayloadType::decode");
		__arg_0.read_enum("AlertPayloadType",
			|_d| -> _
				::std::result::Result::Ok( 	

				//match ApnsMessage::to_alert_payload_type(_d.read_str())				
					match _d.read_enum_variant_arg(0us, ::rustc_serialize::Decodable::decode)
					{
						Ok(__try_var) => {
							println!("APNS:{:?}", __try_var);
							//__try_var
							ApnsMessage::to_alert_payload_type(&__try_var).unwrap()
							
							/*let a = __try_var as Object;
							if (a.is_string()) {
							 	return Ok(AlertPayloadType::StrAlert(a));
							 }else {
							 	return Ok(AlertPayloadType::DictAlert(a));
							 }*/
							/*match __try_var  {
								AlertPayloadType::StrAlert(tt) => {
									return Ok(AlertPayloadType::StrAlert(tt));
								},
								AlertPayloadType::DictAlert(tt) => {
									return Ok(AlertPayloadType::DictAlert(tt));
								},

								
							};*/
							 

						},
						Err(__try_var) => {
							//println!("AlertPayloadType::decode:{:?}", __try_var);
							return Err(__try_var);
						},
					}
				)
					/*
					AlertPayloadType::DictAlert(match _d.read_enum_variant_arg(0us,
						::rustc_serialize::Decodable::decode)
					{
						Ok(__try_var)
						=>
						__try_var,
						Err(__try_var)
						=>
						return Err(__try_var),
					}),*/
					
				)
	}
}
/// APNS message struct.  JSON format
/// APNS Message



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
		println!("to_alert_payload_type: {}", json_str);
		let mut alert_payload : Option<AlertPayloadType> = None;
		//if it is string value e.g "test"  than return
        if json_str.as_slice().contains_char('{') == false {
        	println!("to_alert_payload_type: {} is string", json_str);
          return Ok(AlertPayloadType::StrAlert(json_str.clone()));
        }
        println!("to_alert_payload_type: {}  is a object", json_str);
		let mut json_result = Json::from_str(json_str.as_slice());
		if json_result.is_err() {
			info!("Failed to convert from json_str {}", json_str );
			let e : Result<AlertPayloadType, BuilderError> = Err(json_result.err().unwrap());
			return e;
		}
		let json_result = json_result.unwrap();
		
		if json_result.is_string() {
 			info!("Found alert object as string: {:?}", json_result.as_string().unwrap());
 			alert_payload = Some(AlertPayloadType::StrAlert(json_result.as_string().unwrap().to_string()));
 		}else {
 			//println!("Found alert as object : {:?}", alert_object.unwrap().as_object().unwrap());
 			//let ao = json_result;
 			let mut decoder = rustc_serialize::json::Decoder::new(json_result.clone());
 			let alert_dict_object: AlertDictionary = match Decodable::decode(&mut decoder) {
 				Ok(v) => v,
 				Err(e) => {
 					return Err(ParserError::IoError(IoErrorKind::OtherIoError, 
 						"Failed to convert to AlertDictionary object")); }
 				}; 
 				alert_payload = Some(AlertPayloadType::DictAlert(alert_dict_object));

 		}
 		Ok(alert_payload.unwrap())

	}

	pub fn from_str(json_str: &String) -> Result<ApnsMessage, BuilderError> {

		let mut json_result = Json::from_str(json_str.as_slice());
		if json_result.is_err() {
			println!("Failed to convert from json_str {}", json_str );
			let e : Result<ApnsMessage, BuilderError> = Err(json_result.err().unwrap());
			return e;
		}
		let json_result = json_result.unwrap();
		let  alert_object = json_result.find("alert");
 		//if alert key not found
 		if(alert_object.is_none()) {
 			return Err(ParserError::IoError(IoErrorKind::OtherIoError, 
 				"alert key not found"));
 		}
 		let mut alert_payload : Option<AlertPayloadType> = None;
 		if alert_object.unwrap().is_string() {
 			println!("Found alert object as string: {:?}", alert_object.unwrap().as_string().unwrap());
 			alert_payload = Some(AlertPayloadType::StrAlert(alert_object.unwrap().as_string().unwrap().to_string()));
 		}else {
 			//println!("Found alert as object : {:?}", alert_object.unwrap().as_object().unwrap());
 			let ao = alert_object.unwrap();
 			let mut decoder = rustc_serialize::json::Decoder::new(ao.clone());
 			let alert_dict_object: AlertDictionary = match Decodable::decode(&mut decoder) {
 				Ok(v) => v,
 				Err(e) => {
 					return Err(ParserError::IoError(IoErrorKind::OtherIoError, 
 						"Failed to convert to AlertDictionary object")); }
 				}; 
 				alert_payload = Some(AlertPayloadType::DictAlert(alert_dict_object));

 		}
 		Ok(ApnsMessage {
 			alert_payload : alert_payload.unwrap(),
 			badge : None,
 			sound: None,
 			content_available: None


 		})


 	}

 	fn convert_to_alert_payload_type(payload: AlertPayloadType) -> AlertPayloadType
	{
		AlertPayloadType::StrAlert("test".to_string())
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

				//ApnsMessage::convert_to_alert_payload_type(match _d.read_struct_field( "alert",
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
	/*
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
	}*/

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
		//println!("apns_encode_decode:{:?}", apns_message);
		//assert_eq!(apns_message.badge.unwrap(), 2u64);

	}
/*
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

	}*/


}
