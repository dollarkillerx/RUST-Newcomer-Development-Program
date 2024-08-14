use salvo::prelude::*;

#[handler]
pub async fn hello(res: &mut Response) {
    res.render(Text::Plain("Hello World"));
}
