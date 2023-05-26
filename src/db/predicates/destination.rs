use crate::db::model::destination::VisibilityEnum;
use crate::db::schema::destinations;
use crate::db::Db;
use diesel::prelude::*;
use diesel::sql_types::Bool;

pub fn available_for_user(
  user_id: u64,
) -> Box<dyn BoxableExpression<destinations::table, Db, SqlType = Bool>> {
  Box::new(
    destinations::dsl::isReviewed.eq(true).and(
      destinations::dsl::ownerId
        .eq(user_id)
        .or(destinations::dsl::visibility.eq(Into::<&str>::into(VisibilityEnum::Public))),
    ),
  )
}
