use nom::{
    bytes::complete::tag, character::complete::alpha1, character::complete::char,
    character::complete::multispace0 as sp, sequence::tuple, IResult,
};

// func $add(%x int, %y int) int {
//     %z = add int %x, %y
// 	ret int %z;
// }

enum Operand<'a> {
    Var(&'a str),
}

fn typ(input: &str) -> IResult<&str, &str> {
    let (input, typ) = tag("int")(input)?;
    let (input, _) = sp(input)?;
    Ok((input, typ))
}

fn var(input: &str) -> IResult<&str, &str> {
    let (input, (_, var_literal)) = tuple((char('%'), alpha1))(input)?;
    let (input, _) = sp(input)?;
    Ok((input, var_literal))
}

#[test]
fn var_test() {
    let result = var("%xyz rest");
    assert!(result.is_ok());
    let (rest, var_lit) = result.unwrap();
    assert_eq!("rest", rest);
    assert_eq!("xyz", var_lit);
}

#[test]
fn typ_test() {
    let result = typ("int %x");
    assert!(result.is_ok());
    let (rest, typ_lit) = result.unwrap();
    assert_eq!("%x", rest);
    assert_eq!("int", typ_lit);
}
