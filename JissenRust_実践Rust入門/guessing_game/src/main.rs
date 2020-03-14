extern crate rand;

use std::io;//入出力を行う
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("数当てゲーム");

    let secret_number = rand::thread_rng().gen_range(1,101); //乱数生成
    
    //println!("秘密の数字は: {}", secret_number);
    
    loop {
        println!("数字を入力してください");
        
        let mut guess = String::new();//左辺は変数束縛を作るlet文
        
        io::stdin().read_line(&mut guess)//std::io::stdin()の略,
            .expect("読み込みを失敗しました。");//標準入力read_line()

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => continue,
        };  //シャドーイング
            // trimで\nを除去 // parseで文字列を整数へ
            
        println!("あなたの予測値: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less         => println!("小さすぎ!"),
            Ordering::Greater      => println!("大きすぎ!"),
            Ordering::Equal        => {
                println!("You win!");
                break;
            }   
        }
    }
}
