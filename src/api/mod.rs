use actix_web::web::{self, ServiceConfig};

mod login;
mod shifts;
mod users;

pub fn configure_endpoints(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(login::login_42)
            .service(login::callback_42)
            .service(login::me)
            .service(users::get_user)
            // NOTE: This needs to be first
            .service(shifts::get_shift_users)
            .service(shifts::get_shift_week)
            .service(shifts::register_to_shift)
            .service(shifts::register_user_to_shift)
            .service(shifts::unregister_from_shift)
            .service(shifts::unregister_user_from_shift),
    );
}
