use common::{ ProjectManager, DatabaseConnection, PMSetting };
fn main(){
    common::test();
    let pm = ProjectManager{conn: DatabaseConnection{}, setting: PMSetting{}};
    pm.create("test");
}