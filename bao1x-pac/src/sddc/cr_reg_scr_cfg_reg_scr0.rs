#[doc = "Register `CR_REG_SCR_CFG_REG_SCR0` reader"]
pub type R = crate::R<CrRegScrCfgRegScr0Spec>;
#[doc = "Register `CR_REG_SCR_CFG_REG_SCR0` writer"]
pub type W = crate::W<CrRegScrCfgRegScr0Spec>;
#[doc = "Field `cfg_reg_scr0` reader - cr_reg_scr read/write control register"]
pub type CfgRegScr0R = crate::FieldReader<u32>;
#[doc = "Field `cfg_reg_scr0` writer - cr_reg_scr read/write control register"]
pub type CfgRegScr0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - cr_reg_scr read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_scr0(&self) -> CfgRegScr0R {
        CfgRegScr0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - cr_reg_scr read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_scr0(&mut self) -> CfgRegScr0W<'_, CrRegScrCfgRegScr0Spec> {
        CfgRegScr0W::new(self, 0)
    }
}
#[doc = "See `sddc.sv#L138 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L138>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_scr_cfg_reg_scr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_scr_cfg_reg_scr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrRegScrCfgRegScr0Spec;
impl crate::RegisterSpec for CrRegScrCfgRegScr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_reg_scr_cfg_reg_scr0::R`](R) reader structure"]
impl crate::Readable for CrRegScrCfgRegScr0Spec {}
#[doc = "`write(|w| ..)` method takes [`cr_reg_scr_cfg_reg_scr0::W`](W) writer structure"]
impl crate::Writable for CrRegScrCfgRegScr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_REG_SCR_CFG_REG_SCR0 to value 0"]
impl crate::Resettable for CrRegScrCfgRegScr0Spec {}
