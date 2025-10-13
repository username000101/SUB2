use crate::td::decl::{
    td_json_client_create, td_json_client_destroy, td_json_client_execute, td_json_client_receive,
    td_json_client_send,
};

use serde_json::{Value, json};
use std::ffi::{CStr, CString, c_char, c_void};
use tracing::{debug, info, warn};

#[derive(Clone)]
pub struct ClientResponseRequest {
    response: &'static CStr,
}

struct ClientPtrThreadWrapper {
    pub ptr: *mut c_void,
}

impl ClientPtrThreadWrapper {
    pub fn new(ptr: *mut c_void) -> ClientPtrThreadWrapper {
        ClientPtrThreadWrapper { ptr }
    }
}

unsafe impl Send for ClientPtrThreadWrapper {}

impl ClientResponseRequest {
    pub fn new(response: *const c_char) -> ClientResponseRequest {
        unsafe {
            ClientResponseRequest {
                response: CStr::from_ptr(response),
            }
        }
    }

    pub fn parse_as<T: serde::de::DeserializeOwned + serde::Serialize>(
        &self,
    ) -> Result<T, Box<dyn std::error::Error>> {
        use serde_json::from_str;
        use std::any::type_name;

        if self.response.is_empty() {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "Failed to parse response as {}: response is empty",
                    type_name::<T>()
                ),
            ))
            .unwrap()
        }

        Ok(from_str::<T>(self.response.to_str().unwrap()).unwrap())
    }

    pub fn is_valid(&self) -> bool {
        if self.response.is_empty() {
            false
        } else {
            true
        }
    }

    pub fn get_type(&self) -> String {
        let value: serde_json::Value =
            serde_json::from_str(self.response.to_str().unwrap()).unwrap();
        if let Some(val) = value.get("@type") {
            val.as_str().unwrap().to_string()
        } else {
            "".to_string()
        }
    }
    
    pub fn get_error(&mut self) -> Option<String> {
        if self.get_type() == "error".to_string() {
            Some(String::from_str(self.response.to_str().unwrap()).unwrap())
        } else { None }
    }
    
    pub fn get_raw(&mut self) -> String {
        String::from_str(self.response.to_str().unwrap()).unwrap()
    }
    
    pub fn get_field(&mut self, field: String) -> Option<Value> {
        let value: Value = serde_json::from_str(self.response.to_str().unwrap()).unwrap();
        if let Some(val) = value.get(field) {
            Some(val.clone())
        } else {
            None
        }
    }

    pub fn get_extra(&mut self) -> Option<String> {
        let value: serde_json::Value =
            serde_json::from_str(self.response.to_str().unwrap()).unwrap();
        if let Some(extra) = value.get("@extra") {
            Some(extra.as_str().unwrap().to_string())
        } else {
            None
        }
    }
}

use crate::td;
use parking_lot::FairMutex;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use std::vec::Vec;

pub struct Client {
    client: *mut c_void,
    client_state: bool,
    updates: Arc<FairMutex<Vec<CString>>>,
    responses: Arc<FairMutex<HashMap<String, ClientResponseRequest>>>,
}

impl Client {
    pub fn new() -> Client {
        unsafe {
            Client {
                client: td_json_client_create(),
                client_state: true,
                updates: Arc::new(FairMutex::new(Vec::new())),
                responses: Arc::new(FairMutex::new(HashMap::new())),
            }
        }
    }

    pub fn execute(&self, request: Value) -> ClientResponseRequest {
        if !self.client_state {
            warn!(
                "Failed to execute request '{}': client_state == false",
                request.to_string()
            );
            return ClientResponseRequest::new(std::ptr::null());
        }

        if request.to_string().is_empty() {
            warn!("Empty request");
            return ClientResponseRequest::new(std::ptr::null());
        }

        let request_str = request.to_string();
        unsafe {
            let request_cstring = CString::new(request_str).unwrap();
            ClientResponseRequest::new(td_json_client_execute(
                self.client,
                request_cstring.as_ptr(),
            ))
        }
    }

    pub fn get_update(&mut self) -> String {
        for i in [1, 2, 3, 4, 5] {
            {
                let mut lock = self.updates.lock();
                if !lock.is_empty() {
                    let str = lock.pop().unwrap();
                    return str.into_string().unwrap()
                }
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        "".to_string()
    }

    pub fn send(&mut self, mut request: Value, extra: String) -> ClientResponseRequest {
        use std::thread::sleep;
        use std::time::Duration;

        if !self.client_state {
            warn!(
                "Failed to send request '{}': client_state == false",
                request.to_string()
            );
            return ClientResponseRequest::new(std::ptr::null());
        }

        let result: ClientResponseRequest;
        let request_map = request.as_object_mut().unwrap();
        request_map.insert("@extra".to_string(), json!(extra));
        let request_str = request.to_string();
        unsafe {
            let request_cstring = CString::new(request_str).unwrap();
            debug!(
                "Sending request '{}'",
                request.to_string(),
            );            
            td_json_client_send(self.client, request_cstring.as_ptr());
        }
        loop {
            sleep(Duration::from_millis(50));
            if self.responses.lock().is_empty() {
                continue;
            }

            let mut lock = self.responses.lock();
            if lock.contains_key(&extra) {
                result = lock.remove(&extra).unwrap();
            } else {
                continue;
            }

            return result.clone();
        }
    }

    pub fn start_updates_loop(&mut self) {
        let responses_lock = self.responses.clone();
        let updates_lock = self.updates.clone();
        let client = ClientPtrThreadWrapper::new(self.client);
        std::thread::spawn(move || {
            updates_loop(client, responses_lock, updates_lock);
        });
    }

    pub fn destroy(&mut self) {
        self.send(td::requests::close(), "__client_destroy".to_string());
        self.client_state = false;
        unsafe {
            td_json_client_destroy(self.client);
        }
    }
}

impl std::fmt::Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Client{{client: {:p} }}",
            self.client,
        )
    }
}

fn updates_loop(
    client: ClientPtrThreadWrapper,
    responses: Arc<FairMutex<HashMap<String, ClientResponseRequest>>>,
    updates: Arc<FairMutex<Vec<CString>>>,
) {
    use serde_json::{Value, from_str};

    info!("Starting updates loop");

    loop {
        std::thread::sleep(Duration::from_millis(50));
        unsafe {
            let upd = td_json_client_receive(client.ptr, 3.0 as std::ffi::c_double);
            if upd.is_null() {
                debug!("Update is null");
                continue;
            } else {
                let cstr = CStr::from_ptr(upd);
                let mut update_value: Value = from_str(cstr.to_str().unwrap()).unwrap();
                if let Some(update) = update_value.as_object_mut() {
                    if update.contains_key("@extra") {
                        debug!(
                            "Received response: {}",
                            cstr.to_str().unwrap()
                        );
                        let extra = {
                            let extra = update.get_mut("@extra").unwrap().as_str().unwrap();
                            extra.to_string()
                        };
                        responses
                            .lock()
                            .insert(extra, ClientResponseRequest::new(upd));
                    } else {
                        debug!("Received update: {}", CStr::from_ptr(upd).to_str().unwrap());
                        let upd_own = cstr.to_owned();
                        updates
                            .lock()
                            .push(upd_own);
                    }
                }
            }
        }
    }
}

unsafe impl Send for Client {}
unsafe impl Sync for Client {}

use once_cell::sync::Lazy;
pub static CLIENT: Lazy<FairMutex<Client>> = Lazy::new(|| FairMutex::new(Client::new()));
