use actix_web::{
    get,
    http::{header, StatusCode},
    post, test,
    web::{self, Json},
    App, HttpResponse,
};
use trading_lab_backend::{
    entities::{
        app_error::AppError,
        app_response::AppResponse,
        base_response::{BaseResponse, JsonFromStringTrait},
    },
    http, routes,
};

#[actix_web::test]
async fn root_should_return_success_envelope() {
    let service = test::init_service(App::new().configure(routes::configure)).await;
    let response =
        test::call_service(&service, test::TestRequest::get().uri("/").to_request()).await;

    assert_eq!(response.status(), StatusCode::OK);
    let body = test::read_body(response).await;
    let envelope: serde_json::Value = serde_json::from_slice(&body).expect("valid response JSON");

    assert_eq!(envelope["status"], 200);
    assert_eq!(envelope["data"], "Trading Lab API");
    assert!(envelope["message"].is_null());
    assert!(envelope["timestamp"].is_string());
}

#[actix_web::test]
async fn health_should_return_success_envelope() {
    let service = test::init_service(App::new().configure(routes::configure)).await;
    let response = test::call_service(
        &service,
        test::TestRequest::get().uri("/health").to_request(),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);
    let body = test::read_body(response).await;
    let envelope: serde_json::Value = serde_json::from_slice(&body).expect("valid response JSON");

    assert_eq!(envelope["status"], 200);
    assert_eq!(envelope["data"], "ok");
    assert!(envelope["message"].is_null());
    assert!(envelope["timestamp"].is_string());
}

#[actix_web::test]
async fn not_found_should_return_standard_error_envelope() {
    let service = test::init_service(
        App::new()
            .default_service(web::to(routes::not_found))
            .configure(routes::configure),
    )
    .await;

    let response = test::call_service(
        &service,
        test::TestRequest::get()
            .uri("/non-existent-endpoint")
            .to_request(),
    )
    .await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let body = test::read_body(response).await;
    let envelope: serde_json::Value = serde_json::from_slice(&body).expect("valid response JSON");

    assert_eq!(envelope["status"], 404);
    assert_eq!(envelope["message"], "Not found");
    assert!(envelope["data"].is_null());
    assert!(envelope["timestamp"].is_string());
}

#[derive(serde::Deserialize)]
struct TestPayload {
    name: String,
}

#[post("/test-json")]
async fn test_json_endpoint(body: Json<TestPayload>) -> AppResponse<String> {
    body.into_inner().name.json_data()
}

#[actix_web::test]
async fn invalid_json_payload_should_return_400_with_envelope() {
    let service = test::init_service(
        App::new()
            .app_data(http::json_config())
            .service(test_json_endpoint),
    )
    .await;

    let response = test::call_service(
        &service,
        test::TestRequest::post()
            .uri("/test-json")
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .set_payload(r#"{"name": broken JSON here"#)
            .to_request(),
    )
    .await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = test::read_body(response).await;
    let envelope: serde_json::Value = serde_json::from_slice(&body).expect("valid response JSON");

    assert_eq!(envelope["status"], 400);
    assert_eq!(envelope["message"], "Invalid JSON request body");
    assert!(envelope["data"].is_null());
    assert!(envelope["timestamp"].is_string());
}

#[get("/test-error/{code}")]
async fn test_error_endpoint(code: web::Path<String>) -> Result<HttpResponse, AppError> {
    match code.as_str() {
        "bad-request" => Err(AppError::bad_request("Malformed input parameter")),
        "unauthorized" => Err(AppError::unauthorized("Token expired or missing")),
        "forbidden" => Err(AppError::Forbidden("Admin privileges required".into())),
        "not-found" => Err(AppError::not_found("Requested item does not exist")),
        "conflict" => Err(AppError::conflict("Resource name already taken")),
        "internal" => Err(AppError::Internal),
        _ => Ok(HttpResponse::Ok().json(BaseResponse::<()>::message("ok"))),
    }
}

#[actix_web::test]
async fn app_error_envelopes_should_conform_to_contract() {
    let service = test::init_service(App::new().service(test_error_endpoint)).await;

    let test_cases = [
        (
            "/test-error/bad-request",
            StatusCode::BAD_REQUEST,
            400,
            "Malformed input parameter",
        ),
        (
            "/test-error/unauthorized",
            StatusCode::UNAUTHORIZED,
            401,
            "Token expired or missing",
        ),
        (
            "/test-error/forbidden",
            StatusCode::FORBIDDEN,
            403,
            "Admin privileges required",
        ),
        (
            "/test-error/not-found",
            StatusCode::NOT_FOUND,
            404,
            "Requested item does not exist",
        ),
        (
            "/test-error/conflict",
            StatusCode::CONFLICT,
            409,
            "Resource name already taken",
        ),
        (
            "/test-error/internal",
            StatusCode::INTERNAL_SERVER_ERROR,
            500,
            "Internal server error", // masks internal error details
        ),
    ];

    for (uri, expected_http_status, expected_body_status, expected_message) in test_cases {
        let response =
            test::call_service(&service, test::TestRequest::get().uri(uri).to_request()).await;

        assert_eq!(
            response.status(),
            expected_http_status,
            "Failed status check for {uri}"
        );

        let body = test::read_body(response).await;
        let envelope: serde_json::Value =
            serde_json::from_slice(&body).expect("valid response JSON");

        assert_eq!(envelope["status"], expected_body_status);
        assert_eq!(envelope["message"], expected_message);
        assert!(
            envelope["data"].is_null(),
            "Expected data to be null on error for {uri}"
        );
        assert!(
            envelope["timestamp"].is_string(),
            "Expected ISO timestamp for {uri}"
        );
    }
}

#[actix_web::test]
async fn cors_preflight_should_include_allowed_origin() {
    let service = test::init_service(
        App::new()
            .wrap(http::cors("http://localhost:3000"))
            .service(routes::index),
    )
    .await;

    let response = test::call_service(
        &service,
        test::TestRequest::default()
            .method(actix_web::http::Method::OPTIONS)
            .uri("/")
            .insert_header((header::ORIGIN, "http://localhost:3000"))
            .insert_header((
                header::ACCESS_CONTROL_REQUEST_METHOD,
                actix_web::http::Method::GET.as_str(),
            ))
            .to_request(),
    )
    .await;

    assert_eq!(response.status(), StatusCode::OK);
    let allow_origin = response.headers().get(header::ACCESS_CONTROL_ALLOW_ORIGIN);
    assert!(allow_origin.is_some());
    assert_eq!(
        allow_origin.unwrap().to_str().unwrap(),
        "http://localhost:3000"
    );
}

#[actix_web::test]
async fn strategy_variables_without_auth_should_return_401() {
    let service = test::init_service(App::new().configure(routes::configure)).await;
    let response = test::call_service(
        &service,
        test::TestRequest::get()
            .uri("/api/strategies/variables")
            .to_request(),
    )
    .await;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    let body = test::read_body(response).await;
    let envelope: serde_json::Value = serde_json::from_slice(&body).expect("valid response JSON");
    assert_eq!(envelope["status"], 401);
}
