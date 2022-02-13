use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::alpha1,
    character::complete::char,
    character::complete::multispace0 as sp,
    combinator::{map, value},
    sequence::tuple,
    IResult,
};

mod codegen;

// func $add(%x int, %y int) int {
//  local %z = add int %x, %y;
// 	ret int %z;
// }

#[derive(Debug, PartialEq)]
pub enum Operand<'a> {
    Var(&'a str),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Typ {
    Int,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Instruction<'a> {
    Add(Typ, Operand<'a>, Operand<'a>),
    Ret(Typ, Operand<'a>),
}

#[derive(Debug, PartialEq)]
pub(crate) enum Statement<'a> {
    Local(Operand<'a>, Instruction<'a>),
    Inst(Instruction<'a>),
}

fn typ(input: &str) -> IResult<&str, Typ> {
    let (input, typ) = value(Typ::Int, tag("int"))(input)?;
    let (input, _) = sp(input)?;
    Ok((input, typ))
}

fn operand(input: &str) -> IResult<&str, Operand> {
    map(var, Operand::Var)(input)
}

fn var(input: &str) -> IResult<&str, &str> {
    let (input, (_, var_literal)) = tuple((char('%'), alpha1))(input)?;
    let (input, _) = sp(input)?;
    Ok((input, var_literal))
}

fn add_inst(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("add")(input)?;
    let (input, _) = sp(input)?;
    let (input, typ) = typ(input)?;
    let (input, left_op) = operand(input)?;
    let (input, _) = char(',')(input)?;
    let (input, _) = sp(input)?;
    let (input, right_op) = operand(input)?;
    Ok((input, Instruction::Add(typ, left_op, right_op)))
}

fn ret_inst(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("ret")(input)?;
    let (input, _) = sp(input)?;
    let (input, typ) = typ(input)?;
    let (input, ret_val) = operand(input)?;
    Ok((input, Instruction::Ret(typ, ret_val)))
}

fn local_stmt(input: &str) -> IResult<&str, Statement> {
    let (input, _) = tag("local")(input)?;
    let (input, _) = sp(input)?;
    let (input, opr) = map(var, Operand::Var)(input)?;
    let (input, _) = char('=')(input)?;
    let (input, _) = sp(input)?;
    let (input, inst) = add_inst(input)?;
    Ok((input, Statement::Local(opr, inst)))
}

fn inst(input: &str) -> IResult<&str, Instruction> {
    alt((add_inst, ret_inst))(input)
}

fn inst_stmt(input: &str) -> IResult<&str, Statement> {
    map(inst, Statement::Inst)(input)
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
    assert_eq!(Typ::Int, typ_lit);
}

#[test]
fn add_inst_test() {
    let result = add_inst("add int %x, %y");
    assert!(result.is_ok());
    let (rest, add_inst) = result.unwrap();
    assert_eq!("", rest);
    assert_eq!(
        Instruction::Add(Typ::Int, Operand::Var("x"), Operand::Var("y")),
        add_inst
    );
}

#[test]
fn ret_inst_test() {
    let result = ret_inst("ret int %x");
    assert!(result.is_ok());
    let (rest, ret_inst) = result.unwrap();
    assert_eq!("", rest);
    assert_eq!(Instruction::Ret(Typ::Int, Operand::Var("x")), ret_inst);
}

#[test]
fn inst_test() {
    let result = inst("add int %x, %y");
    assert!(result.is_ok());
    let (rest, add_inst) = result.unwrap();
    assert_eq!("", rest);
    assert_eq!(
        Instruction::Add(Typ::Int, Operand::Var("x"), Operand::Var("y")),
        add_inst
    );

    let result = inst("ret int %x");
    assert!(result.is_ok());
    let (rest, ret_inst) = result.unwrap();
    assert_eq!("", rest);
    assert_eq!(Instruction::Ret(Typ::Int, Operand::Var("x")), ret_inst);
}

#[test]
fn inst_stmt_test() {
    let result = inst_stmt("add int %x, %y");
    assert!(result.is_ok());
    let (rest, add_inst_stmt) = result.unwrap();
    assert_eq!("", rest);
    assert_eq!(
        Statement::Inst(Instruction::Add(
            Typ::Int,
            Operand::Var("x"),
            Operand::Var("y")
        ),),
        add_inst_stmt
    );
}

#[test]
fn local_stmt_test() {
    let result = local_stmt("local %z = add int %x, %y");
    assert!(result.is_ok());
    let (rest, loc) = result.unwrap();
    let add_inst = Instruction::Add(Typ::Int, Operand::Var("x"), Operand::Var("y"));
    assert_eq!("", rest);
    assert_eq!(loc, Statement::Local(Operand::Var("z"), add_inst));
}
