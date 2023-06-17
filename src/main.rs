mod models;
fn main() {
    let difficulty = 1;
    let mut blockchain = models::blockchain::Blockchain::new(difficulty);

    for i in vec![0; 10] {
        models::blockchain::Blockchain::add_block(&mut blockchain);
    }
}
