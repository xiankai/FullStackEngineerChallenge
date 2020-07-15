use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;
use serde::{Serialize, Deserialize};

pub struct Database {
    conn: Connection
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Employee {
    id: i32,
    name: String,
    // is_deleted: bool,
    // is_admin: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Review {
    id: i32,
    employee_id: i32,
    review: String,
    score: i32,
    created: String,
    updated: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Feedback {
    id: i32,
    review_id: i32,
    employee_id: i32,
    feedback: String,
    created: String,
    updated: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReviewRequiringFeedback {
    name: String,
    feedback: String,
}

// Main SQL logic goes here - commented code has not been implemented
impl Database {
    pub fn new() -> Result<Database> {
        let conn = Connection::open("paypay.db")?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS employees (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                is_deleted INTEGER DEFAULT 0,
                is_admin INTEGER DEFAULT 0
            )",
            NO_PARAMS,
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS reviews (
                id INTEGER PRIMARY KEY,
                employee_id INTEGER,
                review TEXT NOT NULL,
                score INTEGER,
                created DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            NO_PARAMS,
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS feedback (
                id INTEGER PRIMARY KEY,
                review_id INTEGER,
                employee_id INTEGER,
                feedback TEXT NOT NULL,
                created DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            NO_PARAMS,
        )?;

        Ok(Database { conn })
    }

    pub fn get_employees(&self) -> Result<Vec<Employee>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name FROM employees
            WHERE is_deleted = 0
            AND is_admin = 0",
        )?;
        let result = stmt.query_map(NO_PARAMS, |row| {
            Ok(Employee {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?;

        let mut employees = Vec::new();
        for row in result {
            employees.push(row.unwrap())
        }
        Ok(employees)
    }

    pub fn add_employee(&self, name: String) -> Result<()> {
        self.conn.execute(
            "INSERT INTO employees (name) VALUES (?1)",
            &[name.to_string()]
        )?;

        Ok(())
    }

    // pub fn update_employee(&self, id: i32, name: String) -> Result<()> {
    //     self.conn.execute(
    //         "UPDATE employees
    //         SET name = (?1)
    //         WHERE id = (?2)",
    //         &[name, id.to_string()]
    //     )?;

    //     Ok(())
    // }

    pub fn delete_employee (&self, employee_id: i32) -> Result<()> {
        self.conn.execute(
            "UPDATE employees
            SET is_deleted = 1
            WHERE id = (?1)",
            &[employee_id.to_string()]
        )?;

        Ok(())
    }

    pub fn get_reviews (&self, employee_id: i32) -> Result<Vec<Review>> {
        let mut stmt = self.conn.prepare(
            "SELECT *
            FROM reviews
            WHERE employee_id = (?1)"
        )?;

        let result = stmt.query_map(&[employee_id.to_string()], |row| {
            Ok(Review {
                id: row.get(0)?,
                employee_id: row.get(1)?,
                review: row.get(2)?,
                score: row.get(3)?,
                created: row.get(4)?,
                updated: row.get(5)?,
            })
        })?;

        let mut reviews = Vec::new();
        for row in result {
            reviews.push(row.unwrap())
        }
        Ok(reviews)
    }

    pub fn add_review (&self, employee_id: i32, review: String, score: i32) -> Result<()> {
        self.conn.execute(
            "INSERT INTO reviews
            (employee_id, review, score)
            VALUES (?1, ?2, ?3)",
            &[employee_id.to_string(), review,score.to_string()]
        )?;

        Ok(())
    }

    // pub fn update_review (&self, review_id: i32, score: i32, feedback: String) -> Result<()> {
    //     self.conn.execute(
    //         "UPDATE reviews
    //         SET feedback = (?1),
    //         SCORE = (?2),
    //         updated = CURRENT_TIMESTAMP
    //         WHERE id = (?3)",
    //         &[feedback, score.to_string(), review_id.to_string()]
    //     )?;

    //     Ok(())
    // }

    // pub fn get_reviews_requiring_feedback (&self, employee_id: i32) -> Result<Vec<ReviewRequiringFeedback>> {
    //     let mut stmt = self.conn.prepare(
    //         "SELECT employees.name, feedback.feedback
    //         FROM reviews
    //         INNER JOIN feedback
    //         ON reviews.id = review_id
    //         INNER JOIN employees
    //         ON feedback.employee_id = employees.id
    //         WHERE feedback.employee_id = (?1)"
    //     )?;

    //     let mut reviews = Vec::new();
    //     stmt.query_map(NO_PARAMS, |row| {
    //         reviews.push(ReviewRequiringFeedback {
    //             name: row.get(0)?,
    //             feedback: row.get(1)?,
    //         });
    //         Ok(())
    //     })?;

    //     Ok(reviews)
    // }

    // pub fn flag_for_feedback(&self, review_id: i32, employee_id: i32) -> Result<()> {
    //     self.conn.execute(
    //         "INSERT INTO feedback
    //         SET review_id = (?1),
    //         employee_id = (?2)",
    //         &[review_id.to_string(), employee_id.to_string()]
    //     )?;

    //     Ok(())
    // }

    // pub fn submit_feedback (&self, feedback_id: i32,feedback: String) -> Result<()> {
    //     self.conn.execute(
    //         "UPDATE feedback
    //         SET feedback = (?1),
    //         updated = CURRENT_TIMESTAMP
    //         WHERE id = (?2)",
    //         &[feedback, feedback_id.to_string()]
    //     )?;

    //     Ok(())
    // }
}