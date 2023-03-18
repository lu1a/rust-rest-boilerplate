mod db {
    use std::sync::Mutex;
    use postgres::{Client, NoTls};

    lazy_static! {
        static ref CLIENT: Mutex<Client> = Mutex::new(establish_connection());
    }

    fn establish_connection() -> Client {
        let database_url = "postgres://user:password@localhost/mydatabase";
        let client = Client::connect(database_url, NoTls).expect("Failed to connect to the database");
        client
    }

    pub fn get_users() -> Result<Vec<User>, Box<dyn Error>> {
        let mut client = CLIENT.lock().unwrap();
    
        let max_retries = 10;
        let retry_interval = Duration::from_secs(5);
    
        for i in 0..max_retries {
            match client.query("SELECT * FROM users", &[]) {
                Ok(rows) => {
                    // process the query results and return the list of users
                    let users = process_rows(rows);
                    return Ok(users);
                }
                Err(e) => {
                    if i < max_retries - 1 {
                        // if the query fails, sleep for a bit and try again
                        std::thread::sleep(retry_interval);
                        *client = establish_connection();
                    } else {
                        return Err(format!("Failed to execute query after {} retries: {}", max_retries, e).into());
                    }
                }
            }
        }
        unreachable!();
    }
}
