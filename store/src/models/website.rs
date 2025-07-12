use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

use crate::store::Store;


#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::website)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Website {
    pub id: String,
    pub url: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub user_id: String
}
impl Store {
    pub fn create_website(&mut self, user_id: String, url: String) -> Result<Website, diesel::result::Error> {

        let id = Uuid::new_v4();
        let website_record = Website {
            user_id,
            url,
            id: id.to_string(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc()
        };

        let website = diesel::insert_into(crate::schema::website::table).
        values(&website_record).returning(Website::as_returning())
        .get_result(&mut self.conn)?;

        Ok(website)
    }

    pub fn get_website(&mut self, input_id: String, input_user_id: String) -> Result<Website, diesel::result::Error> {
        use crate::schema::website::dsl::*;

        let website_result = website
        .filter(id.eq(input_id))
        .filter(user_id.eq(input_user_id))
        .select(Website::as_select())
        .first(&mut self.conn)?;

        Ok(website_result)
    }
}
