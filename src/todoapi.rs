use sqlite::{Connection, State};
use std::{path::PathBuf, sync::Arc};
pub type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;
const SQL_TABLE_INIT: &str = "
DROP TABLE IF EXISTS todo;
CREATE TABLE todo (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    checked BOOLEAN DEFAULT FALSE,
    description VARCHAR(255) NOT NULL
);
";

pub struct TodoPoint {
    pub finished: bool,
    pub description: String,
    pub sql_id: i64,
    connection: Arc<Connection>,
}
impl PartialEq for TodoPoint {
    fn eq(&self, other: &Self) -> bool {
        self.sql_id == other.sql_id 
    }
}
impl TodoPoint {
    fn new(
        finished: bool,
        sql_id: i64,
        description: String,
        cloned_arc_connection: Arc<Connection>,
    ) -> Self {
        Self {
            finished,
            description,
            sql_id,
            connection: cloned_arc_connection,
        }
    }
    pub fn description(&mut self, to: String) -> BoxResult<&mut Self> {
        if self.description.to_lowercase() != to.to_lowercase() {
        let mut statement = self.connection.prepare("UPDATE todo SET description=? WHERE id=?;")?;
        statement.bind((1, to.as_str()))?;
        statement.bind((2, self.sql_id))?;
        statement.next()?;
        self.description = to;
        }

        Ok(self)
    }
    pub fn finished(&mut self, to: bool) -> BoxResult<&mut Self> {
        if to != self.finished {
            let mut statement = self
                .connection
                .prepare("UPDATE todo SET checked=? WHERE id=?;")?;
            statement.bind((1, if to == true { 1 } else { 0 }))?;
            statement.bind((2, self.sql_id))?;
            statement.next()?;
            self.finished = to;
        }
        Ok(self)
    }
}

pub struct TodoApp {
    pub todopoints: Vec<TodoPoint>,
    conn: Arc<Connection>,
}
impl TodoApp {
    pub fn from_existing_file(path: PathBuf) -> BoxResult<Self> {
        let connection = Arc::new(sqlite::open(path)?);
        let mut points: Vec<TodoPoint> = vec![];
        {
            let mut query = connection.prepare("SELECT id, checked, description FROM todo;")?;
            while let Ok(State::Row) = query.next() {
                let todop = TodoPoint::new(
                    query.read::<i64, _>("checked")? > 0,
                    query.read::<i64, _>("id")?,
                    query.read::<String, _>("description")?,
                    Arc::clone(&connection),
                );
                points.push(todop);
            }
        }
        Ok(Self {
            todopoints: points,
            conn: connection,
        })
    }
    pub fn setup(path_destination: PathBuf) -> BoxResult<Self> {
        let connection = sqlite::open(path_destination)?;
        connection.execute(SQL_TABLE_INIT)?;
        Ok(Self {
            todopoints: vec![],
            conn: Arc::new(connection),
        })
    }
    pub fn new_todo_point(&mut self, description: String, checked: bool) -> BoxResult<&TodoPoint> {
        let mut query = self
            .conn
            .prepare("INSERT INTO todo(description, checked) VALUES(?, ?);")?;
        query.bind((1, description.as_str()))?;
        query.bind((2, if checked == true { 1 } else { 0 }))?;
        query.next()?; // to execute
        let last = self.todopoints.last();
        self.todopoints.push(TodoPoint::new(
            checked,
            if last.is_some() {
                last.unwrap().sql_id + 1
            } else {
                1
            },
            description,
            Arc::clone(&self.conn),
        ));
        Ok(self.todopoints.last().unwrap())
    }
    pub fn print_all_todos(&self) -> BoxResult<Vec<String>> {
        let mut result: Vec<String> = vec![];
        for todo in &self.todopoints {
            result.push(
                format!("{}. [{}] '{}'",
                todo.sql_id,
                if todo.finished {'x'} else {' '},
                todo.description
            ));
        }

        Ok(result)
    }
    pub fn remove_point_by_id(&mut self, id: i64) -> BoxResult<&mut Self> {
        let index = self.todopoints
                        .iter()
                        .position(|x| x.sql_id == id);
        if let Some(index) = index {
            let mut query = self.conn.prepare("DELETE FROM todo WHERE id = ?;")?;
            query.bind((1, id))?;
            query.next()?;
            self.todopoints.remove(index);
        }

        Ok(self)
    }
}
