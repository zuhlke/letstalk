#![feature(phase)]
/** ************************************************************************************************
 * \file      letstalk.rs
 * \ingroup   Let's_Talk!
 * \brief     
 * \details   
 * \author    Jeremy Hannon <hje@zuhlke.com>
 * \date      29th September 2014
 **************************************************************************************************/

#[phase(plugin, link)] extern crate log;

extern crate serialize;             // TODO: why is this required here?  Used in file_io file.
use common::message;
use std::io::TcpStream;
use std::os;
pub mod file_io;
pub mod client_information;
#[path = "../common"]
mod common {pub mod message;}

static USER_INFO_FILENAME: &'static str = "userInfo.json";
static FRIEND_LIST_FILENAME: &'static str = "friendList.json";

fn main() {
	println!("Let's Talk!");
	
    let args = os::args();
    if args.len() < 2 {
        println!("use: letstalk server_addr");
    }
    let addr = args[1].as_slice();
	
	// Open socket to the server.
    let mut socket = TcpStream::connect(addr, 7777).unwrap();
	
	// read client info, such as my nickname
    let result = file_io::read_friends_from_file(USER_INFO_FILENAME);
	let mut stored_user_info = match result {
        Ok(x)  => x, 
        Err(e) => {
            error!("read_friends_from_file() returned Err({}). What should we do?", e);
            Vec::new()
        }
    };
    for n in range(0u, stored_user_info.len()) {
        println!("User Info contains: {}", stored_user_info.get(n).friend_nickname);
    }
    
    // send my nickname to the server
    let temp_user_info = stored_user_info.pop();
    if temp_user_info.is_some() {
        let register_msg = message::SignIn(message::SignInMessage {
                        user_name: temp_user_info.unwrap().friend_nickname});
        // send the message to the server
        let _ = socket.write(register_msg.convert_to_json().into_bytes().as_slice());
    }
	
    // read the stored friends list to know which friends to request from the server.
    let result = file_io::read_friends_from_file(FRIEND_LIST_FILENAME);
    let mut stored_friend_info = match result {
        Ok(x)  => x, 
        Err(e) => {
	        error!("read_friends_from_file() returned Err({}). What should we do?", e);
	        Vec::new()
        }
    };

    // spawn a thread to listen for server responses
    let mut server_friend_info: Vec<message::AddressResponseMessage> = Vec::new();
//    handle_server_friend_info_responses(&server_friend_info);

    // send list of friends to the server to request their IP addresses.
    for n in range(0u, stored_friend_info.len()) {
        println!("Friend list contains: {}", stored_friend_info.get(n).friend_nickname);
        let temp_friend_info = stored_friend_info.pop();
        if temp_friend_info.is_some() {
	        let address_request_msg = message::AddressRequest(message::AddressRequestMessage {
	                           user_name: temp_friend_info.unwrap().friend_nickname});
	        // send the message to the server
	        match socket.write(address_request_msg.convert_to_json().into_bytes().as_slice()) {
                Err(e) => error!("couldn't send friend info to server: {}", e),
		        Ok(_) => {}
		    }
	    }
    }
    
    
    println!("Let's Talk again some time ;)");
}

//fn handle_server_friend_info_responses(server_friend_info: &Vec) {
//}
