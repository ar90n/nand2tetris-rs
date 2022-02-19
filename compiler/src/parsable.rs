use anyhow::*;

use super::foundation::*;
use super::token::Token;

pub(crate) fn token_parser(
    target: Token,
) -> impl Fn(&[Token]) -> Result<(Box<PlaceHolder>, &[Token])> {
    move |tokens: &[Token]| {
        if tokens.is_empty() {
            return Err(anyhow!("tokens are empty"));
        }

        let (token, rem) = tokens.split_at(1);
        if token[0] == target {
            Ok((Box::new(PlaceHolder { token: target.clone() }), rem))
        } else {
            Err(anyhow!("invalid token: {:?}", token))
        }
    }
}

pub(crate) fn option_parser<T>(
    parser: impl Fn(&[Token]) -> Result<(Box<T>, &[Token])>,
) -> impl Fn(&[Token]) -> Result<(Box<Option<Box<T>>>, &[Token])> {
    move |tokens: &[Token]| {
        if let anyhow::Result::Ok((t, tokens)) = parser(tokens) {
            Ok((Box::new(Some(t)), tokens))
        } else {
            Ok((Box::new(None), tokens))
        }
    }
}

pub(crate) fn repeat_parser<T>(
    parser: impl Fn(&[Token]) -> Result<(Box<T>, &[Token])>,
) -> impl Fn(&[Token]) -> Result<(Box<Vec<Box<T>>>, &[Token])> {
    move |tokens: &[Token]| {
        let mut result = vec![];
        let mut tokens = tokens;
        while let anyhow::Result::Ok((item, rem)) = parser(tokens) {
            result.push(item);
            tokens = rem;
        }
        Ok((Box::new(result), tokens))
    }
}

//pub(crate) fn repeat_parser2<T>(
//    parser: impl Fn(&[Token]) -> Result<(Box<T>, &[Token])>,
//) -> impl Fn(&[Token]) -> Result<(Box<Collection<T>>, &[Token])> {
//    move |tokens: &[Token]| {
//        let mut result = vec![];
//        let mut tokens = tokens;
//        while let anyhow::Result::Ok((item, rem)) = parser(tokens) {
//            result.push(item);
//            tokens = rem;
//        }
//        Ok((Box::new(result), tokens))
//    }
//}

pub(crate) fn seq2_parser<T, U>(
    parser_t: impl Fn(&[Token]) -> Result<(Box<T>, &[Token])>,
    parser_u: impl Fn(&[Token]) -> Result<(Box<U>, &[Token])>,
) -> impl Fn(&[Token]) -> Result<(Box<(Box<T>, Box<U>)>, &[Token])> {
    move |tokens: &[Token]| {
        let (t, tokens) = parser_t(tokens)?;
        let (u, tokens) = parser_u(tokens)?;
        Ok((Box::new((t, u)), tokens))
    }
}

pub(crate) fn drop1_parser<T, U>(
    parser_t: impl Fn(&[Token]) -> Result<(Box<T>, &[Token])>,
    parser_u: impl Fn(&[Token]) -> Result<(Box<U>, &[Token])>,
) -> impl Fn(&[Token]) -> Result<(Box<U>, &[Token])> {
    move |tokens: &[Token]| {
        let (_, tokens) = parser_t(tokens)?;
        let (u, tokens) = parser_u(tokens)?;
        Ok((u, tokens))
    }
}

pub(crate) fn seq3_parser<T, U, V>(
    parser_t: impl Fn(&[Token]) -> Result<(Box<T>, &[Token])>,
    parser_u: impl Fn(&[Token]) -> Result<(Box<U>, &[Token])>,
    parser_v: impl Fn(&[Token]) -> Result<(Box<V>, &[Token])>,
) -> impl Fn(&[Token]) -> Result<(Box<(Box<T>, Box<U>, Box<V>)>, &[Token])> {
    move |tokens: &[Token]| {
        let (t, tokens) = parser_t(tokens)?;
        let (u, tokens) = parser_u(tokens)?;
        let (v, tokens) = parser_v(tokens)?;
        Ok((Box::new((t, u, v)), tokens))
    }
}

pub(crate) fn seq4_parser<T, U, V, W>(
    parser_t: impl Fn(&[Token]) -> Result<(Box<T>, &[Token])>,
    parser_u: impl Fn(&[Token]) -> Result<(Box<U>, &[Token])>,
    parser_v: impl Fn(&[Token]) -> Result<(Box<V>, &[Token])>,
    parser_w: impl Fn(&[Token]) -> Result<(Box<W>, &[Token])>,
) -> impl Fn(&[Token]) -> Result<(Box<(Box<T>, Box<U>, Box<V>, Box<W>)>, &[Token])> {
    move |tokens: &[Token]| {
        let (t, tokens) = parser_t(tokens)?;
        let (u, tokens) = parser_u(tokens)?;
        let (v, tokens) = parser_v(tokens)?;
        let (w, tokens) = parser_w(tokens)?;
        Ok((Box::new((t, u, v, w)), tokens))
    }
}

pub(crate) fn surround_parser<T>(
    parser: impl Fn(&[Token]) -> Result<(Box<T>, &[Token])>,
    surround_left: Token,
    surround_right: Token,
) -> impl Fn(&[Token]) -> Result<(Box<T>, &[Token])> {
    let parser_left = token_parser(surround_left);
    let parser_right = token_parser(surround_right);
    move |tokens: &[Token]| {
        let (_, tokens) = parser_left(tokens)?;
        let (t, tokens) = parser(tokens)?;
        let (_, tokens) = parser_right(tokens)?;
        Ok((t, tokens))
    }
}
