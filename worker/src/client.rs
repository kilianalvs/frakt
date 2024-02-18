// use worker::
// use shared::messages_types::{};

pub mod client{
    
    use std::{default, io::{Read, Write}};
    use std::net::{TcpListener, TcpStream};

    use hostname::get;
    use serde::Serialize;
    use serde_json::{json, Value};
    use shared::messages_types::{FragmentRequest, FragmentResult, FragmentTask};
    use debug_print::{debug_println}

    use crate::message::Message;
    

    //main client function
    fn handle_client(mut stream : TcpStream){
        let mut buffer = [0;1024];  //to read data from the client
        stream.read(&mut buffer)//read data from the stream and store it in the buffer
            .expect("Failed to read from client !");

        let request = String::from_utf8_lossy(&buffer[..]); //buffer => utf8 encoded string
        debug_println!("Received Request : {}", request); //debug only
        let response = "hello client".as_bytes();
        stream.write(response).expect("Failed to write response!");
    }

    //entry point
    fn main(){
        let target_adress: &str = "127.0.0.1:16142";
        let listener = TcpListener::bind(target_adress).expect("failed to bind to addresse");
        debug_println!("server is listening on {}", target_adress);

        for stream in listener.incoming(){
            match stream {
                Ok(stream) =>{
                    std::thread::spawn(|| handle_client(stream));
                }
                Err(e) =>{
                    eprintln!("failed to established connexion: {}",e);
                    //stderr - standart error stream
                }
            }
        }
    }


    // Fonction pour envoyer un message JSON sur le flux TCP
    fn send_message(stream: &TcpStream, message: &dyn Message) {
        match message.serialize() {
            Ok(serialized_message) => {
                let message_str = serde_json::to_string(&serialized_message)
                    .expect("Unable to serialize message");
                let mut stream_clone = stream.try_clone()
                    .expect("Unable to clone TcpStream");
                writeln!(stream_clone, "{}", message_str)
                    .expect("Failed to send message");
            }
            Err(err) => {
                // Gérer l'erreur de sérialisation ici
                panic!("Error serializing message: {:?}", err);
            }
            default =>{
                panic!("Something wrong happed: {:?}", default);
            }
        }
    }

    // Fonction pour recevoir un message JSON du flux TCP
    fn receive_message(stream: &TcpStream) -> Box<dyn Message> {
        let mut buffer = String::new();
        let mut stream_clone = stream.clone();
        stream_clone.read_to_string(&mut buffer).expect("Failed to read from stream");

        // Désérialiser le message JSON en fonction de son type
        if buffer.contains("FragmentRequest") {
            Box::new(serde_json::from_str::<FragmentRequest>(&buffer).expect("Failed to deserialize FragmentRequest"))
        } else if buffer.contains("FragmentResult") {
            Box::new(serde_json::from_str::<FragmentResult>(&buffer).expect("Failed to deserialize FragmentResult"))
        } else if buffer.contains("FragmentTask") {
            Box::new(serde_json::from_str::<FragmentTask>(&buffer).expect("Failed to deserialize FragmentTask"))
        } else {
            panic!("Unknown message type");
        }
    }

    pub fn receive_fragment_task(stream: &TcpStream) -> Result<FragmentTask, Box<dyn std::error::Error>> {
        let mut buffer: String = String::new();
        let mut stream_clone: TcpStream = stream.try_clone()?;
        stream_clone.read_to_string(&mut buffer)?;

        if buffer.contains("FragmentTask") {
            let fragment_task: FragmentTask = serde_json::from_str(&buffer)?;
            Ok(fragment_task)
        } else {
            Err("Received message is not a FragmentTask".into())
        }
    }

    // Fonction pour calculer un FragmentResult à partir d'une tâche de fragment
    fn calculate_fragment_result(fragment_task: &FragmentRequest) -> FragmentResult {
        todo!();

        // generate
        // json!({
        //     "FragmentResult": {
        //         // Vos données calculées ici
        //     }
        // })
    }


    pub fn run_client(stream: TcpStream) {
        loop {
            match run_client_once(stream.try_clone()) {
                Ok(_) => break, // Si la fonction s'exécute avec succès, sortir de la boucle
                Err(err) => {
                    eprintln!("Error running client: {}", err);
                    continue; // Redémarrer la boucle pour réessayer
                }
            }
        }
    }
    
    fn run_client_once(stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
        // Envoyer un FragmentRequest au serveur
        let fragment_request = FragmentRequest::new(
            // hostname::get() as String,
            "client de test".to_string(),
            1000,
        );
        send_message(&stream, &fragment_request);
    
        // Attendre la réponse du serveur (FragmentTask)
        let fragment_task = receive_fragment_task(&stream)?;
        debug_println!("Received FragmentTask: {:?}", fragment_task);
    
        // Boucle principale du client
        loop {
            // Calculer les pixels du fragment et créer un FragmentResult
            let fragment_result = calculate_fragment_result(&fragment_task);
    
            // Envoyer le FragmentResult au serveur
            send_message(&stream, &fragment_result)?;
    
            // Attendre la réponse du serveur (FragmentTask ou FragmentResult)
            let server_response = receive_message(&stream);
            match server_response.try_into(FragmentTask).get("FragmentTask") {
                Some(task) => {
                    // Nouvelle tâche du serveur
                    println!("Received FragmentTask: {:?}", task);
                    fragment_task = task.clone();
                }
                None => {
                    // Le serveur a accepté le FragmentResult, continuer la boucle
                }
            }
        }
    }
    
}

#[cfg(test)]
mod tests {
    use std::net::{TcpListener, TcpStream};
    use std::os::windows::thread;
    
    use self::client::run_client;

    use super::*;

    #[test]
    fn test_run_client() {
        // Créez un serveur factice pour simuler la connexion TCP
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
        let port = listener.local_addr().unwrap().port();

        // Lancez le serveur dans un thread séparé
        let server_thread = std::thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        // Simulation du serveur avec un FragmentTask factice en réponse
                        let fragment_task = json!({
                            "FragmentTask": {
                                // Valeurs factices
                            }
                        });
                        send_message(&stream, &fragment_task);
                    }
                    Err(e) => {
                        eprintln!("Error accepting connection: {:?}", e);
                    }
                }
            }
        });

        // Créez une connexion client et exécutez le client
        let client_thread = std::thread::spawn;(move || {
            // Créez une connexion client vers le serveur factice
            let stream = TcpStream::connect(("127.0.0.1", port)).expect("Failed to connect to server");

            // Exécutez la fonction run_client pour tester
            run_client(stream);
        });

        // Attendez que les threads se terminent
        server_thread.join().expect("Server thread panicked");
        client_thread.join().expect("Client thread panicked");
    }
}