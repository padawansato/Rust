fn rebind(){
    let sum = 0;
    println!("sum : {}",sum);
    for i in 0..10 {
        // 新しい束縛を作っているので上の束縛には影響がない。
        let sum = sum + i;
    }
    println!("rebind sum : {}", sum); // => 0
}

fn reassign() {
    let mut sum = 0;
    println!("sum : {}", sum);
    for i in 0..10 {
        // 上の束縛の値を書き換える。 
        sum = sum + i;
    }
    println!("reassign sum : {}", sum);
}

fn rebind2(){
    let y = 0;
    println!("let y : {}", y);
    let i = 10;
    let y = y + i;
        // let y = y + 1;
    println!("letyで新しい束縛を作った結果\ny : {}", y);
}

fn reassign2(){
    let mut y = 0;
    println!("let mut y : {}", y);
    y = y + 1;
    println!("y=y+1でyを再代入した結果\ny : {}", y);
}


fn main(){
    rebind();
    reassign();
    rebind2();
    reassign2();
}