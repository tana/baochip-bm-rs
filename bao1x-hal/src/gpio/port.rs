use bao1x_pac::{
    generic::{Readable, Reg, RegisterSpec, Writable},
    Iox,
};
use pastey::paste;

pub(crate) trait Sealed {}

#[allow(private_bounds)]
pub trait Port: Sealed {
    type AfselLSpec: RegisterSpec<Ux = u32> + Readable + Writable;
    type AfselHSpec: RegisterSpec<Ux = u32> + Readable + Writable;
    type GpioOutSpec: RegisterSpec<Ux = u32> + Readable + Writable;
    type GpioOeSpec: RegisterSpec<Ux = u32> + Readable + Writable;
    type GpioPuSpec: RegisterSpec<Ux = u32> + Readable + Writable;
    type GpioInSpec: RegisterSpec<Ux = u32> + Readable + Writable;
    type SchmSpec: RegisterSpec<Ux = u32> + Readable + Writable;
    type SlewSpec: RegisterSpec<Ux = u32> + Readable + Writable;
    type DrvselSpec: RegisterSpec<Ux = u32> + Readable + Writable;

    fn afsel_l(regs: &Iox) -> &Reg<Self::AfselLSpec>;
    fn afsel_h(regs: &Iox) -> &Reg<Self::AfselHSpec>;
    fn gpio_out(regs: &Iox) -> &Reg<Self::GpioOutSpec>;
    fn gpio_oe(regs: &Iox) -> &Reg<Self::GpioOeSpec>;
    fn gpio_pu(regs: &Iox) -> &Reg<Self::GpioPuSpec>;
    fn gpio_in(regs: &Iox) -> &Reg<Self::GpioInSpec>;
    fn schm(regs: &Iox) -> &Reg<Self::SchmSpec>;
    fn slew(regs: &Iox) -> &Reg<Self::SlewSpec>;
    fn drvsel(regs: &Iox) -> &Reg<Self::DrvselSpec>;
}

macro_rules! def_port {
    ($name:ident, $suffix:literal, $suffix_afsel_l:literal, $suffix_afsel_h:literal) => {
        pub struct $name {}

        impl Sealed for $name {}

        paste! {
            impl Port for $name {
                type AfselLSpec = bao1x_pac::iox::[<sfr_afsel_crafsel $suffix_afsel_l>]::[<SfrAfselCrafsel $suffix_afsel_l Spec>];
                type AfselHSpec = bao1x_pac::iox::[<sfr_afsel_crafsel $suffix_afsel_h>]::[<SfrAfselCrafsel $suffix_afsel_h Spec>];
                type GpioOutSpec = bao1x_pac::iox::[<sfr_gpioout_crgo $suffix>]::[<SfrGpiooutCrgo $suffix Spec>];
                type GpioOeSpec = bao1x_pac::iox::[<sfr_gpiooe_crgoe $suffix>]::[<SfrGpiooeCrgoe $suffix Spec>];
                type GpioPuSpec = bao1x_pac::iox::[<sfr_gpiopu_crgpu $suffix>]::[<SfrGpiopuCrgpu $suffix Spec>];
                type GpioInSpec = bao1x_pac::iox::[<sfr_gpioin_srgi $suffix>]::[<SfrGpioinSrgi $suffix Spec>];
                type SchmSpec = bao1x_pac::iox::[<sfr_cfg_schm_cr_cfg_schmsel $suffix>]::[<SfrCfgSchmCrCfgSchmsel $suffix Spec>];
                type SlewSpec = bao1x_pac::iox::[<sfr_cfg_slew_cr_cfg_slewslow $suffix>]::[<SfrCfgSlewCrCfgSlewslow $suffix Spec>];
                type DrvselSpec = bao1x_pac::iox::[<sfr_cfg_drvsel_cr_cfg_drvsel $suffix>]::[<SfrCfgDrvselCrCfgDrvsel $suffix Spec>];

                #[inline(always)]
                fn afsel_l(regs: &Iox) -> &Reg<Self::AfselLSpec> {
                    regs.[<sfr_afsel_crafsel $suffix_afsel_l>]()
                }

                #[inline(always)]
                fn afsel_h(regs: &Iox) -> &Reg<Self::AfselHSpec> {
                    regs.[<sfr_afsel_crafsel $suffix_afsel_h>]()
                }

                #[inline(always)]
                fn gpio_out(regs: &Iox) -> &Reg<Self::GpioOutSpec> {
                    regs.[<sfr_gpioout_crgo $suffix>]()
                }

                #[inline(always)]
                fn gpio_oe(regs: &Iox) -> &Reg<Self::GpioOeSpec> {
                    regs.[<sfr_gpiooe_crgoe $suffix>]()
                }

                #[inline(always)]
                fn gpio_pu(regs: &Iox) -> &Reg<Self::GpioPuSpec> {
                    regs.[<sfr_gpiopu_crgpu $suffix>]()
                }

                #[inline(always)]
                fn gpio_in(regs: &Iox) -> &Reg<Self::GpioInSpec> {
                    regs.[<sfr_gpioin_srgi $suffix>]()
                }

                #[inline(always)]
                fn schm(regs: &Iox) -> &Reg<Self::SchmSpec> {
                    regs.[<sfr_cfg_schm_cr_cfg_schmsel $suffix>]()
                }

                #[inline(always)]
                fn slew(regs: &Iox) -> &Reg<Self::SlewSpec> {
                    regs.[<sfr_cfg_slew_cr_cfg_slewslow $suffix>]()
                }

                #[inline(always)]
                fn drvsel(regs: &Iox) -> &Reg<Self::DrvselSpec> {
                    regs.[<sfr_cfg_drvsel_cr_cfg_drvsel $suffix>]()
                }
            }
        }
    };
}

def_port!(PortA, 0, 0, 1);
def_port!(PortB, 1, 2, 3);
def_port!(PortC, 2, 4, 5);
def_port!(PortD, 3, 6, 7);
def_port!(PortE, 4, 8, 9);
