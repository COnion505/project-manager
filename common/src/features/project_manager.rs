use crate::features::db_handler::DatabaseConnection;
use crate::features::project_setting::PMSetting;
pub struct ProjectManager{
    pub conn: DatabaseConnection,
    pub setting: PMSetting,
}

impl ProjectManager {
    pub fn create(self, project_name: &str) {
        println!("create {}", project_name);
    }

    pub fn info(self, project_name: &str) {

        println!("show info of {}", project_name);
    }

    pub fn list(self) {
        println!("show project list");
    }

    pub fn delete(self, project_name: &str) {

        println!("delete {}", project_name);
    }
}