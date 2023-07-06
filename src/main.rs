use error_chain::error_chain;
use select::document::Document;
use select::predicate::Name;

error_chain! {
    foreign_links{
        ReqError(reqwest::Error);
        IoError(std::io::Error);
    }
}
#[tokio::main]
async fn main() -> Result<()> {
    let res = reqwest::get("https://www.zubzz.com/recommendations")
        .await?
        .text()
        .await?;

    // let res = reqwest::get("https://www.zubzz.com/menus")
    //     .await?
    //     .text()
    //     .await?;

    // println!("restaurants on web page 'zubzz.com/menus' :");

    // Document::from(res.as_str()).find(Name("li")).for_each(|n| {
    //     n.find(Name("span"))
    //         .filter_map(|n| match n.attr("class") {
    //             Some("sqsrte-text-color--accent") => Some(n),
    //             _ => None,
    //         })
    //         .for_each(|n| {
    //             println!("{}", n.text());
    //         });
    // });
    // println!("restaurants on web page 'zubzz.com/recommendations' :");

    Document::from(res.as_str()).find(Name("li")).for_each(|n| {
        n.find(Name("span"))
            .filter_map(|n| match n.attr("class") {
                Some("accordion-item__title") => Some(n),
                _ => None,
            })
            .for_each(|n| {
                println!("{}", n.text());
            });
        n.find(Name("span"))
            .filter_map(|n| match n.attr("style") {
                Some("text-decoration:underline") => Some(n),
                _ => None,
            })
            .for_each(|n| {
                println!("{}", n.text());
            });
    });

    Ok(())
}
