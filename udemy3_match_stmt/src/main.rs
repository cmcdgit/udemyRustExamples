#![allow(dead_code)]
#![allow(unused_variables)]
// CONTROL FLOW

fn if_statement()
{
    let temp = 35;
    if temp > 30
    {
        println!("It's really hot outside");
        println!("{}", temp > 30);
    }
    else if temp < 10 
    {
        println!("really cold");
    }
    else 
    {    
        println!("The temperature is good");   
    }

    let day = if temp > 20 {"sunny"} else {"cloudy"};   //assign value to a variable inline
    println!("Today is {}", day);

    println!("#1 - it is {}",
        if temp > 20 {"hot"} else if temp < 10 {"cold"} else {"ok"});

    println!("#2 - it is {}",
        if temp > 20 {
            if temp > 30 {"very hot"} else {"hot"}
        }
        else if temp < 10 {"cold"} else {"very cold"});
}


fn while_loops()
{
    let mut x = 1;

    while x < 1000
    {
        x *= 2;

        if x == 64 { continue; }    // continue - return to the start of the loop and continue

        println!("x = {}", x);
    }

    let mut y = 1;
    loop 
    {
        y *= 2;
        println!("y = {}", y);

        if y == 1<<10 { break };
    }

}

fn for_loop()
{
    for x in 1..11                      // range  1 - 10
    {
        if x == 3 { continue; };
        if x == 8 { break; };
        println!("x = {}", x);
    }

    for (pos, y) in (30..41).enumerate()
    {
        println!("{} : {}", pos, y);
    }
}

fn match_statement()
{
    let country_code = 1000;     //country codes 1 - 999

    let country = match country_code
    {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        353 => "Ireland",
        1..=999 => "Unknown",
        _ => "Invalid"

    };

    println!("the country with code {} is {}", country_code, country);
}

fn main() 
{
    match_statement()
    // for_loop()
    // while_loops();
    // if_statement();
}
