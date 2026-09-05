#[doc = "Register `SFR_CFG4` reader"]
pub type R = crate::R<SfrCfg4Spec>;
#[doc = "Register `SFR_CFG4` writer"]
pub type W = crate::W<SfrCfg4Spec>;
#[doc = "Field `sfr_cfg4` reader - sfr_cfg4 read/write control register"]
pub type SfrCfg4R = crate::FieldReader<u16>;
#[doc = "Field `sfr_cfg4` writer - sfr_cfg4 read/write control register"]
pub type SfrCfg4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - sfr_cfg4 read/write control register"]
    #[inline(always)]
    pub fn sfr_cfg4(&self) -> SfrCfg4R {
        SfrCfg4R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - sfr_cfg4 read/write control register"]
    #[inline(always)]
    pub fn sfr_cfg4(&mut self) -> SfrCfg4W<'_, SfrCfg4Spec> {
        SfrCfg4W::new(self, 0)
    }
}
#[doc = "See `dkpc.sv#L171 <https://github.com/baochip/baochip-1x/blob/main/rtl/modules/a o/rtl/dkpc.sv#L171>`__ (line numbers are approximate)\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr_cfg4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr_cfg4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfrCfg4Spec;
impl crate::RegisterSpec for SfrCfg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfr_cfg4::R`](R) reader structure"]
impl crate::Readable for SfrCfg4Spec {}
#[doc = "`write(|w| ..)` method takes [`sfr_cfg4::W`](W) writer structure"]
impl crate::Writable for SfrCfg4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFR_CFG4 to value 0"]
impl crate::Resettable for SfrCfg4Spec {}
