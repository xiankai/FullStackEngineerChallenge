use serde::{Serialize, Deserialize};
use serde_json::Result;
use crate::db::Database;

#[derive(Serialize, Deserialize, Debug)]
pub struct AddEmployeePayload {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateEmployeePayload {
    id: i32,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteEmployeePayload {
    employee_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetReviewsPayload {
    employee_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddReviewPayload {
    employee_id: i32,
    review: String,
    score: i32,
}

// commented routes are not implemented
// each route starts a new DB connection - we cannot reuse the existing connection
// due to thread safety concerns raised by the rusqlite crate
pub fn routes (method: &str, path: &str, raw_payload: &str) -> Result<String> {
    let db = Database::new().unwrap();
    match (method, path) {
        ("GET", "/get_employees") => {
            let employees = db.get_employees().unwrap();
            serde_json::to_string(&employees)
        },
        ("POST", "/add_employee") => {
            let payload: AddEmployeePayload = serde_json::from_str(raw_payload)?;
            db.add_employee(payload.name);
            Ok("success".to_string())
        },
        // ("POST", "/update_employee") => {
        //     let payload: UpdateEmployeePayload = serde_json::from_str(raw_payload)?;
        //     db.update_employee(payload.id, payload.name);
        //     Ok("success".to_string())
        // },
        ("POST", "/delete_employee") => {
            let payload: DeleteEmployeePayload = serde_json::from_str(raw_payload)?;
            db.delete_employee(payload.employee_id);
            Ok("success".to_string())
        },
        ("POST", "/get_reviews") => {
            let payload: GetReviewsPayload = serde_json::from_str(raw_payload)?;
            let reviews = db.get_reviews(payload.employee_id).unwrap();
            serde_json::to_string(&reviews)
        },
        ("POST", "/add_review") => {
            let payload: AddReviewPayload = serde_json::from_str(raw_payload)?;
            db.add_review(payload.employee_id, payload.review, payload.score);
            Ok("success".to_string())
        },
        // ("POST", "/update_review") => {
        //     db.update_review();
        //     Ok("success".to_string())
        // },
        // ("GET", "/get_reviews_requiring_feedback") => {
        //     let reviews = db.get_reviews_requiring_feedback().unwrap();
        //     serde_json::to_string(&reviews)
        // },
        // ("POST", "/flag_for_feedback") => {
        //     db.flag_for_feedback();
        //     Ok("success".to_string())
        // },
        // ("POST", "/submit_feedback") => {
        //     db.submit_feedback();
        //     Ok("success".to_string())
        // },
        _ => { Ok("page not found".to_string()) }
    }
}