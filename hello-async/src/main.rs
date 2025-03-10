use trpl::Html;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    trpl::run(async {
        let url1 = page_title(&args[1]);
        let url2 = page_title(&args[2]);
        let (url, maybe_title) = match trpl::race(url1, url2).await {
            trpl::Either::Right(right) => right,
            trpl::Either::Left(left) => left,
        };
        println!("{url} returned first");

        match maybe_title {
            Some(title) => println!("The title for the {url} is {title}"),
            None => println!("{url} has no title"),
        }
    })
}
async fn page_title(url: &str) -> (&str, Option<String>) {
    let text = trpl::get(url).await.text().await;
    let title = Html::parse(&text)
        .select_first("title")
        .map(|title_element| title_element.inner_html());
    (url, title)
}
