use std::fmt::Debug;
use axum::{extract::State, response::IntoResponse, Json};
use axum::http::StatusCode;
use crate::{models::*, AppState};
use crate::handlers::handlers_inner::HandlerError;

mod handlers_inner;

// ---- CRUD for Questions ----

pub async fn create_question(
    // Example of how to add state to a route. Note that we are using ".." to ignore the other fields in AppState.
    State(AppState { questions_dao, .. }): State<AppState>,
    Json(question): Json<Question>,
) -> impl IntoResponse {
    let qd = questions_dao
        .create_question(question)
        .await
        .map_err(|_| HandlerError::default_internal_error())
        .unwrap();

    Json(QuestionDetail {
        question_uuid: qd.question_uuid,
        title: qd.title,
        description: qd.description,
        created_at: qd.created_at,
    })
}

pub async fn read_questions(// TODO: add questions_dao from app state as an argument
    State(AppState { questions_dao, .. }): State<AppState>,
) -> impl IntoResponse {

    let records  =
        questions_dao
            .get_questions()
            .await
            .map_err(|_| HandlerError::default_internal_error())
        .unwrap();

    Json(records)
}

pub async fn delete_question(
    // TODO: add questions_dao from app state as an argument
    State(AppState {questions_dao, ..}) : State<AppState>,
    Json(question_uuid): Json<QuestionId>,
) {
    questions_dao
        .delete_question(question_uuid.question_uuid)
        .await
        .map_err(|_| HandlerError::default_internal_error())
        .unwrap()
}

// ---- CRUD for Answers ----

pub async fn create_answer(
    // Example of how to add state to a route
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(answer): Json<Answer>,
) -> impl IntoResponse {

    let answer = answers_dao
        .create_answer(answer)
        .await
        .map_err(|_| HandlerError::default_internal_error())
        .unwrap();

    Json(AnswerDetail {
        answer_uuid: answer.answer_uuid,
        question_uuid: answer.question_uuid,
        content: answer.content,
        created_at: answer.created_at,
    })
}

pub async fn read_answers(
    // TODO: add answers_dao from app state as an argument
    State(AppState {answers_dao, ..}): State<AppState>,
    Json(question_uuid): Json<QuestionId>,
) -> impl IntoResponse {
    let answers = answers_dao
        .get_answers(question_uuid.question_uuid)
        .await
        .map_err(|_| HandlerError::default_internal_error())
        .unwrap();
    Json(answers)
}

pub async fn delete_answer(
    // TODO: add answers_dao from app state as an argument
    State(AppState {answers_dao, ..}): State<AppState>,
    Json(answer_uuid): Json<AnswerId>,
) {
    answers_dao
        .delete_answer(answer_uuid.answer_uuid)
        .await
        .map_err(|_| HandlerError::default_internal_error())
        .unwrap()
}
