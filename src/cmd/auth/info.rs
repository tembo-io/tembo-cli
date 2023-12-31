//! auth info command

use crate::cli::config::Config;
use crate::Result;
use clap::{ArgMatches, Command};
use dateparser::parse;
use jwt::Claims;
use jwt::Header;
use jwt::Token;
use simplelog::*;

// example usage: tembo auth info
pub fn make_subcommand() -> Command {
    Command::new("info").about("Command used to login/authenticate")
}

pub fn execute(args: &ArgMatches) -> Result<()> {
    let config = Config::new(args, &Config::full_path(args));
    let jwt = config.jwt.unwrap();

    if jwt.is_empty() {
        info!("No auth info, to authenticate, run tembo auth login");
    } else {
        let _ = print_jwt_info(&jwt);
    }

    Ok(())
}

// NOTE: uses println rather than logging intentionally
fn print_jwt_info(jwt: &str) -> Result<()> {
    println!("Tembo auth information:");

    let token: Token<Header, Claims, _> = Token::parse_unverified(jwt)?;
    let claims = token.claims();
    let registered = &claims.registered;
    println!("- issuer: {}", &registered.issuer.clone().unwrap());

    let expiration = &registered.expiration.unwrap();
    let human_expire = parse(&expiration.to_string());
    println!("- expiration: {}", human_expire.unwrap().to_rfc2822());

    Ok(())
}
