// quiz1.rs
// This is a quiz for the following sections:
// - Variables
// - Functions

// Mary is buying apples. One apple usually costs 2 Rustbucks, but if you buy
// more than 40 at once, each apple only costs 1! Write a function that calculates
// the price of an order of apples given the quantity bought. No hints this time!

// I AM DONE

// Put your function here!
//Iniciar a função para levar o numero de maças e returnar um inteiro (a nº de maças)
fn calculate_apple_price(a: i32) -> i32 {
    // quant - quantidade comprada
    //prince - preço da compra
    let mut quant = a;
    let mut price = 0;
    //if a < 40 então a função retorn a*2rustabucks
    if quant > 40 {
        price = quant;
        price
    } else {
        price = quant * 2;
        price
    }
}
//else o programa vai retornar a*1rustabucks

// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_apple_price(35);
    let price2 = calculate_apple_price(40);
    let price3 = calculate_apple_price(65);

    assert_eq!(70, price1);
    assert_eq!(80, price2);
    assert_eq!(65, price3);
}
