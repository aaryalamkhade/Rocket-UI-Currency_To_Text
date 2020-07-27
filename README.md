# Rocket-Currency_To_Text
Rocket is a web framework for Rust that makes it simple to write fast, secure web applications without sacrificing flexibility, usability, or type safety <br>
**Documentation** <br>
Rocket => https://rocket.rs/ <br>
Rust => https://doc.rust-lang.org/stable/rust-by-example/index.html   <br>
Tera templates =>1.https://tera.netlify.app/docs/  2.https://docs.rs/crate/tera/0.7.2   <br>     
                 
# Usage
1.Rust <br>
2.rocket <br>
3.Tera templates <br>
4.html <br>
Rocket makes abundant use of Rust's syntax extensions and other advanced, unstable features.Thus you need to install rust nightly.<br>
**Commands**
Open command line and Enter this command.<br>
```
1.rustup default nightly <br>
2.rustup override set nightly <br>
3.rustup update <br>
4.cargo update <br>
```

**Rocket** 
```
#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open("templates/base.html").ok()
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, Amount])
        .attach(Template::fairing())
}
```
 Run main.rs using cargo run.  <br>
 Visit http://localhost:8000 to see Rocket application in action!   <br>
 localhost:8000 will trigger the html page which will take amount and langauge as an input and it will give the currency in words.  <br>
 ![Screenshot (172)](https://user-images.githubusercontent.com/49730497/88544631-53724d80-d037-11ea-909f-e8b3f04be7d5.png)
![Screenshot (173)](https://user-images.githubusercontent.com/49730497/88544700-69800e00-d037-11ea-98e0-19823df8a57a.png)
![Screenshot (174)](https://user-images.githubusercontent.com/49730497/88544730-769cfd00-d037-11ea-86fd-9555ce8be591.png)


