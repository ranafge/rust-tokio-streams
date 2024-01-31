

use futures::{stream::{self, StreamExt}, Stream};
use tokio::process::Command;


#[tokio::main]
async fn main() {
    let mut stream2 = stream::iter(2..=50).map(|x| if x < 5 { Some(x) } else { None });

    // stream is asynchronous sequencial stream behave like iterator
    while let Some(x) = stream2.next().await {
        match x {
            Some(v) => println!("the value is : {}", v),
            None => break,
        };
    }

    let filtered = futures::stream::iter(1..=100)
    .filter_map(|x| async move{
        if x % 2 == 0 {
            Some(x)
        }else {
            None
        }
    }).collect::<Vec<_>>().await;

    println!("{:?}", filtered);


    let r = command_out().await;
    match r {
        Ok(_) => println!("OK"),
        Err(e) => println!("STDOUT ERROR : {}",e)
    }

}



async fn command_out() -> std::io::Result<()> {
    let out = Command::new("echo") 
    .arg("hellow")
    .arg("world")
    .output()
    .await?;
    let s = String::from_utf8_lossy(&out.stdout);
    println!("{}", s);
    Ok(())
}