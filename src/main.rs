// #![no_std]
// #![no_main]

// extern crate alloc;

// use crate::alloc::string::ToString;
// use noli::*;
// use saba_core::browser::Browser;
// use saba_core::http::HttpResponse;

// use alloc::format;
// use alloc::rc::Rc;
// use alloc::string::String;
// use core::cell::RefCell;
// use net_wasabi::http::HttpClient;
// use saba_core::error::Error;
// use saba_core::url::Url;
// use ui_wasabi::app::WasabiUI;

// // static TEST_HTTP_RESPONSE: &str = r#"HTTP/1.1 200 OK
// // Data: xx xx xx

// // <html>
// // <head></head>
// // <body>
// //   <h1 id="title">H1 title</h1>
// //   <h2 class="class">H2 title</h2>
// //   <p>Test text.</p>
// //   <p>
// //     <a href="example.com">Link1</a>
// //     <a href="example.com">Link2</a>
// //   </p>
// // </body>
// // </html>
// // "#;

// // fn main() -> u64 {
// //     // ch.5
// //     let browser = Browser::new();

// //     let response =
// //         HttpResponse::new(TEST_HTTP_RESPONSE.to_string()).expect("failed to parse http response");
// //     let page = browser.borrow().current_page();
// //     let dom_string = page.borrow_mut().receive_response(response);
// //     for log in dom_string.lines() {
// //         println!("{}", log);
// //     }

// //     0
// // }

// fn handle_url(url: String) -> Result<HttpResponse, Error> {
//     // URLを解釈する
//     let parsed_url = match Url::new(url.to_string()).parse() {
//         Ok(url) => url,
//         Err(e) => {
//             return Err(Error::UnexpectedInput(format!(
//                 "input html is not supported: {:?}",
//                 e
//             )));
//         }
//     };

//     // HTTPリクエストを送信する
//     let client = HttpClient::new();
//     let response = match client.get(
//         parsed_url.host(),
//         parsed_url.port().parse::<u16>().expect(&format!(
//             "port number should be u16 but got {}",
//             parsed_url.port()
//         )),
//         parsed_url.path(),
//     ) {
//         Ok(res) => {
//             // HTTPレスポンスのステータスコードが302のとき、転送する（リダイレクト）
//             if res.status_code() == 302 {
//                 let location = match res.header_value("Location") {
//                     Ok(value) => value,
//                     Err(_) => return Ok(res),
//                 };
//                 let redirect_parsed_url = Url::new(location);

//                 let redirect_res = match client.get(
//                     redirect_parsed_url.host(),
//                     redirect_parsed_url.port().parse::<u16>().expect(&format!(
//                         "port number should be u16 but got {}",
//                         parsed_url.port()
//                     )),
//                     redirect_parsed_url.path(),
//                 ) {
//                     Ok(res) => res,
//                     Err(e) => return Err(Error::Network(format!("{:?}", e))),
//                 };

//                 redirect_res
//             } else {
//                 res
//             }
//         }
//         Err(e) => {
//             return Err(Error::Network(format!(
//                 "failed to get http response: {:?}",
//                 e
//             )))
//         }
//     };
//     Ok(response)
// }

// fn main() -> u64 {
//     // Browser構造体を初期化
//     let browser = Browser::new();

//     // WasabiUI構造体を初期化
//     let ui = Rc::new(RefCell::new(WasabiUI::new(browser)));

//     // アプリの実行を開始
//     match ui.borrow_mut().start(handle_url) {
//         Ok(_) => {}
//         Err(e) => {
//             println!("browser fails to start {:?}", e);
//             return 1;
//         }
//     };

//     0
// }

// entry_point!(main);

#![no_std]
#![no_main]

extern crate alloc;

use crate::alloc::string::ToString;
use alloc::format;
use alloc::rc::Rc;
use alloc::string::String;
use core::cell::RefCell;
use net_wasabi::http::HttpClient;
use noli::*;
use saba_core::browser::Browser;
use saba_core::error::Error;
use saba_core::http::HttpResponse;
use saba_core::url::Url;
use ui_wasabi::app::WasabiUI;

fn handle_url(url: String) -> Result<HttpResponse, Error> {
    // URLを解釈する
    let parsed_url = match Url::new(url.to_string()).parse() {
        Ok(url) => url,
        Err(e) => {
            return Err(Error::UnexpectedInput(format!(
                "input html is not supported: {:?}",
                e
            )));
        }
    };
    println!("{:?}", parsed_url);

    // HTTPリクエストを送信する
    let client = HttpClient::new();
    let response = match client.get(
        parsed_url.host(),
        parsed_url.port().parse::<u16>().expect(&format!(
            "port number should be u16 but got {}",
            parsed_url.port()
        )),
        parsed_url.path(),
    ) {
        Ok(res) => {
            // HTTPレスポンスのステータスコードが302のとき、転送する（リダイレクト）
            if res.status_code() == 302 {
                let location = match res.header_value("Location") {
                    Ok(value) => value,
                    Err(_) => return Ok(res),
                };
                let redirect_parsed_url = Url::new(location);

                let redirect_res = match client.get(
                    redirect_parsed_url.host(),
                    redirect_parsed_url.port().parse::<u16>().expect(&format!(
                        "port number should be u16 but got {}",
                        parsed_url.port()
                    )),
                    redirect_parsed_url.path(),
                ) {
                    Ok(res) => res,
                    Err(e) => return Err(Error::Network(format!("{:?}", e))),
                };

                redirect_res
            } else {
                res
            }
        }
        Err(e) => {
            return Err(Error::Network(format!(
                "failed to get http response: {:?}",
                e
            )))
        }
    };
    Ok(response)
}

fn main() -> u64 {
    // Browser構造体を初期化
    let browser = Browser::new();

    // WasabiUI構造体を初期化
    let ui = Rc::new(RefCell::new(WasabiUI::new(browser)));

    // アプリの実行を開始
    match ui.borrow_mut().start(handle_url) {
        Ok(_) => {}
        Err(e) => {
            println!("browser fails to start {:?}", e);
            return 1;
        }
    };

    0
}

entry_point!(main);
