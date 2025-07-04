use actix_web::web::{self, ServiceConfig};

mod login;
mod shifts;
mod users;

pub fn configure_endpoints(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(login::login)
            .service(login::register)
            .service(login::me)
            .service(users::get_user)
            // NOTE: This needs to be first
            .service(shifts::get_shift_range)
            .service(shifts::get_shift)
            .service(shifts::get_shift_users),
    );
}
