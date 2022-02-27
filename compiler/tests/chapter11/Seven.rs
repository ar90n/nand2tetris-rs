use super::compile;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seven() {
        let main = r#"
class Main {

   function void main() {
      do Output.printInt(1 + (2 * 3));
      return;
   }

}"#;

        compile(main, "Seven", "Main".to_string());
    }
}
