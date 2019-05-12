use crate::schema::*;

#[derive(Queryable,Debug,Clone)]
pub struct User {
    pub uid: i32,
    pub wechat_id: String,
    pub phone: String,
    pub personal_info: String,
    pub email: String,
    pub username: String,
    pub verified: bool,
    pub tokens: i32,
    pub user_type: i8
}

#[derive(Insertable,Debug,Clone)]
#[table_name="emtm_users"]
pub struct NewUser<'a> {
    pub wechat_id: &'a str,
    pub phone: &'a str,
    pub personal_info: &'a str,
    pub email: &'a str,
    pub username: &'a str,
    pub verified: bool,
    pub tokens: i32,
    pub user_type: i8
}


#[derive(Queryable,Debug,Clone)]
pub struct Student {
    pub uid: i32,
    pub school_id: i32,
    pub credit: i32,
    pub accepted: i32,
    pub finished: i32,
    pub major: String,
    pub year: i32,
}

#[derive(Insertable,Debug,Clone)]
#[table_name="emtm_students"]
pub struct NewStudent<'a> {
    pub uid: i32,
    pub school_id: i32,
    pub credit: i32,
    pub accepted: i32,
    pub finished: i32,
    pub major: &'a str,
    pub year: i32,
}


#[derive(Queryable,Debug,Clone)]
pub struct Cow {
    pub uid: i32,
    pub company: String,
}


#[derive(Insertable,Debug,Clone)]
#[table_name="emtm_cows"]
pub struct NewCow<'a> {
    pub uid: i32,
    pub company: &'a str,
}
