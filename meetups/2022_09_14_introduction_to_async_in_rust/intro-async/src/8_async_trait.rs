use async_trait::async_trait;

#[async_trait]
trait File {
    async fn read(&self);
}

struct TextFile;

#[async_trait]
impl File for TextFile {
    async fn read(&self){}    
}

fn main() {

}