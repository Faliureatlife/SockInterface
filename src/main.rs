#![allow(non_snake_case)]

use std::fs::*;
use std::io::prelude::*;
use std::os::unix::net::*;
use users::get_current_username;

fn establishSock(sockLocation: &str) -> Option<UnixStream> {
    let stream = UnixStream::connect(sockLocation).ok(); //fix the trashing of the Result
    return Some(stream?);
}

fn msgSock(mut stream: &UnixStream, msg: &str) -> std::io::Result<()> {
    let streamres = stream.write_all(msg.as_bytes());
    return streamres;
}

fn readSock(mut stream: &UnixStream) -> Option<String> {
    let mut responce = String::new();
    let readres = stream.read_to_string(&mut responce);
    match readres {
        Ok(_) => return Some(responce),
        Err(_) => return None,
    }
}

fn msgSockAndRead(mut stream: &UnixStream, msg: &str) -> Option<String> {
    let streamres = stream.write_all(msg.as_bytes());
    match streamres {
        Ok(_) => {}
        Err(_) => return None,
    }
    let mut responce = String::new();
    let readres = stream.read_to_string(&mut responce);
    match readres {
        Ok(_) => return Some(responce),
        Err(_) => return None,
    }
}

fn scriptInstallCheck(usr: String) {
    let filecheck = create_dir("/home/{}/.local/vlcenv");
    match filecheck {
        Ok(v) => {}
        Err(_) => scriptInstall(usr),
    }
}

fn scriptInstall(usr: String) {}

fn main() -> std::io::Result<()> {
    //creating the neccesary values
    //getting current user
    match get_current_username() {
        Some(uname) => scriptInstallCheck(uname.into_string().expect("Unable to convert into string")),
        None => eprintln!("Current user no longer exists"),
    }
    let inputting: bool = true;
    let mut usr_input = String::new();
    let stream = establishSock("/home/bob/personal/vlc.sock");
    let stdin = std::io::stdin();
    //main loop of the program
    while inputting != false {
        stdin.read_line(&mut usr_input)?;
        //check if user wants to exit env
        if usr_input == "q" {
            break;
        }
        //send message to the socket and return it to socket
        let responce = msgSockAndRead(&stream.unwrap(), &usr_input);
        //print out the responce from the socket
        println!("{}", responce.expect("Error returning socket repsonce"));
        //clear out the values from the user input
    }
    Ok(())
}













