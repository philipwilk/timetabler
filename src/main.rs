use std::env;
use fancy_regex::Regex; 
use lazy_static::lazy_static;  
use curl::easy::{Easy2, Handler, WriteError};
struct Collector(Vec<u8>);

fn main() {
    let password: String;
    let username: String;
    let week: i32;

    match env::var("ACC_USERNAME") {
        Ok(a) => username = a,
        Err(_e) => {println!("username not set"); panic!();},
    };
    match env::var("ACC_PASSWORD") {
        Ok(a) => password = a,
        Err(_e) => {println!("password not set"); panic!();},
    };
    match env::var("ACC_WEEK") {
        Ok(a) => match a.parse::<i32>() {
            Ok(b) => week = b,
            Err(_e) => {println!("week not set or invalid. defaulting to now."); week=0},
        },
        Err(_e) => {println!("week not set or invalid. defaulting to now."); week=0},
    };
    // --- What we doin? ---
    // Get JSESSIONID and random-string cookie using this request, save in cookie jar (dont eat any!)

    const URL_INITIALISE_COOKIES: &str = "https://auth.psc.ac.uk/nidp/app/login?id=5&sid=10&option=credential&sid=10&target=https%3A%2F%2Fintranet.psc.ac.uk%2F";
    impl Handler for Collector {
        fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
            self.0.extend_from_slice(data);
            Ok(data.len())
        }
    }
    let mut initialise_cookies = Easy2::new(Collector(Vec::new()));
    initialise_cookies.url(URL_INITIALISE_COOKIES).unwrap();
    initialise_cookies.cookie_jar("./cookies.txt").unwrap();
    initialise_cookies.cookie_file("./cookies.txt").unwrap();
    initialise_cookies.post(true).unwrap();

    initialise_cookies.perform().unwrap();
    assert_eq!(initialise_cookies.response_code().unwrap(), 200);
    let contents = String::from_utf8_lossy(&initialise_cookies.get_ref().0);

    let mut init_lines = Vec::new();
    for i in contents.lines() {
        init_lines.push(i);
    }

    lazy_static! {
       static ref MATCH_S: Regex = Regex::new("(?<=k\":\")[A-Za-z0-9=]+(?=\")\\").unwrap();
    }

    let s = MATCH_S.captures(init_lines[74]).unwrap().unwrap();
    println!("S is {:?}", s);
    
    println!("{}", init_lines[74]);
}
