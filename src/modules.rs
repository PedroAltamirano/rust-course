mod mod_auth;

fn main() {
  let mut user = mod_auth::User::new("jeremy", "super-secret");

  println!("The username is: {}", user.get_username());
  user.set_password("even-more-secret");
}
