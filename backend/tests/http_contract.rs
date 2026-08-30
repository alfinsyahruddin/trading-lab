use actix_web::{http::StatusCode, test, App};
use trading_lab_backend::routes;

#[actix_web::test]
async fn root_should_return_success_envelope() {
    let service = test::init_service(App::new().configure(routes::configure)).await;
    let response =
        test::call_service(&service, test::TestRequest::get().uri("/").to_request()).await;
    let body = test::read_body(response).await;
    let envelope: serde_json::Value = serde_json::from_slice(&body).expect("valid response JSON");
    assert_eq!(envelope["status"], StatusCode::OK.as_u16());
}
