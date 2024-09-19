use std::{
    sync::{Arc, Mutex},
    thread,
};

use tiny_http::{Header, Response, Server};

pub fn start_webserver_thread(time: Arc<Mutex<Option<i64>>>) {
    thread::spawn(|| webserver(time));
}

fn webserver(time: Arc<Mutex<Option<i64>>>) {
    // Create a new HTTP server and bind it to localhost:8080
    let server = Server::http("0.0.0.0:8080").unwrap();
    println!("Listening on http://0.0.0.0:8080");

    for request in server.incoming_requests() {
        // Log the received request
        println!("Received request: {}", request.url());

        // Match on the URL to handle different routes
        let response = match request.url() {
            "/" => {
                let mut response =
                    Response::from_data(generate_html(*time.lock().unwrap()).as_bytes());
                response.add_header(
                    Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap(),
                );
                response
            }
            _ => Response::from_string("404 Not Found").with_status_code(404),
        };

        // Send the response to the client
        request.respond(response).unwrap();
    }
}

fn generate_html(time: Option<i64>) -> String {
    let target_time = time.unwrap_or(-1);
    let refresh_delay = match time {
        Some(_) => 30,
        None => 3,
    };
    format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <meta http-equiv="refresh" content="{}">
            <title>.</title>
            <style>
                body {{
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    height: 100vh;
                    font-family: Arial, sans-serif;
                    background-color: #f0f0f0;
                    margin: 0;
                }}
                #countdown {{
                    font-size: 48px;
                    color: #333;
                }}
            </style>
        </head>
        <body>
            <div id="countdown"></div>

            <script>
                const targetDate = {};

                // Update the countdown every 1 second
                const countdown = setInterval(function() {{
                    let text = "";

                    const now = new Date().getTime();
                    const distance = targetDate - now;

                    if (targetDate == -1) {{
                        text = "not started";
                    }} else if (distance < 0) {{
                        text = "time is up!";
                    }} else {{
                        const minutes = Math.floor(distance / (1000 * 60));
                        const seconds = Math.floor((distance % (1000 * 60)) / 1000);
                        text = minutes + "m " + seconds + "s";
                    }}


                    document.getElementById("countdown").innerHTML = text;
                    document.title = text;

                }}, 1000);
            </script>
        </body>
        </html>
        "#,
        refresh_delay, target_time
    )
}
