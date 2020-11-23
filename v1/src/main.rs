use anyhow::{anyhow, Result};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use cityhash::city_hash_64;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{udp::SendHalf, UdpSocket};
use tokio::sync::Mutex;
use tracing::{error, info};
use v1::utils::router::{ReqContext, ResponseContext};
use v1::{build_routers, get_diesel_pool};

const KEY: &str = "F9B14CEC-60B6-810F-1FF7-8BAE688466AC";

#[tokio::main]
async fn main() -> Result<()> {
    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_env_filter(
                tracing_subscriber::EnvFilter::from_default_env()
                    .add_directive(tracing_subscriber::filter::LevelFilter::ERROR.into()),
            )
            .finish(),
    )
    .unwrap();

    let socket = UdpSocket::bind("192.168.1.129:4434")
        .await
        .map_err(|e| return e)?;

    let router = build_routers();

    let diesel_pool = get_diesel_pool();

    let state: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));

    let (mut recv, sender) = socket.split();

    let sender = Arc::new(Mutex::new(sender));

    loop {
        let mut stream_buf = [0; 65535];
        let sender = sender.clone();
        let current_sender = sender.clone();
        let diesel_pool = diesel_pool.clone();
        let router = router.clone();
        let state = state.clone();

        let (receiverd, peer) = match recv.recv_from(&mut stream_buf).await.map_err(|e| {
            error!("failed udp socket recv message:{:?}", e);
            return e;
        }) {
            Ok(v) => v,
            Err(e) => {
                error!("failed recv data:{:?}", e);
                continue;
            }
        };
        let mut cursor = std::io::Cursor::new(&stream_buf[..receiverd]);
        let code = match cursor.read_u16::<LittleEndian>() {
            Ok(v) => v,
            Err(e) => {
                error!("invalid code formats:{:?}", e);
                let resp = match ResponseContext::get_bincode(0, 0, 506, "invalid code.", "") {
                    Ok(v) => v,
                    Err(e) => {
                        error!("fialed encode response: {:?}", e);
                        continue;
                    }
                };

                handle_stream(current_sender, &peer, resp).await;
                continue;
            }
        };

        let version = match cursor.read_u8() {
            Ok(v) => v,
            Err(e) => {
                error!("invalid version formats:{:?}", e);
                let resp = match ResponseContext::get_bincode(code, 0, 506, "invalid version.", "")
                {
                    Ok(v) => v,
                    Err(e) => {
                        error!("fialed encode response: {:?}", e);
                        continue;
                    }
                };

                handle_stream(current_sender, &peer, resp).await;
                continue;
            }
        };

        let session_id = match cursor.read_u64::<LittleEndian>() {
            Ok(v) => v,
            Err(e) => {
                error!("invalid session id formats:{:?}", e);
                let resp =
                    match ResponseContext::get_bincode(code, 0, 506, "invalid session id.", "") {
                        Ok(v) => v,
                        Err(e) => {
                            error!("fialed encode response: {:?}", e);
                            continue;
                        }
                    };

                handle_stream(current_sender, &peer, resp).await;
                continue;
            }
        };

        let signature = match cursor.read_u64::<LittleEndian>() {
            Ok(v) => v,
            Err(e) => {
                error!("invalid signature formats:{:?}", e);
                let resp = match ResponseContext::get_bincode(
                    code,
                    session_id,
                    506,
                    "invalid signature format.",
                    "",
                ) {
                    Ok(v) => v,
                    Err(e) => {
                        error!("fialed encode response: {:?}", e);
                        continue;
                    }
                };

                handle_stream(current_sender, &peer, resp).await;
                continue;
            }
        };

        let timestamp = match cursor.read_u64::<LittleEndian>() {
            Ok(v) => v,
            Err(e) => {
                error!("invalid timestamp formats:{:?}", e);
                let resp = match ResponseContext::get_bincode(
                    code,
                    session_id,
                    506,
                    "invalid timestamp.",
                    "",
                ) {
                    Ok(v) => v,
                    Err(e) => {
                        error!("fialed encode response: {:?}", e);
                        continue;
                    }
                };

                handle_stream(current_sender, &peer, resp).await;
                continue;
            }
        };

        let len = match cursor.read_u32::<LittleEndian>() {
            Ok(v) => v,
            Err(e) => {
                error!("invalid length formats:{:?}", e);
                let resp = match ResponseContext::get_bincode(
                    code,
                    session_id,
                    506,
                    "invalid body length.",
                    "",
                ) {
                    Ok(v) => v,
                    Err(e) => {
                        error!("fialed encode response: {:?}", e);
                        continue;
                    }
                };

                handle_stream(current_sender, &peer, resp).await;
                continue;
            }
        };
        let position = cursor.position() as usize;
        let mut new_body = Vec::new();

        if len > 0 {
            let body = &stream_buf[position..position + (len as usize)];
            new_body.extend_from_slice(body);

            info!(
                "message content->session id:{},signature:{},signature x:{:x},timestamp:{},len:{},content:{:?}",
                session_id,
                signature,
                signature,
                timestamp,
                new_body.len(),
                new_body,
            );
        }

        //signature valid
        if let Err(e) = signature_valid(signature, timestamp, &new_body) {
            error!("invalid signature formats:{:?}", e);

            let resp =
                match ResponseContext::get_bincode(code, session_id, 506, "invalid signature.", "")
                {
                    Ok(v) => v,
                    Err(e) => {
                        error!("fialed encode response: {:?}", e);
                        continue;
                    }
                };

            handle_stream(current_sender, &peer, resp).await;
            continue;
        }

        let ctx = ReqContext {
            socket: sender.clone(),
            peer: peer,
            db: diesel_pool.clone(),
            state,
            code,
            version,
            session_id,
            body_length: len,
            body: new_body,
        };

        info!(
            "stream request content->receiverd:{}, code:{},version:{},len:{},position:{}",
            receiverd, code, version, len, position
        );

        tokio::spawn(async move {
            match router.call(code) {
                Ok(f) => {
                    let resp = match f(ctx).await {
                        Ok(v) => v,
                        Err(e) => {
                            error!("failed method exec:{:?}", e);
                            let resp = match ResponseContext::get_bincode(
                                code,
                                session_id,
                                506,
                                &e.to_string(),
                                "",
                            ) {
                                Ok(v) => v,
                                Err(e) => {
                                    error!("fialed encode response: {:?}", e);
                                    return;
                                }
                            };
                            handle_stream(sender.clone(), &peer, resp).await;
                            return;
                        }
                    };

                    handle_stream(sender.clone(), &peer, resp).await;
                }

                Err(e) => {
                    error!("response error:{}", &e.to_string());
                    let resp = match ResponseContext::get_bincode(
                        code,
                        session_id,
                        506,
                        &e.to_string(),
                        "",
                    ) {
                        Ok(v) => v,
                        Err(e) => {
                            error!("fialed encode response: {:?}", e);
                            return;
                        }
                    };
                    handle_stream(sender.clone(), &peer, resp).await;
                    return;
                }
            }
        });
    }
}

//signature valid
fn signature_valid(sign: u64, timestamp: u64, body: &[u8]) -> Result<()> {
    let mut signature = Vec::new();

    signature.extend_from_slice(KEY.as_bytes());

    signature.write_u64::<LittleEndian>(timestamp)?;

    signature.extend_from_slice(body);

    let signature = city_hash_64(&signature);

    // info!("signaturex:{:x},signature:{}", signature, signature);

    if sign != signature {
        return Err(anyhow!("invalid signature."));
    }

    Ok(())
}

async fn handle_stream(socket: Arc<Mutex<SendHalf>>, peer: &SocketAddr, body: Vec<u8>) {
    let _written = match socket.lock().await.send_to(&body, peer).await {
        Ok(v) => v,

        Err(e) => {
            error!("stream send failed {:?}", e);
            return;
        }
    };
}

#[allow(unused)]
fn hex_dump(buf: &[u8]) -> String {
    let vec: Vec<String> = buf.iter().map(|b| format!("{:02x}", b)).collect();

    vec.join("")
}
