use rocket::{serde::json::Json};

use crate::models::*;

// ---- CRUD for Questions ----

#[post("/question", data = "<question>")]
pub async fn create_question(
    question: Json<Question>,
) -> Json<QuestionDetail> {
    Json(QuestionDetail{
        question_uuid: "question_uuid".to_string(),
        title: "title".to_string(),
        description: "description".to_string(),
        created_at: "created_at".to_string()
    }
    )
}

#[get("/questions")]
pub async fn read_questions() -> Json<Vec<QuestionDetail>> {
    Json (
        vec![QuestionDetail {
            question_uuid: "question_uuid".to_owned(),
            title: "title".to_owned(),
            description: "description".to_owned(),
            created_at: "created_at".to_owned()
        }]
    )
}

#[delete("/question", data = "<question_uuid>")]
pub async fn delete_question(
    question_uuid: Json<QuestionId>
) {
    // ...
}

// ---- CRUD for Answers ----

// TODO: Create a POST route to /answer which accepts an `Answer` and returns `AnswerDetail` as JSON.
//       The handler function should be called `create_answer`.
//       
//       hint: this function should look very similar to the create_question function above

#[post("/answer", data = "<answer>")]
pub async fn create_answer(
    answer: Json<Answer>,
) -> Json<AnswerDetail> {
    Json(AnswerDetail{
        answer_uuid: "answer_uuid".to_string(),
        question_uuid: "question_uuid".to_string(),
        content: "content".to_string(),
        created_at: "created_at".to_string()
    })
}


// TODO: Create a GET route to /answers which accepts an `QuestionId` and returns a vector of `AnswerDetail` as JSON.
//       The handler function should be called `read_answers`.
//       
//       hint: this function should look very similar to the read_questions function above

#[get("/answers", data = "<question_uuid>")]
pub async fn read_answers(question_uuid: Json<QuestionId>) -> Json<Vec<AnswerDetail>> {
    Json (
        vec![AnswerDetail{
            answer_uuid: "answer_uuid".to_string(),
            question_uuid: "question_uuid".to_string(),
            content: "content".to_string(),
            created_at: "created_at".to_string()
        }]
    )
}

// TODO: Create a DELETE route to /answer which accepts an `AnswerId` and does not return anything.
//       The handler function should be called `delete_answer`.
//       
//       hint: this function should look very similar to the delete_question function above

#[delete("/answer", data = "<answer_uuid>")]
pub async fn delete_answer(
    answer_uuid: Json<AnswerId>
) {
    // ...
}
