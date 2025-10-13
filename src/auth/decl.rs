use std::io::Write;
use tracing::{info, warn};
use crate::td;
use crate::logger;
use crate::td::interface::ClientResponseRequest;

fn get_authorization_state() -> String {
    let mut lock = td::interface::CLIENT.lock();
    let current_auth_state = lock.send(td::requests::getAuthorizationState(), "get_auth_state".to_string());
    current_auth_state.get_type()
}

fn get_authorization_state_raw() -> ClientResponseRequest {
    let mut lock = td::interface::CLIENT.lock();
    lock.send(td::requests::getAuthorizationState(), "get_auth_state".to_string())
}

pub fn sub2_tdlib_auth() {
    let mut password_hint = String::new();
    
    {
        loop {
            let auth_state = get_authorization_state();
            match auth_state.as_str() {
                "authorizationStateWaitTdlibParameters" => {
                    let mut lock = td::interface::CLIENT.lock();
                    let mut set_tdlib_params = lock.send(td::requests::setTdlibParameters(), "set_tdlib_params".to_string());
                    match set_tdlib_params.get_type().as_str() {
                        "error" => panic!("Failed to set tdlib parameters: {}", set_tdlib_params.get_error().unwrap()),
                        "ok" => {}
                        _ => {
                            warn!("Failed to set tdlib parameters: unexpected response: {}", set_tdlib_params.get_type());
                            return;
                        }
                    }
                }
                "authorizationStateWaitPhoneNumber" => break,
                "authorizationStateReady" => {
                    let mut lock = td::interface::CLIENT.lock();
                    let mut me = lock.send(td::requests::getMe(), "get_me".to_string());
                    info!("You're already logged in! Welcome, {}!", me.get_field("first_name".to_string()).unwrap().as_str().unwrap());
                    return;
                }
                _ => panic!("Unexpected state: {}", auth_state)
            }
        }
    }

    {
        let mut phone_number = String::new();
        logger::log::disable_logging_for(|| {
            let mut stdout = std::io::stdout();
            loop {
                crossterm::execute!(stdout, crossterm::terminal::Clear(crossterm::terminal::ClearType::CurrentLine)).unwrap();
                print!("Enter phone number: ");
                stdout.flush().unwrap();
                std::io::stdin().read_line(&mut phone_number).unwrap();
                phone_number = phone_number.trim_end().to_string();
                println!("Phone number is '{}', correct?", phone_number);
                stdout.flush().unwrap();
                let mut confirmation = String::new();
                std::io::stdin().read_line(&mut confirmation).unwrap();
                if confirmation.starts_with('y') || confirmation.starts_with('Y') {
                    break;
                } else {
                    phone_number.clear();
                }
            }
        });

        let mut lock = td::interface::CLIENT.lock();
        let mut result = lock.send(td::requests::setAuthenticationPhoneNumber(phone_number), "set_auth_phone_number".to_string());
        match result.get_type().as_str() {
            "ok" => {},
            "error" => {
                panic!("Failed to set authentication phone number: {}", result.get_error().unwrap());
            },
            _ => panic!("Unexpected state(setAuthenticationPhoneNumber): {}", result.get_type())
        }
    }

    {
        let auth_state = get_authorization_state();
        match auth_state.as_str() {
            "authorizationStateWaitCode" => {},
            _ => panic!("Unexpected state(get_auth_state_after_set_auth_phone_number): {}", auth_state)
        }
    }

    {
        let mut code = String::new();
        logger::log::disable_logging_for(|| {
            let mut stdout = std::io::stdout();
            loop {
                print!("Enter code: ");
                stdout.flush().unwrap();
                std::io::stdin().read_line(&mut code).unwrap();
                code = code.trim_end().to_string();
                println!("Code is '{}', correct?", code);
                stdout.flush().unwrap();
                let mut confirmation = String::new();
                std::io::stdin().read_line(&mut confirmation).unwrap();
                if confirmation.starts_with('y') || confirmation.starts_with('Y') {
                    break;
                } else {
                    code.clear();
                }
            }
        });

        let mut lock = td::interface::CLIENT.lock();
        let mut result = lock.send(td::requests::checkAuthenticationCode(code), "check_auth_code".to_string());
        match result.get_type().as_str() {
            "ok" => {},
            "error" => {
                panic!("Failed to set authentication phone number: {}", result.get_error().unwrap());
            },
            _ => panic!("Unexpected state(checkAuthenticationCode): {}", result.get_type())
        }
    }

    {
        let mut auth_state = get_authorization_state_raw();
        match auth_state.get_type().as_str() {
            "authorizationStateReady" => {
                let mut lock = td::interface::CLIENT.lock();
                let mut me = lock.send(td::requests::getMe(), "get_me".to_string());
                info!("Successfully logged in! JSON info: \n{}", me.get_raw());
                return;
            },
            "authorizationStateWaitPassword" => {
                if let Some(hint) = auth_state.get_field("password_hint".to_string()) {
                    password_hint = hint.as_str().unwrap().to_string().clone();
                }
            },
            _ => panic!("Unexpected state(get_auth_state_after_check_auth_code): {}", auth_state.get_type())
        }
    }

    {
        let mut password  = String::new();
        logger::log::disable_logging_for(|| {
            let mut stdout = std::io::stdout();
            loop {                
                crossterm::execute!(stdout, crossterm::terminal::Clear(crossterm::terminal::ClearType::CurrentLine)).unwrap();
                print!("Enter password(hint: '{}'): ", if password_hint.is_empty() { "SUB2_EMPTY_PASSWORD_HINT" } else { password_hint.as_str() });
                stdout.flush().unwrap();
                std::io::stdin().read_line(&mut password).unwrap();
                password = password.trim_end().to_string();
                println!("Password is '{}', correct?", password);
                stdout.flush().unwrap();
                let mut confirmation = String::new();
                std::io::stdin().read_line(&mut confirmation).unwrap();
                if confirmation.starts_with('y') || confirmation.starts_with('Y') {
                    break;
                } else {
                    password.clear();
                }
            }
        });
        
        let mut lock = td::interface::CLIENT.lock();
        let mut result = lock.send(td::requests::checkAuthenticationPassword(password), "set_auth_password".to_string());
        match result.get_type().as_str() {
            "ok" => {},
            "error" => { panic!("Failed to set authentication password: {}", result.get_error().unwrap()); },
            _ => panic!("Unexpected state(checkAuthenticationPassword): {}", result.get_type())
        }
    }

    {
        let auth_state = get_authorization_state();
        match auth_state.as_str() {
            "authorizationStateReady" => {
                let mut lock = td::interface::CLIENT.lock();
                let mut me = lock.send(td::requests::getMe(), "get_me".to_string());
                info!("You're already logged in! Welcome, {}!", me.get_field("first_name".to_string()).unwrap().as_str().unwrap());
                return;
            },
            _ => panic!("Unexpected state(get_auth_state_after_check_auth_password): {}", auth_state)
        }
    }
}