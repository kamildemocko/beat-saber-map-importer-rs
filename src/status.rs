use chrono::Local;

#[derive(Default)]
pub struct Status {
    rows: Vec<String>
}

impl Status {
    pub fn new() -> Self {
        Self { rows: Vec::with_capacity(5) }
    }

    pub fn get_string(&self) -> String {
        self.rows.join("\n")
    }

    pub fn insert_status(&mut self, status: String) {
        if self.rows.len() > 4 {
            self.rows.remove(4);
        }

        let now = Local::now().format("%H:%M:%S").to_string();

        self.rows.insert(0, format!("{} - {}", now, status));

    }
}