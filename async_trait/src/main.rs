fn main() {
    {
        // let mut page_title_fut = page_title(url);
        // loop {
        //     match page_title_fut.poll() {
        //         Ready(value) => match page_title {
        //             Some(title) => println!("The title for {url} was {title}"),
        //             None => println!("{url} had no title"),
        //         },
        //     }
        // }
    }
}
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
