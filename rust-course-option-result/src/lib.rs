//! `Option<T>`와 `Result<T, E>` 강의의 종합 실습에 사용하는 함수입니다.

use std::{error::Error, fmt, num::ParseIntError};

/// 전체 값이 0이 아닐 때만 부분이 차지하는 비율을 백분율로 계산합니다.
pub fn percentage(part: f64, total: f64) -> Option<f64> {
    (total != 0.0).then(|| part * 100.0 / total)
}

/// `@`를 포함한 첫 문자열을 찾습니다.
pub fn first_email<'a>(values: &'a [&'a str]) -> Option<&'a str> {
    values.iter().copied().find(|value| value.contains('@'))
}

/// `KEY=VALUE` 형식의 문자열에서 `VALUE`를 `u16`으로 읽습니다.
pub fn parse_setting(input: &str) -> Option<u16> {
    let (_, value) = input.split_once('=')?;
    value.trim().parse().ok()
}

/// 모든 문자열을 정수로 변환할 수 있을 때만 목록을 반환합니다.
pub fn parse_all(inputs: &[&str]) -> Option<Vec<i32>> {
    inputs.iter().map(|input| input.parse().ok()).collect()
}

#[derive(Debug, PartialEq)]
pub enum ExpressionError {
    InvalidFormat,
    InvalidNumber,
    DivisionByZero,
    Overflow,
}

impl fmt::Display for ExpressionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::InvalidFormat => "LEFT/RIGHT 형식이 아닙니다",
            Self::InvalidNumber => "피연산자가 정수가 아닙니다",
            Self::DivisionByZero => "0으로 나눌 수 없습니다",
            Self::Overflow => "계산 결과가 i32 범위를 벗어납니다",
        };
        f.write_str(message)
    }
}

impl Error for ExpressionError {}

pub fn evaluate_division(input: &str) -> Result<i32, ExpressionError> {
    let (left, right) = input
        .split_once('/')
        .ok_or(ExpressionError::InvalidFormat)?;
    let left = left
        .trim()
        .parse::<i32>()
        .map_err(|_| ExpressionError::InvalidNumber)?;
    let right = right
        .trim()
        .parse::<i32>()
        .map_err(|_| ExpressionError::InvalidNumber)?;
    if right == 0 {
        return Err(ExpressionError::DivisionByZero);
    }
    left.checked_div(right).ok_or(ExpressionError::Overflow)
}

pub fn parse_ports(input: &str) -> Result<Vec<u16>, ParseIntError> {
    input.split(',').map(|port| port.trim().parse()).collect()
}

#[derive(Debug, PartialEq)]
pub enum PaymentError {
    ZeroAmount,
    Overflow,
}

impl fmt::Display for PaymentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::ZeroAmount => "결제 금액은 0보다 커야 합니다",
            Self::Overflow => "결제 합계가 u32 범위를 벗어납니다",
        })
    }
}

impl Error for PaymentError {}

pub fn total_payments(amounts: &[u32]) -> Result<u32, PaymentError> {
    let mut total = 0_u32;
    for &amount in amounts {
        if amount == 0 {
            return Err(PaymentError::ZeroAmount);
        }
        total = total.checked_add(amount).ok_or(PaymentError::Overflow)?;
    }
    Ok(total)
}

/// 포트 설정이 있으면 포트 번호로 변환하고, 없으면 `Ok(None)`을 반환합니다.
pub fn optional_port(input: Option<&str>) -> Result<Option<u16>, ParseIntError> {
    input.map(str::parse).transpose()
}
