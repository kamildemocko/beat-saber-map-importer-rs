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
            self.rows.remove(0);
        }

        let now = Local::now().format("%H:%M:%S").to_string();
        let status_clean = match status.len() {
            len if len <= 90 => status,
            _ => format!("{}..", status[..88].to_string()),
        };
        let msg = format!("{} » {}", now, status_clean);

        self.rows.push(msg);
    }
}