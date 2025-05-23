use std::{sync::Arc, thread};

use chrono::Utc;
use tiny_http::{Header, Response, Server};

use crate::{nonclient_timeraccess::NonclientTimerAccess, shared_data::SharedData};

pub fn start_webserver_thread(shared_data: Arc<SharedData>) {
    thread::spawn(|| webserver(shared_data));
}

fn webserver(shared_data: Arc<SharedData>) {
    let port = shared_data.config.timer_port;
    let server = Server::http(format!("0.0.0.0:{}", port)).unwrap();

    for request in server.incoming_requests() {
        let response = match request.url() {
            "/" => {
                // store access time
                let mut is_valid_client = false;
                if let Some(std::net::SocketAddr::V4(sockaddr)) = request.remote_addr() {
                    let remote_ip = sockaddr.ip();
                    let now = Utc::now();
                    let mut time_stored = false;
                    // check if request comes from registered client
                    for client in &shared_data.clients {
                        if client.ip_address.eq(remote_ip) {
                            *client.last_timer_access.lock().unwrap() = Some(now);
                            is_valid_client = true;
                            time_stored = true;
                        }
                    }
                    // check if request comes from unregistered address that requested before
                    if !time_stored {
                        for nonclient in shared_data.nonclients.lock().unwrap().iter_mut() {
                            if nonclient.ip_address.eq(remote_ip) {
                                nonclient.last_timer_access = now;
                                time_stored = true;
                            }
                        }
                    }
                    // ip address has never requested timer before, create new nonclient to track
                    if !time_stored {
                        let new_nonclient = NonclientTimerAccess::new(remote_ip.clone(), now);
                        shared_data.nonclients.lock().unwrap().push(new_nonclient);
                    }
                }
                let mut response =
                    if is_valid_client || shared_data.config.timer_allow_nonclient_access {
                        Response::from_data(
                            generate_html(
                                shared_data.finish_time_as_unix(),
                                shared_data.config.timer_duration_minutes,
                                shared_data.config.timer_webpage_refresh_seconds,
                                shared_data.config.timer_webpage_refresh_unstarted_seconds,
                            )
                            .as_bytes(),
                        )
                    } else {
                        Response::from_data(generate_html_illegal_access().as_bytes())
                    };
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

fn generate_html(
    times: Option<i64>,
    default_time: i64,
    refresh_interval_running: u32,
    refresh_interval_unstarted: u32,
) -> String {
    let target_time = times.unwrap_or(-1);
    let refresh_delay = match times {
        Some(_) => refresh_interval_running,
        None => refresh_interval_unstarted,
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

                    const now = new Date().getTime() / 1000;
                    const distance = targetDate - now;

                    if (targetDate == -1) {{
                        text = "Time left: {}min 0s";
                    }} else if (distance < 0) {{
                        text = "time is up!";
                    }} else {{
                        const minutes = Math.floor(distance / 60);
                        const seconds = Math.floor((distance % 60));
                        text = "Time left: " + minutes + "m " + seconds + "s";
                    }}

                    document.getElementById("countdown").innerHTML = text;
                    document.title = text;

                }}, 1000);
            </script>
        </body>
        </html>
        "#,
        refresh_delay, target_time, default_time
    )
}

fn generate_html_illegal_access() -> String {
    r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Illegal Access!</title>
        <style>
            body {
                color: white;
                text-align: center;
                font-size: 50px;
                font-family: Arial, sans-serif;
                margin-top: 20%;
                animation: blink-bg 1s infinite;
            }

            @keyframes blink-bg {
                0% { background-color: black; }
                50% { background-color: red; }
                100% { background-color: black; }
            }
        </style>
    </head>
    <body>
        <div>Illegal Access!</div>
    </body>
    </html>
    "#
    .to_string()
}
