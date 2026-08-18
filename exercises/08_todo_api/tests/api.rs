use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use serde_json::Value;
use stage08_todo_api::{AppState, router};
use tower::ServiceExt;

#[tokio::test]
async fn runs_the_crud_flow() {
    let app = router(AppState::default());
    let response = app
        .clone()
        .oneshot(
            Request::post("/tasks")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"title":"学习 Axum"}"#))
                .expect("request should be valid"),
        )
        .await
        .expect("router should respond");
    assert_eq!(response.status(), StatusCode::CREATED);
    let body = to_bytes(response.into_body(), 4096)
        .await
        .expect("response body should be readable");
    let created: Value = serde_json::from_slice(&body).expect("response should be JSON");
    assert_eq!(created["id"], 1);
    assert_eq!(created["completed"], false);

    let response = app
        .clone()
        .oneshot(
            Request::patch("/tasks/1/complete")
                .body(Body::empty())
                .expect("request should be valid"),
        )
        .await
        .expect("router should respond");
    assert_eq!(response.status(), StatusCode::OK);

    let response = app
        .clone()
        .oneshot(
            Request::delete("/tasks/1")
                .body(Body::empty())
                .expect("request should be valid"),
        )
        .await
        .expect("router should respond");
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    let response = app
        .oneshot(
            Request::get("/tasks/1")
                .body(Body::empty())
                .expect("request should be valid"),
        )
        .await
        .expect("router should respond");
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn rejects_a_blank_title() {
    let response = router(AppState::default())
        .oneshot(
            Request::post("/tasks")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"title":"  "}"#))
                .expect("request should be valid"),
        )
        .await
        .expect("router should respond");
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}
