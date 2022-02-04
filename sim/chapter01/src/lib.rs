mod And;
mod Or;
mod Not;
mod Xor;
mod Mux;
mod DMux;
mod And16;
mod Not16;
mod Or16;
mod Mux16;
mod Or8Way;
mod Mux4Way16;
mod Mux8Way16;
mod DMux4Way;
mod DMux8Way;
mod modules;

#[cfg(test)]
mod tests {
    use super::modules::*;

    #[test]
    fn test_nand() {
        let mut nand = Nand::new();

        nand.a = false;
        nand.b = false;
        nand.prop();
        assert_eq!(nand.out, true);

        nand.a = true;
        nand.b = false;
        nand.prop();
        assert_eq!(nand.out, true);

        nand.a = false;
        nand.b = true;
        nand.prop();
        assert_eq!(nand.out, true);

        nand.a = true;
        nand.b = true;
        nand.prop();
        assert_eq!(nand.out, false);
    }
}
