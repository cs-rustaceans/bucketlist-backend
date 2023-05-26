use crate::db::model::user::{RoleEnum, User};
use actix_web::guard::GuardContext;

pub fn admin_guard(ctx: &GuardContext) -> bool {
  let req_data = ctx.req_data();
  let user_optional = req_data.get::<User>();
  if user_optional.is_none() {
    return false;
  }
  if user_optional.unwrap().role != Into::<&str>::into(RoleEnum::Admin) {
    return false;
  }
  return true;
}
