#[doc = "Register `CR_REG_SCR_CFG_REG_SCR1` reader"]
pub type R = crate::R<CrRegScrCfgRegScr1Spec>;
#[doc = "Register `CR_REG_SCR_CFG_REG_SCR1` writer"]
pub type W = crate::W<CrRegScrCfgRegScr1Spec>;
#[doc = "Field `cfg_reg_scr1` reader - cr_reg_scr read/write control register"]
pub type CfgRegScr1R = crate::FieldReader<u32>;
#[doc = "Field `cfg_reg_scr1` writer - cr_reg_scr read/write control register"]
pub type CfgRegScr1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - cr_reg_scr read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_scr1(&self) -> CfgRegScr1R {
        CfgRegScr1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - cr_reg_scr read/write control register"]
    #[inline(always)]
    pub fn cfg_reg_scr1(&mut self) -> CfgRegScr1W<'_, CrRegScrCfgRegScr1Spec> {
        CfgRegScr1W::new(self, 0)
    }
}
#[doc = "See `sddc.sv#L138 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/i fsub/rtl/sddc.sv#L138>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_reg_scr_cfg_reg_scr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_reg_scr_cfg_reg_scr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrRegScrCfgRegScr1Spec;
impl crate::RegisterSpec for CrRegScrCfgRegScr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr_reg_scr_cfg_reg_scr1::R`](R) reader structure"]
impl crate::Readable for CrRegScrCfgRegScr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr_reg_scr_cfg_reg_scr1::W`](W) writer structure"]
impl crate::Writable for CrRegScrCfgRegScr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR_REG_SCR_CFG_REG_SCR1 to value 0"]
impl crate::Resettable for CrRegScrCfgRegScr1Spec {}
