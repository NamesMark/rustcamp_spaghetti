use nutype::nutype;
use regex;
use lazy_static::lazy_static;

fn main() {
    ()
}

struct Server {
    //ip: server::IpAddr,
    name: server::Name,
    //cron_date: server::CronDate,
}

mod server {
    use super::*;
    // struct IpAddr(std::net::IpAddr);

    // impl FromStr for IpAddr {
        
    // }

    //SRV_main_01
    #[nutype(
        sanitize(trim),
        validate(regex = r#"^[A-Z]{3}_.+_\d{2}$"#)
    )]
    pub struct Name(String);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn server_name_validation() {
        let test_scenarios = [("SRV_main_01", true), ("bad_one", false), ("SRV_main_0", false), ("_main_01", false)];
        for (test_name, expected) in test_scenarios {
            assert_eq!(server::Name::try_new(test_name).is_ok(), expected);
        }
    }

    #[test]
    fn password_validation() {

    }
}