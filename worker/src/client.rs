// use worker::
// use shared::messages_types::{};

pub mod client{
    
    use std::{io::{Read, Write}, net::TcpStream};

    use hostname::get;
    use serde::Serialize;
    use serde_json::{json, Value};
    use shared::messages_types::{FragmentRequest, FragmentResult, FragmentTask};

    use crate::message::Message;
    
    


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
                eprintln!("Error serializing message: {:?}", err);
            }
        }
    }

    // Fonction pour recevoir un message JSON du flux TCP
    fn receive_message(stream: &dyn Message) ->  Box<dyn Message> {
        let mut buffer = String::new();
        let mut stream_clone = stream.try_clone().expect("Unable to clone TcpStream");
        stream_clone.read_to_string(&mut buffer).expect("Failed to read from stream");

        serde_json::from_str(&buffer).expect("Failed to deserialize message")
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


    // Fonction principale du client qui gère la boucle de communication avec le serveur
    pub fn run_client(stream: TcpStream) {
        //setupClient();

        // Envoyer un FragmentRequest au serveur
        let fragment_request: FragmentRequest = FragmentRequest::new(
                // hostname::get() as String,
                "client de test".to_string(),
                1000
        );
        send_message(&stream, &fragment_request);

        // Attendre la réponse du serveur (FragmentTask)
        let mut fragment_task = receive_message(&stream);
        println!("Received FragmentTask: {:?}", fragment_task);

        // Boucle principale du client
        loop {
            // Calculer les pixels du fragment et créer un FragmentResult
            let fragment_result = calculate_fragment_result(&fragment_task);

            // Envoyer le FragmentResult au serveur
            send_message(&stream, &fragment_result);

            // Attendre la réponse du serveur (FragmentTask ou FragmentResult)
            let server_response = receive_message(&stream);
            match server_response.get("FragmentTask") {
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