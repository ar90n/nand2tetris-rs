mod Bit;
mod Register;
mod PC;
mod RAM8;
mod RAM64;
mod modules;

#[cfg(test)]
mod tests{
    use super::modules::*;

    #[test]
    fn test_dff() {
        let mut dff = DFF::new();

        dff.in_ = false;
        dff.prop();
        dff.posedge_clk();
        dff.prop();
        assert_eq!(dff.out, false);

        dff.prop();
        dff.posedge_clk();
        dff.prop();
        assert_eq!(dff.out, false);

        dff.in_ = true;
        dff.prop();
        assert_eq!(dff.out, false);

        dff.posedge_clk();
        assert_eq!(dff.out, false);

        dff.prop();
        assert_eq!(dff.out, true);
    }
}