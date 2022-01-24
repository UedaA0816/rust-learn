pub trait Summary {
  fn summarize(&self) -> String{
      // "（もっと読む）"
      String::from("(Read more...)")
  }

  fn summarize_author(&self) -> String;

}

pub trait Display {
  fn show(&self) -> String{
    String::from("(Show more...)")
  }

}

pub struct NewsArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

impl Summary for NewsArticle {
  fn summarize(&self) -> String {
      format!("{}, by {} ({})", self.headline, self.author, self.location)
  }
  fn summarize_author(&self) -> String {
    format!("@{}", self.author)
  }
}

impl Display for NewsArticle {

}

pub struct Tweet {
  pub username: String,
  pub content: String,
  pub reply: bool,
  pub retweet: bool,
}

impl Summary for Tweet {
  fn summarize(&self) -> String {
      format!("{}: {}", self.username, self.content)
  }
  fn summarize_author(&self) -> String {
    format!("@{}", self.username)
  }
}

pub fn traits() {

  let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from(
        // もちろん、ご存知かもしれませんがね、みなさん
        "of course, as you probably already know, people",
    ),
    reply: false,
    retweet: false,
  };

  println!("1 new tweet: {}", tweet.summarize());


  notify(&tweet);


  let article = NewsArticle {
    // ペンギンチームがスタンレーカップチャンピオンシップを勝ち取る！
    headline: String::from("Penguins win the Stanley Cup Championship!"),
    // アメリカ、ペンシルベニア州、ピッツバーグ
    location: String::from("Pittsburgh, PA, USA"),
    // アイスバーグ
    author: String::from("Iceburgh"),
    // ピッツバーグ・ペンギンが再度NHL(National Hockey League)で最強のホッケーチームになった
    content: String::from(
        "The Pittsburgh Penguins once again are the best \
         hockey team in the NHL.",
    ),
  };

  println!("New article available! {}", article.summarize());


  notify(&article);


  
  
  let item = returns_summarizable();
  
  
  println!("New item available! {}", item.summarize());

  
  println!("New article display! {}", article.show());
  
}

pub fn notify(item: &impl Summary) {
  println!("Breaking news! {}", item.summarize());
}

pub fn notify_syntax_sugar<T: Summary>(item: &T) {
  // 速報！ {}
  println!("Breaking news! {}", item.summarize());
}

pub fn notify_multi_trait(item: &(impl Summary + Display)) {
  println!("Breaking news! {}", item.summarize());
}

fn returns_summarizable() -> impl Summary {
  Tweet {
      username: String::from("horse_ebooks"),
      content: String::from(
          "of course, as you probably already know, people",
      ),
      reply: false,
      retweet: false,
  }
}