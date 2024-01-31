pub fn or(a: bool, b: bool) -> bool{
    let result = 0;
    match (a,b){
        (1,1)=> result = 1,
        (1,0)=> result = 1,
        (0,1)=> result = 1,
        (0,0)=> result = 0,
        _ => ()
    }
    return result;
}

pub fn and(a: bool, b: bool) -> bool{
    let result = 0;
    match (a,b){
        (1,1)=> result = 1,
        (1,0)=> result = 0,
        (0,1)=> result = 0,
        (0,0)=> result = 0,
        _ => ()
    }
    return result;
}