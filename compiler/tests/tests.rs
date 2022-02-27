mod chapter10 {
    mod ArrayTest_Main;
    mod ArrayTest_MainT;
    mod ExpressionLessSquare_Main;
    mod ExpressionLessSquare_MainT;
    mod ExpressionLessSquare_Square;
    mod ExpressionLessSquare_SquareGame;
    mod ExpressionLessSquare_SquareGameT;
    mod ExpressionLessSquare_SquareT;
    mod Square_Main;
    mod Square_MainT;
    mod Square_Square;
    mod Square_SquareGame;
    mod Square_SquareGameT;
    mod Square_SquareT;
}

mod chapter11 {
    use compiler::dump_vm::*;
    use compiler::parser::parse;
    use compiler::tokenizer::tokenize;

    use std::fs::{create_dir_all, File};
    use std::io::Write;

    fn compile(program: &str, project: &str, name: String) {
        let tokens = tokenize(program).unwrap();
        let class = parse(&tokens).unwrap();

        create_dir_all(format!("out/{}", project)).unwrap();
        let mut file = File::create(format!("out/{}/{}.vm", project, &name)).unwrap();

        let mut context = Context::new(name);
        let commands = class.dump_as_vm(&mut context);
        for c in &commands {
            writeln!(file, "{}", c.dump()).unwrap();
        }
        file.flush().unwrap();
    }

    mod Average;
    mod ComplexArrays;
    mod ConvertToBin;
    mod Pong;
    mod Seven;
    mod Square;
}
